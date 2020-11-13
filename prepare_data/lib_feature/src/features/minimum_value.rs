use std::cmp::min;
use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::Feature;

pub struct MinimumValue(pub [i16; 9]);

impl Deref for MinimumValue {
    type Target = [i16; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MinimumValue {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [i16; 9] = gesture.frames.get(0).unwrap().pixel.clone();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                result[i] = min(result[i], frame.pixel[i]);
            }
        }
        MinimumValue(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i16::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use crate::entities::{Frame, Gesture};
    use crate::features::{Feature, MinimumValue};
    use crate::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("10,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,20,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,30,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,40,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,100,100,50,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,100,100,100,60,100,100,100,1").unwrap();
        let frame7 = Frame::from_str("100,100,100,100,100,100,70,100,100,1").unwrap();
        let frame8 = Frame::from_str("100,100,100,100,100,100,100,80,100,1").unwrap();
        let frame9 = Frame::from_str("100,100,100,100,100,100,100,100,90,1").unwrap();
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
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = MinimumValue::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = MinimumValue([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5,6,7,8"));
    }
}