use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::Feature;

pub struct SumOfSlopes(pub [i16; 9]);

impl Deref for SumOfSlopes {
    type Target = [i16; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for SumOfSlopes {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        assert!(gesture.frames.len() > 1);
        let mut last_frame = gesture.frames.get(0).unwrap();
        let mut result: [i16; 9] = [0; 9];
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                result[i] += last_frame.pixel[i] - frame.pixel[i];
            }
            last_frame = frame;
        }
        SumOfSlopes(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i16::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{Feature, SumOfSlopes};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,120,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,130,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,140,100,100,100,100,100,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = SumOfSlopes::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[10, 0, 0, -40, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = SumOfSlopes([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5,6,7,8"));
    }
}