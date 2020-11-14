use crate::entities::Frame;
use crate::value_objects::GestureType;

#[derive(Debug)]
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
}