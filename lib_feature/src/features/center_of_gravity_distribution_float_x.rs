use std::ops::Deref;
use crate::Feature;
use lib_gesture::entities::Gesture;

pub struct CenterOfGravityDistributionFloatX(pub [f64; 6]);

impl Deref for CenterOfGravityDistributionFloatX {
    type Target = [f64; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for CenterOfGravityDistributionFloatX {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut center_of_gravities = Vec::with_capacity(gesture.frames.len());
        for frame in gesture.frames.iter() {
            let total_brightness = frame.pixel.iter().sum::<i16>();
            if total_brightness == 0 {
                center_of_gravities.push(0.0);
                continue;
            }

            let amount = frame.pixel[0] + frame.pixel[3] + frame.pixel[6] - frame.pixel[2] - frame.pixel[5] - frame.pixel[8];
            center_of_gravities.push((amount as f64) / (total_brightness as f64));
        }

        let merge_threshold = center_of_gravities.len() as f64 / 6.0;
        let mut values = Vec::new();
        let mut perma_result: [f64; 6] = [0.0; 6];
        let mut perma_result_index = 0;
        for i in 0..center_of_gravities.len() {
            values.push(center_of_gravities[i]);
            if ((i + 1) as f64) < merge_threshold * ((perma_result_index + 1) as f64) {
                continue;
            }

            perma_result[perma_result_index] = values.iter().sum::<f64>() / (values.len() as f64);
            perma_result_index += 1;
            values.clear();
        }

        CenterOfGravityDistributionFloatX(perma_result)
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
    use crate::features::{CenterOfGravityDistributionFloatX, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("10,0,5,0,0,0,0,0,0,1").unwrap();
        let frame2 = Frame::from_str("0,0,0,10,0,5,0,0,0,1").unwrap();
        let frame3 = Frame::from_str("0,0,0,0,0,0,10,0,5,1").unwrap();
        let frame4 = Frame::from_str("0,0,0,0,0,0,5,0,10,1").unwrap();
        let frame5 = Frame::from_str("0,0,0,5,0,10,0,0,0,1").unwrap();
        let frame6 = Frame::from_str("5,0,10,0,0,0,0,0,0,1").unwrap();
        let frame7 = Frame::from_str("10,0,5,0,0,0,0,0,0,1").unwrap();
        let frame8 = Frame::from_str("0,0,0,10,0,5,0,0,0,1").unwrap();
        let frame9 = Frame::from_str("0,0,0,0,0,0,10,0,5,1").unwrap();
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
        let feature = CenterOfGravityDistributionFloatX::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[1.0 / 3.0, 1.0 / 3.0, -1.0 / 3.0, -1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]);
    }

    #[test]
    fn test_calculate_6_frames() {
        // Arrange
        let frame1 = Frame::from_str("10,0,5,0,0,0,0,0,0,1").unwrap();
        let frame2 = Frame::from_str("0,0,0,10,0,5,0,0,0,1").unwrap();
        let frame3 = Frame::from_str("0,0,0,0,0,0,10,0,5,1").unwrap();
        let frame4 = Frame::from_str("0,0,0,0,0,0,5,0,10,1").unwrap();
        let frame5 = Frame::from_str("0,0,0,5,0,10,0,0,0,1").unwrap();
        let frame6 = Frame::from_str("5,0,10,0,0,0,0,0,0,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.frames.push(frame5);
        gesture.frames.push(frame6);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = CenterOfGravityDistributionFloatX::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0 / 3.0, -1.0 / 3.0, -1.0 / 3.0]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = CenterOfGravityDistributionFloatX([0.1, 1.1, 2.1, 0.1, 1.1, 2.1]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0.1,1.1,2.1,0.1,1.1,2.1"));
    }
}