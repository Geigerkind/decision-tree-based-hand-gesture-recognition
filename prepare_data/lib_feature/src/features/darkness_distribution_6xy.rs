use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::darkness_distribution::calc_darkness_distribution_float_xy;
use crate::features::Feature;

pub struct DarknessDistribution6XY(pub [usize; 6]);

impl Deref for DarknessDistribution6XY {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for DarknessDistribution6XY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_darkness_distribution_float_xy(6, gesture);
        DarknessDistribution6XY([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(usize::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use crate::entities::{Frame, Gesture};
    use crate::features::{DarknessDistribution6XY, Feature};
    use crate::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("90,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,90,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,90,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,90,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,100,100,90,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,100,100,100,90,100,100,100,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.frames.push(frame5);
        gesture.frames.push(frame6);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = DarknessDistribution6XY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = DarknessDistribution6XY([0, 1, 2, 3, 4, 5]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5"));
    }
}