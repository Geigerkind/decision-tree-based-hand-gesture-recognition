use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::Feature;

pub struct AverageAmplitudeChange(pub i32);

impl Deref for AverageAmplitudeChange {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for AverageAmplitudeChange {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: i32 = 0;
        let mut last_frame = gesture.frames.get(0).unwrap();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                result += (last_frame.pixel[i] - frame.pixel[i]).abs() as i32;
            }
            last_frame = frame;
        }
        let len = ((gesture.frames.len() - 1) * 9) as f64;
        result = ((result as f64) / len) as i32;
        AverageAmplitudeChange(result)
    }

    fn marshal(&self) -> String {
        self.deref().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{AverageAmplitudeChange, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("100,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("110,110,110,110,110,110,110,110,110,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = AverageAmplitudeChange::calculate(&gesture);

        // Assert
        assert_eq!(*feature.deref(), 10);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = AverageAmplitudeChange(10);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("10"));
    }
}