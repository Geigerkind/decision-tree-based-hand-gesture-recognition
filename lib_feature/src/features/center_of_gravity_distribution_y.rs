use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::Feature;

/// Calculates the average center of gravity for 6 time slots, i.e. if more than 6 samples are obtained, they
/// are squished into 6 values by applying the average of the sum.
/// y_g = (top_row - bottom_row) / total_of_all_pixel
pub struct CenterOfGravityDistributionY(pub [i8; 6]);

impl Deref for CenterOfGravityDistributionY {
    type Target = [i8; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for CenterOfGravityDistributionY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut center_of_gravities = Vec::with_capacity(gesture.frames.len());
        for frame in gesture.frames.iter() {
            let total_brightness = frame.pixel.iter().sum::<i16>();
            if total_brightness == 0 {
                center_of_gravities.push(0.0);
                continue;
            }
            let amount = frame.pixel[0] + frame.pixel[1] + frame.pixel[2] - frame.pixel[6] - frame.pixel[7] - frame.pixel[8];
            center_of_gravities.push((amount as f32) / (total_brightness as f32));
        }

        let merge_threshold = center_of_gravities.len() as f32 / 6.0;
        let mut values = Vec::new();
        let mut perma_result: [i8; 6] = [0; 6];
        let mut perma_result_index = 0;
        for i in 0..center_of_gravities.len() {
            values.push(center_of_gravities[i]);
            if ((i + 1) as f32) < merge_threshold * ((perma_result_index + 1) as f32) {
                continue;
            }
            perma_result[perma_result_index] = (values.iter().sum::<f32>() / (values.len() as f32)).round() as i8;
            perma_result_index += 1;
            values.clear();
        }

        CenterOfGravityDistributionY(perma_result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i8::to_string).collect::<Vec<String>>().join(",")
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::str::FromStr;

    use lib_gesture::entities::{Frame, Gesture};
    use crate::features::{CenterOfGravityDistributionY, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("100,0,5,0,0,0,0,0,0,1").unwrap();
        let frame2 = Frame::from_str("0,0,0,100,0,5,0,0,0,1").unwrap();
        let frame3 = Frame::from_str("0,0,0,0,0,0,100,0,5,1").unwrap();
        let frame4 = Frame::from_str("0,0,0,0,0,0,5,0,100,1").unwrap();
        let frame5 = Frame::from_str("0,0,0,5,0,100,0,0,0,1").unwrap();
        let frame6 = Frame::from_str("5,0,100,0,0,0,0,0,0,1").unwrap();
        let frame7 = Frame::from_str("100,0,5,0,0,0,0,0,0,1").unwrap();
        let frame8 = Frame::from_str("0,0,0,100,0,5,0,0,0,1").unwrap();
        let frame9 = Frame::from_str("0,0,0,0,0,0,100,0,5,1").unwrap();
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
        let feature = CenterOfGravityDistributionY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[1, -1, -1, 1, 1, -1]);
    }

    #[test]
    fn test_calculate_6_frames() {
        // Arrange
        let frame1 = Frame::from_str("100,0,5,0,0,0,0,0,0,1").unwrap();
        let frame2 = Frame::from_str("0,0,0,100,0,5,0,0,0,1").unwrap();
        let frame3 = Frame::from_str("0,0,0,0,0,0,100,0,5,1").unwrap();
        let frame4 = Frame::from_str("0,0,0,0,0,0,5,0,100,1").unwrap();
        let frame5 = Frame::from_str("0,0,0,5,0,100,0,0,0,1").unwrap();
        let frame6 = Frame::from_str("5,0,100,0,0,0,0,0,0,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.frames.push(frame4);
        gesture.frames.push(frame5);
        gesture.frames.push(frame6);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = CenterOfGravityDistributionY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[1, 0, -1, -1, 0, 1]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = CenterOfGravityDistributionY([0, 1, 2, 0, 1, 2]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,0,1,2"));
    }
}