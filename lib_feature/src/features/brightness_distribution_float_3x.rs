use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::brightness_distribution::calc_brightness_distribution_float_x;
use crate::features::Feature;

/// See the calc_brightness_distribution_float_x
pub struct BrightnessDistribution3X(pub [f64; 3]);

impl Deref for BrightnessDistribution3X {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution3X {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_x(3, gesture);
        BrightnessDistribution3X([result[0], result[1], result[2]])
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
    use crate::features::{BrightnessDistribution3X, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,110,100,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,110,100,100,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,110,100,100,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,110,100,100,100,100,100,100,1").unwrap();
        let frame7 = Frame::from_str("100,100,100,100,100,100,100,120,100,1").unwrap();
        let frame8 = Frame::from_str("100,100,100,100,100,100,100,120,100,1").unwrap();
        let frame9 = Frame::from_str("100,100,100,100,100,100,100,120,100,1").unwrap();
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
        let feature = BrightnessDistribution3X::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[1.0 / 3.0, 5.0 / 3.0, 1.0]);
    }

    #[test]
    fn test_calculate_3_frames() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,110,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,110,100,100,100,100,100,100,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = BrightnessDistribution3X::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = BrightnessDistribution3X([0.1, 1.1, 2.1]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0.1,1.1,2.1"));
    }
}