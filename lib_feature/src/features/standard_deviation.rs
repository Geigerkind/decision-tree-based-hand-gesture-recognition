use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{Feature, MeanValue};

/// Calculates the standard deviation of each pixel.
pub struct StandardDeviation(pub [f64; 9]);

impl Deref for StandardDeviation {
    type Target = [f64; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for StandardDeviation {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [f64; 9] = [0.0; 9];
        let average = MeanValue::calculate(gesture);

        for frame in gesture.frames.iter() {
            for i in 0..9 {
                let difference = (frame.pixel[i] as i32 - average.deref()[i]) as f64;
                result[i] += difference * difference;
            }
        }

        let result_len = gesture.frames.len() as f64;
        for i in 0..9 {
            result[i] = ((1.0 / result_len) * (result[i] as f64)).sqrt();
        }

        StandardDeviation(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{Feature, StandardDeviation};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("100,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("92,92,92,92,92,92,92,92,92,1").unwrap();
        let frame3 = Frame::from_str("108,108,108,108,108,108,108,108,108,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = StandardDeviation::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[6.531972647421808; 9]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = StandardDeviation([0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0.1,1.1,2.1,3.1,4.1,5.1,6.1,7.1,8.1"));
    }
}