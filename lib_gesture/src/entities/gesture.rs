use crate::entities::Frame;
use crate::value_objects::GestureType;

#[derive(Debug, Clone)]
pub struct Gesture {
    pub frames: Vec<Frame>,
    pub gesture_type: GestureType,
}

impl Default for Gesture {
    fn default() -> Self {
        Gesture {
            frames: Vec::with_capacity(20),
            gesture_type: GestureType::None,
        }
    }
}

impl Gesture {
    pub fn add_frame(&mut self, frame: Frame) {
        if self.gesture_type == GestureType::None {
            self.gesture_type = frame.gesture_type;
        }
        self.frames.push(frame);
    }

    // Dont flame me for this implementation
    // its 19:43, and I've been coding already for almost 13 hours
    pub fn infer_rotations(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let left_to_right = self.rotate_to_left_to_right();

        if self.gesture_type != GestureType::LeftToRight {
            result.push(left_to_right);
        }

        if self.gesture_type != GestureType::TopToBottom {
            let mut gesture = Gesture::default();
            gesture.gesture_type = GestureType::TopToBottom;
            gesture.frames = self.frames.iter().map(|frame| {
                let mut new_frame = Frame::default();
                new_frame.pixel[0] = frame.pixel[6];
                new_frame.pixel[1] = frame.pixel[3];
                new_frame.pixel[2] = frame.pixel[0];
                new_frame.pixel[3] = frame.pixel[7];
                new_frame.pixel[4] = frame.pixel[4];
                new_frame.pixel[5] = frame.pixel[1];
                new_frame.pixel[6] = frame.pixel[8];
                new_frame.pixel[7] = frame.pixel[5];
                new_frame.pixel[8] = frame.pixel[2];
                new_frame
            }).collect();
            result.push(gesture);
        }

        if self.gesture_type != GestureType::RightToLeft {
            let mut gesture = Gesture::default();
            gesture.gesture_type = GestureType::RightToLeft;
            gesture.frames = self.frames.iter().map(|frame| {
                let mut new_frame = frame.clone();
                new_frame.pixel[0] = frame.pixel[2];
                new_frame.pixel[3] = frame.pixel[5];
                new_frame.pixel[6] = frame.pixel[8];
                new_frame.pixel[2] = frame.pixel[0];
                new_frame.pixel[5] = frame.pixel[3];
                new_frame.pixel[8] = frame.pixel[6];
                new_frame
            }).collect();
            result.push(gesture);
        }

        if self.gesture_type != GestureType::BottomToTop {
            let mut gesture = Gesture::default();
            gesture.gesture_type = GestureType::BottomToTop;
            gesture.frames = self.frames.iter().map(|frame| {
                let mut new_frame = Frame::default();
                new_frame.pixel[0] = frame.pixel[6];
                new_frame.pixel[1] = frame.pixel[3];
                new_frame.pixel[2] = frame.pixel[0];
                new_frame.pixel[3] = frame.pixel[7];
                new_frame.pixel[4] = frame.pixel[4];
                new_frame.pixel[5] = frame.pixel[1];
                new_frame.pixel[6] = frame.pixel[8];
                new_frame.pixel[7] = frame.pixel[5];
                new_frame.pixel[8] = frame.pixel[2];
                new_frame
            }).collect();
            result.push(gesture);
        }

        result
    }

    pub fn infer_garbage(&self) -> Vec<Self> {
        let splitting_point = self.frames.len() / 2;
        self.infer_rotations().into_iter()
            .map(|mut gesture| {
                gesture.gesture_type = GestureType::NotGesture;
                for i in splitting_point..self.frames.len() {
                    gesture.frames[i] = self.frames[i].clone();
                }
                gesture
            }).collect()
    }

    fn rotate_to_left_to_right(&self) -> Self {
        match self.gesture_type {
            GestureType::LeftToRight => self.clone(),
            GestureType::RightToLeft => {
                let mut gesture = Gesture::default();
                gesture.gesture_type = GestureType::LeftToRight;
                gesture.frames = self.frames.iter().map(|frame| {
                    let mut new_frame = frame.clone();
                    new_frame.pixel[0] = frame.pixel[2];
                    new_frame.pixel[3] = frame.pixel[5];
                    new_frame.pixel[6] = frame.pixel[8];
                    new_frame.pixel[2] = frame.pixel[0];
                    new_frame.pixel[5] = frame.pixel[3];
                    new_frame.pixel[8] = frame.pixel[6];
                    new_frame
                }).collect();
                gesture
            }
            GestureType::TopToBottom => {
                let mut gesture = Gesture::default();
                gesture.gesture_type = GestureType::LeftToRight;
                gesture.frames = self.frames.iter().map(|frame| {
                    let mut new_frame = Frame::default();
                    new_frame.pixel[0] = frame.pixel[2];
                    new_frame.pixel[1] = frame.pixel[5];
                    new_frame.pixel[2] = frame.pixel[8];
                    new_frame.pixel[3] = frame.pixel[1];
                    new_frame.pixel[4] = frame.pixel[4];
                    new_frame.pixel[5] = frame.pixel[7];
                    new_frame.pixel[6] = frame.pixel[0];
                    new_frame.pixel[7] = frame.pixel[3];
                    new_frame.pixel[8] = frame.pixel[6];
                    new_frame
                }).collect();
                gesture
            }
            GestureType::BottomToTop => {
                let mut gesture = Gesture::default();
                gesture.gesture_type = GestureType::LeftToRight;
                gesture.frames = self.frames.iter().map(|frame| {
                    let mut new_frame = Frame::default();
                    new_frame.pixel[0] = frame.pixel[6];
                    new_frame.pixel[1] = frame.pixel[3];
                    new_frame.pixel[2] = frame.pixel[0];
                    new_frame.pixel[3] = frame.pixel[7];
                    new_frame.pixel[4] = frame.pixel[4];
                    new_frame.pixel[5] = frame.pixel[1];
                    new_frame.pixel[6] = frame.pixel[8];
                    new_frame.pixel[7] = frame.pixel[5];
                    new_frame.pixel[8] = frame.pixel[2];
                    new_frame
                }).collect();
                gesture
            }
            _ => unreachable!()
        }
    }
}