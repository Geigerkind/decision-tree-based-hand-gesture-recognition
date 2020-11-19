use crate::entities::{Gesture, Frame};
use std::collections::VecDeque;

/// This is a reimplementation of the threshold function defined by Kubik.
/// See his thesis for a detailed explanation.
#[derive(Debug)]
pub struct GestureReader {
    threshold_low: f64,
    threshold_high: f64,
    // In frameBuffer.py its called "ema", this is my best guess
    trigger_mean: f64,
    alpha: f64,
    pixel_treshold: f64,
    margin: f64,
    gesture_buffer: Gesture,
    padding_counter: u8,
    record_gesture: bool,
    finished_recording: bool,
    pad_gesture: bool,
    pad_buffer: VecDeque<Frame>,
    is_initialized: bool
}

impl GestureReader {
    pub fn new(alpha: f64, pixel_treshold: f64, margin: f64, pad_gesture: bool) -> Self {
        assert!(margin >= 0.0 && margin <= 1.0);
        assert!(pixel_treshold >= 0.0 && pixel_treshold <= 1.0);
        GestureReader {
            threshold_low: 0.0,
            threshold_high: 0.0,
            trigger_mean: 0.0,
            alpha,
            pixel_treshold,
            margin,
            gesture_buffer: Gesture::default(),
            padding_counter: 3,
            record_gesture: false,
            finished_recording: false,
            pad_gesture,
            pad_buffer: VecDeque::with_capacity(3),
            is_initialized: false
        }
    }

    /// This function continuously updates its current frame mean
    /// If a frame is beyond the mean, the recording is started
    /// If it is beyond the mean again, the recording is stopped
    /// Additionally, we collect the 3 events before and after the recorded events
    /// This is configurable at the reader
    /// Note: This function is similar to the original in frameBuffer.py, but only at its threshold calculation
    pub fn feed_frame(&mut self, frame: Frame) -> Option<Gesture> {
        // I guess an initialization step
        if !self.is_initialized {
            self.trigger_mean = frame.mean();
            self.threshold_low = self.trigger_mean * (1.0 - self.margin);
            self.threshold_high = self.trigger_mean * (1.0 + self.margin);
            self.is_initialized = true;
            return None;
        }

        if self.record_gesture {
            self.gesture_buffer.add_frame(frame.clone());
        } else if self.finished_recording && self.pad_gesture && self.padding_counter > 0 {
            self.gesture_buffer.add_frame(frame.clone());
            self.padding_counter -= 1;
        }

        if self.finished_recording {
            if !self.pad_gesture || self.padding_counter == 0 {
                self.finished_recording = false;

                self.trigger_mean = frame.mean();
                self.threshold_low = self.trigger_mean * (1.0 - self.margin);
                self.threshold_high = self.trigger_mean * (1.0 + self.margin);

                // We require at least 6 frames!
                if (self.pad_gesture && self.gesture_buffer.frames.len() + self.pad_buffer.len() >= 6) || self.gesture_buffer.frames.len() >= 6 {
                    let mut result = Gesture::default();
                    std::mem::swap(&mut self.gesture_buffer, &mut result);
                    if self.pad_gesture {
                        let mut padded_result = Vec::with_capacity(self.gesture_buffer.frames.len() + self.pad_buffer.len());
                        padded_result.append(&mut self.pad_buffer.iter().cloned().collect());
                        padded_result.append(&mut result.frames.clone());
                        result.frames = padded_result;
                    }
                    return Some(result);
                } else {
                    // Dismiss otherwise, I guess
                    self.gesture_buffer = Gesture::default();
                    self.pad_buffer = VecDeque::with_capacity(3);
                    self.record_gesture = false;
                    return None;
                }
            }
        }

        // This apparently means that no event has happened, so we reset the padding counter
        // and adjust the thresholds
        if frame.mean() >= self.threshold_low && frame.mean() <= self.threshold_high {
            // Finish recording!
            if self.record_gesture {
                self.padding_counter = 3;
                self.record_gesture = false;
                self.finished_recording = true;
            }

            if !self.finished_recording && !self.record_gesture && self.pad_gesture {
                if self.pad_buffer.len() == 3 {
                    self.pad_buffer.pop_front();
                }
                self.pad_buffer.push_back(frame.clone());
            }

            return None;
        }

        // This apparently means that a gesture happened
        if frame.mean() < self.threshold_low || frame.mean() > self.threshold_high {
            if !self.finished_recording {
                if self.record_gesture && self.pad_gesture {
                    self.padding_counter = 3;
                } else if !self.record_gesture {
                    self.gesture_buffer.add_frame(frame.clone());
                    self.record_gesture = true;
                }
            }
        }

        if !self.finished_recording && !self.record_gesture {
            self.trigger_mean = self.trigger_mean * (1.0 - self.alpha) + frame.mean() * self.alpha;
            self.threshold_low = self.trigger_mean * (1.0 - self.margin);
            self.threshold_high = self.trigger_mean * (1.0 + self.margin);
        }

        if self.gesture_buffer.frames.len() == 100 {
            self.finished_recording = false;
            self.record_gesture = false;
            self.padding_counter = 3;

            self.trigger_mean = frame.mean();
            self.threshold_low = self.trigger_mean * (1.0 - self.margin);
            self.threshold_high = self.trigger_mean * (1.0 + self.margin);

            self.gesture_buffer = Gesture::default();
            return None;
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::entities::{Frame, GestureReader};
    use std::str::FromStr;

    #[test]
    fn test_parse_gesture() {
        // Arrange
        let frames = vec![
            Frame::from_str("802,809,805,817,773,804,814,777,757,0").unwrap(),
            Frame::from_str("803,809,805,816,774,805,813,777,757,0").unwrap(),
            Frame::from_str("803,809,805,816,773,805,814,777,756,0").unwrap(),
            Frame::from_str("803,809,806,816,773,804,813,776,757,1").unwrap(),
            Frame::from_str("801,808,805,815,770,805,812,773,757,1").unwrap(),
            Frame::from_str("798,806,801,810,769,801,802,771,752,1").unwrap(),
            Frame::from_str("653,786,797,684,734,796,674,722,748,1").unwrap(),
            Frame::from_str("787,676,723,795,609,718,790,626,623,1").unwrap(),
            Frame::from_str("804,809,751,817,769,734,814,770,593,1").unwrap(),
            Frame::from_str("803,811,803,817,776,803,814,780,758,1").unwrap(),
            Frame::from_str("804,810,806,817,774,806,814,778,759,1").unwrap(),
            Frame::from_str("803,809,805,817,774,805,814,776,757,0").unwrap(),
            Frame::from_str("804,809,805,817,773,806,814,777,758,0").unwrap(),
            Frame::from_str("803,809,805,817,773,805,814,776,757,0").unwrap(),
            Frame::from_str("803,809,805,817,773,805,814,776,757,0").unwrap(),
            Frame::from_str("803,809,805,817,773,805,814,776,757,0").unwrap(),
        ];

        // Act
        let mut gesture_reader = GestureReader::new(0.01, 0.01, 0.01, true);
        let mut result = Vec::new();
        for frame in frames {
            if let Some(gesture) = gesture_reader.feed_frame(frame) {
                result.push(gesture);
            }
        }

        // Assert
        assert_eq!(result.len(), 1);
    }
}