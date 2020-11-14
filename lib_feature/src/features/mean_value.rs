use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::Feature;

pub struct MeanValue(pub [i32; 9]);

impl Deref for MeanValue {
    type Target = [i32; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MeanValue {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [i32; 9] = [0; 9];
        for frame in gesture.frames.iter() {
            for i in 0..9 {
                result[i] += frame.pixel[i] as i32;
            }
        }
        let len = gesture.frames.len() as f64;
        for i in 0..9 {
            result[i] = ((result[i] as f64) / len) as i32;
        }
        MeanValue(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i32::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{Feature, MeanValue};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,120,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,130,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,140,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,100,100,150,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,100,100,100,160,100,100,100,1").unwrap();
        let frame7 = Frame::from_str("100,100,100,100,100,100,170,100,100,1").unwrap();
        let frame8 = Frame::from_str("100,100,100,100,100,100,100,180,100,1").unwrap();
        let frame9 = Frame::from_str("100,100,100,100,100,100,100,100,190,1").unwrap();
        let frame10 = Frame::from_str("100,100,100,100,100,100,100,100,100,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.frames.push(frame5);
        gesture.frames.push(frame6);
        gesture.frames.push(frame7);
        gesture.frames.push(frame8);
        gesture.frames.push(frame9);
        gesture.frames.push(frame10);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = MeanValue::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[101, 102, 103, 104, 105, 106, 107, 108, 109]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = MeanValue([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5,6,7,8"));
    }
}