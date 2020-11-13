use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::brightness_distribution::calc_brightness_distribution_float_xy;
use crate::features::Feature;

pub struct BrightnessDistribution6XY(pub [usize; 6]);

impl Deref for BrightnessDistribution6XY {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution6XY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_xy(6, gesture);
        BrightnessDistribution6XY([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(usize::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{BrightnessDistribution6XY, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,110,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,110,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,110,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,100,100,110,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,100,100,100,110,100,100,100,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.frames.push(frame5);
        gesture.frames.push(frame6);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = BrightnessDistribution6XY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = BrightnessDistribution6XY([0, 1, 2, 3, 4, 5]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5"));
    }
}