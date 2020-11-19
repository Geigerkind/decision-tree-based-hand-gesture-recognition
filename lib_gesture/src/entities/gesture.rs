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
        if self.gesture_type == GestureType::NotGesture || self.gesture_type == GestureType::None {
            return Vec::new();
        }

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

    /// Infer garbage by inferring the rotations and splitting them at the half.
    /// Stich together the original gesture's first half and the second half from the rotation.
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
            _ => {
                println!("{:?}", self);
                unreachable!()
            }
        }
    }
}