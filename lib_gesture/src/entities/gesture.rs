use crate::entities::Frame;
use crate::value_objects::GestureType;

/// A gesture (candidate) is defined as a sequence of frames and is a assigned a gesture type.
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
    /// Pushes a frame and if the gesture (candidates)'s gesture type is None, assign to to the parsed one.
    pub fn add_frame(&mut self, frame: Frame) {
        if self.gesture_type == GestureType::None {
            self.gesture_type = frame.gesture_type;
        }
        self.frames.push(frame);
    }

    /// Infer rotations of the gesture by rotating it first to the LeftToRight and then rotate it 3 times.
    // Dont flame me for this implementation
    // its 19:43, and I've been coding already for almost 13 hours
    pub fn infer_rotations(&self) -> Vec<Self> {
        if self.gesture_type == GestureType::None {
            return Vec::new();
        }

        let mut result = Vec::new();
        let left_to_right = self.rotate_to_left_to_right();

        if self.gesture_type != GestureType::LeftToRight && self.gesture_type != GestureType::NotGesture {
            result.push(left_to_right);
        }

        if self.gesture_type != GestureType::TopToBottom {
            let mut gesture = Gesture::default();
            gesture.gesture_type = if self.gesture_type == GestureType::NotGesture {
                GestureType::NotGesture
            } else {
                GestureType::TopToBottom
            };
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
            gesture.gesture_type = if self.gesture_type == GestureType::NotGesture {
                GestureType::NotGesture
            } else {
                GestureType::RightToLeft
            };
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
            gesture.gesture_type = if self.gesture_type == GestureType::NotGesture {
                GestureType::NotGesture
            } else {
                GestureType::BottomToTop
            };
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

    /// Infer garbage by inferring the rotations and splitting them at the half.
    /// Stitch together the original gesture's first half and the second half from the rotation.
    pub fn infer_garbage(&self) -> Vec<Self> {
        if self.gesture_type == GestureType::NotGesture || self.gesture_type == GestureType::None {
            return Vec::new();
        }

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

    /// We shift LR RL 3 times to the top (and put pixel that went out at the top back to the bottom)
    /// Analogously we shift TB BT by doing the same to the right
    pub fn infer_shifting(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(2);
        match self.gesture_type {
            GestureType::LeftToRight | GestureType::RightToLeft => {
                for i in 1..3 {
                    let mut new_gesture = self.clone();
                    for (index, frame) in new_gesture.frames.iter_mut().enumerate() {
                        for j in 0..9 {
                            frame.pixel[((j as i8) - (i as i8) * 3).rem_euclid(9) as usize] = self.frames[index].pixel[j];
                        }
                    }
                    result.push(new_gesture);
                }
            }
            GestureType::TopToBottom | GestureType::BottomToTop => {
                for i in 1..3 {
                    let mut new_gesture = self.clone();
                    for (index, frame) in new_gesture.frames.iter_mut().enumerate() {
                        for j in 0..9 {
                            let mut shifted_index = j;
                            for _ in 0..i {
                                shifted_index = match shifted_index {
                                    0 | 1 | 3 | 4 | 6 | 7 => shifted_index + 1,
                                    2 | 5 | 8 => shifted_index - 2,
                                    _ => unreachable!()
                                };
                            }
                            frame.pixel[shifted_index] = self.frames[index].pixel[j];
                        }
                    }
                    result.push(new_gesture);
                }
            }
            _ => {}
        };
        result
    }

    /// Rotates all border pixel once clock wise and once anti clock wise
    /// The gesture type remains the same, for now this is added to make
    /// the algorithm more robust to non perfect gesture
    pub fn infer_diagonal(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(2);
        match self.gesture_type {
            GestureType::LeftToRight | GestureType::RightToLeft
            | GestureType::TopToBottom | GestureType::BottomToTop => {
                let mut left_gesture = self.clone();
                left_gesture.frames = left_gesture.frames.into_iter().map(|frame| rotate_frame_left(&frame)).collect();
                result.push(left_gesture);
                let mut right_gesture = self.clone();
                right_gesture.frames = right_gesture.frames.into_iter().map(|frame| rotate_frame_right(&frame)).collect();
                result.push(right_gesture);
            }
            _ => {}
        };

        result
    }

    /// Helper function to rotate a gesture into left to right position.
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
            GestureType::NotGesture => self.clone(),
            _ => {
                println!("{:?}", self);
                unreachable!()
            }
        }
    }

    /// Helfer function to simply print each frame
    pub fn print(&self) {
        for frame in self.frames.iter() {
            frame.print();
        }
    }
}

fn rotate_frame_left(frame: &Frame) -> Frame {
    let mut new_frame = frame.clone();
    for i in 0..9 {
        let shifted_index = match i {
            0 | 3 => i + 3,
            1 | 2 => i - 1,
            5 | 8 => i - 3,
            6 | 7 => i + 1,
            _ => i
        };
        new_frame.pixel[shifted_index] = frame.pixel[i];
    }
    new_frame
}

fn rotate_frame_right(frame: &Frame) -> Frame {
    let mut new_frame = frame.clone();
    for i in 0..9 {
        let shifted_index = match i {
            0 | 1 => i + 1,
            2 | 5 => i + 3,
            3 | 6 => i - 3,
            7 | 8 => i - 1,
            _ => i
        };
        new_frame.pixel[shifted_index] = frame.pixel[i];
    }
    new_frame
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::entities::{Frame, Gesture};
    use crate::value_objects::GestureType;

    #[test]
    fn test_infer_shifting_lrrl() {
        // Arrange
        let mut gesture = Gesture::default();
        gesture.gesture_type = GestureType::LeftToRight;
        gesture.frames.push(Frame::from_str("100,100,100,90,90,90,80,80,80,1").unwrap());

        // Act
        let shifting = gesture.infer_shifting();

        // Assert
        assert_eq!(shifting.len(), 2);
        let shift1 = shifting.get(0).unwrap();
        assert_eq!(shift1.frames[0].pixel[0], 90);
        assert_eq!(shift1.frames[0].pixel[1], 90);
        assert_eq!(shift1.frames[0].pixel[2], 90);
        assert_eq!(shift1.frames[0].pixel[3], 80);
        assert_eq!(shift1.frames[0].pixel[4], 80);
        assert_eq!(shift1.frames[0].pixel[5], 80);
        assert_eq!(shift1.frames[0].pixel[6], 100);
        assert_eq!(shift1.frames[0].pixel[7], 100);
        assert_eq!(shift1.frames[0].pixel[8], 100);
        let shift2 = shifting.get(1).unwrap();
        assert_eq!(shift2.frames[0].pixel[0], 80);
        assert_eq!(shift2.frames[0].pixel[1], 80);
        assert_eq!(shift2.frames[0].pixel[2], 80);
        assert_eq!(shift2.frames[0].pixel[3], 100);
        assert_eq!(shift2.frames[0].pixel[4], 100);
        assert_eq!(shift2.frames[0].pixel[5], 100);
        assert_eq!(shift2.frames[0].pixel[6], 90);
        assert_eq!(shift2.frames[0].pixel[7], 90);
        assert_eq!(shift2.frames[0].pixel[8], 90);
    }

    #[test]
    fn test_infer_shifting_tbbt() {
        // Arrange
        let mut gesture = Gesture::default();
        gesture.gesture_type = GestureType::TopToBottom;
        gesture.frames.push(Frame::from_str("100,90,80,100,90,80,100,90,80,3").unwrap());

        // Act
        let shifting = gesture.infer_shifting();

        // Assert
        assert_eq!(shifting.len(), 2);
        let shift1 = shifting.get(0).unwrap();
        assert_eq!(shift1.frames[0].pixel[0], 80);
        assert_eq!(shift1.frames[0].pixel[1], 100);
        assert_eq!(shift1.frames[0].pixel[2], 90);
        assert_eq!(shift1.frames[0].pixel[3], 80);
        assert_eq!(shift1.frames[0].pixel[4], 100);
        assert_eq!(shift1.frames[0].pixel[5], 90);
        assert_eq!(shift1.frames[0].pixel[6], 80);
        assert_eq!(shift1.frames[0].pixel[7], 100);
        assert_eq!(shift1.frames[0].pixel[8], 90);
        let shift2 = shifting.get(1).unwrap();
        assert_eq!(shift2.frames[0].pixel[0], 90);
        assert_eq!(shift2.frames[0].pixel[1], 80);
        assert_eq!(shift2.frames[0].pixel[2], 100);
        assert_eq!(shift2.frames[0].pixel[3], 90);
        assert_eq!(shift2.frames[0].pixel[4], 80);
        assert_eq!(shift2.frames[0].pixel[5], 100);
        assert_eq!(shift2.frames[0].pixel[6], 90);
        assert_eq!(shift2.frames[0].pixel[7], 80);
        assert_eq!(shift2.frames[0].pixel[8], 100);
    }

    #[test]
    fn test_infer_diagonal() {
        // Arrange
        let mut gesture = Gesture::default();
        gesture.gesture_type = GestureType::LeftToRight;
        gesture.frames.push(Frame::from_str("100,100,100,90,90,90,80,80,80,1").unwrap());

        // Act
        let diagonal = gesture.infer_diagonal();

        // Assert
        assert_eq!(diagonal.len(), 2);
        let diag1 = diagonal.get(0).unwrap();
        assert_eq!(diag1.frames[0].pixel[0], 100);
        assert_eq!(diag1.frames[0].pixel[1], 100);
        assert_eq!(diag1.frames[0].pixel[2], 90);
        assert_eq!(diag1.frames[0].pixel[3], 100);
        assert_eq!(diag1.frames[0].pixel[4], 90);
        assert_eq!(diag1.frames[0].pixel[5], 80);
        assert_eq!(diag1.frames[0].pixel[6], 90);
        assert_eq!(diag1.frames[0].pixel[7], 80);
        assert_eq!(diag1.frames[0].pixel[8], 80);
        let diag2 = diagonal.get(1).unwrap();
        assert_eq!(diag2.frames[0].pixel[0], 90);
        assert_eq!(diag2.frames[0].pixel[1], 100);
        assert_eq!(diag2.frames[0].pixel[2], 100);
        assert_eq!(diag2.frames[0].pixel[3], 80);
        assert_eq!(diag2.frames[0].pixel[4], 90);
        assert_eq!(diag2.frames[0].pixel[5], 100);
        assert_eq!(diag2.frames[0].pixel[6], 80);
        assert_eq!(diag2.frames[0].pixel[7], 80);
        assert_eq!(diag2.frames[0].pixel[8], 90);
    }
}