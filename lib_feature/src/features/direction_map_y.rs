use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::Feature;

/// Calculates for 6 time frames the direction average from top to bottom.
/// First for all frames and rows the difference is summed from first to last frame.
/// Afterwards the average of each row is squished as average into one of the 6 time frames.
pub struct DirectionMapY(pub [i16; 6]);

impl Deref for DirectionMapY {
    type Target = [i16; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for DirectionMapY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        assert!(gesture.frames.len() > 2);
        let mut directions = Vec::with_capacity(gesture.frames.len() * 5);
        let mut prev_frame = gesture.frames.first().unwrap();
        for frame in gesture.frames.iter().skip(1) {
            directions.push([prev_frame.pixel[0] - prev_frame.pixel[3],
                prev_frame.pixel[1] - prev_frame.pixel[4],
                prev_frame.pixel[2] - prev_frame.pixel[5]]);

            directions.push([prev_frame.pixel[3] - prev_frame.pixel[6],
                prev_frame.pixel[4] - prev_frame.pixel[7],
                prev_frame.pixel[5] - prev_frame.pixel[8]]);

            directions.push([prev_frame.pixel[6] - frame.pixel[0],
                prev_frame.pixel[7] - frame.pixel[1],
                prev_frame.pixel[8] - frame.pixel[2]]);
            prev_frame = frame;
        }
        directions.push([prev_frame.pixel[0] - prev_frame.pixel[3],
            prev_frame.pixel[1] - prev_frame.pixel[4],
            prev_frame.pixel[2] - prev_frame.pixel[5]]);

        directions.push([prev_frame.pixel[3] - prev_frame.pixel[6],
            prev_frame.pixel[4] - prev_frame.pixel[7],
            prev_frame.pixel[5] - prev_frame.pixel[8]]);

        let threshold = (directions.len() as f32) / 6.0;
        let mut current_value = [0; 3];
        let mut perma_result = [0; 6];
        let mut index = 0;
        for i in 0..directions.len() {
            current_value[0] += directions[i][0];
            current_value[1] += directions[i][1];
            current_value[2] += directions[i][2];
            if (i as f32) < threshold * (index as f32) {
                continue;
            }
            perma_result[index] = (current_value[0] + current_value[1] + current_value[2]) / 3;
            index += 1;
            current_value = [0; 3];
        }
        DirectionMapY(perma_result)
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
    use crate::features::{DirectionMapY, Feature};
    use lib_gesture::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("100,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("110,110,110,110,110,110,110,110,110,1").unwrap();
        let frame3 = Frame::from_str("120,120,120,120,120,120,120,120,120,1").unwrap();
        let mut gesture = Gesture::default();
        gesture.frames.push(frame1);
        gesture.frames.push(frame2);
        gesture.frames.push(frame3);
        gesture.gesture_type = GestureType::LeftToRight;

        // Act
        let feature = DirectionMapY::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[0, -10, 0, 0, -10, 0]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = DirectionMapY([0, 1, 2, 3, 4, 5]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5"));
    }
}