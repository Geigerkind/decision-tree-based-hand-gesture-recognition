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