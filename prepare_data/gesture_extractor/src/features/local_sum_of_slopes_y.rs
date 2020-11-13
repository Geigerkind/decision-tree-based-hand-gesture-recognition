use crate::features::Feature;
use crate::entities::Gesture;
use std::ops::Deref;

pub struct LocalSumOfSlopeY(pub [i16; 3]);

impl Deref for LocalSumOfSlopeY {
    type Target = [i16; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for LocalSumOfSlopeY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        assert!(gesture.frames.len() > 1);
        let mut result = [0, 0, 0];
        let mut prev_frame = gesture.frames.first().unwrap();
        for frame in gesture.frames.iter().skip(1) {
            result[0] += prev_frame.pixel[0] - prev_frame.pixel[3];
            result[1] += prev_frame.pixel[1] - prev_frame.pixel[4];
            result[2] += prev_frame.pixel[2] - prev_frame.pixel[5];

            result[0] += prev_frame.pixel[3] - prev_frame.pixel[6];
            result[1] += prev_frame.pixel[4] - prev_frame.pixel[7];
            result[2] += prev_frame.pixel[5] - prev_frame.pixel[8];

            result[0] += prev_frame.pixel[6] - frame.pixel[0];
            result[1] += prev_frame.pixel[7] - frame.pixel[1];
            result[2] += prev_frame.pixel[8] - frame.pixel[2];
            prev_frame = frame;
        }
        result[0] += prev_frame.pixel[0] - prev_frame.pixel[3];
        result[1] += prev_frame.pixel[1] - prev_frame.pixel[4];
        result[2] += prev_frame.pixel[2] - prev_frame.pixel[5];

        result[0] += prev_frame.pixel[3] - prev_frame.pixel[6];
        result[1] += prev_frame.pixel[4] - prev_frame.pixel[7];
        result[2] += prev_frame.pixel[5] - prev_frame.pixel[8];
        LocalSumOfSlopeY(result)
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
    use crate::features::{LocalSumOfSlopeY, Feature};
    use crate::value_objects::GestureType;

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
        let feature = LocalSumOfSlopeY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[-10,-10,-10]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = LocalSumOfSlopeY([0, 1, 2]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2"));
    }
}