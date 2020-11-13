use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{AverageAmplitudeChange, Feature};

pub struct MotionHistory(pub [i16; 9]);

impl Deref for MotionHistory {
    type Target = [i16; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MotionHistory {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [i16; 9] = [8192; 9];
        let average_amplitude_change = *AverageAmplitudeChange::calculate(gesture).deref() as i16;
        let mut last_frame = gesture.frames.get(0).unwrap();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                if (last_frame.pixel[i] - frame.pixel[i]).abs() < average_amplitude_change {
                    result[i] /= 2;
                } else {
                    result[i] = 8192;
                }
            }
            last_frame = frame;
        }
        MotionHistory(result)
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
    use crate::features::{Feature, MotionHistory};
    use crate::value_objects::GestureType;

    #[test]
    fn test_calculate() {
        // Arrange
        let frame1 = Frame::from_str("110,100,100,100,100,100,100,100,100,1").unwrap();
        let frame2 = Frame::from_str("100,120,100,100,100,100,100,100,100,1").unwrap();
        let frame3 = Frame::from_str("100,100,130,100,100,100,100,100,100,1").unwrap();
        let frame4 = Frame::from_str("100,100,100,140,100,100,100,100,100,1").unwrap();
        let frame5 = Frame::from_str("100,100,100,100,150,100,100,100,100,1").unwrap();
        let frame6 = Frame::from_str("100,100,100,100,100,160,100,100,100,1").unwrap();
        let frame7 = Frame::from_str("100,100,100,100,100,100,170,100,100,1").unwrap();
        let frame8 = Frame::from_str("100,100,100,100,100,100,100,180,100,1").unwrap();
        let frame9 = Frame::from_str("100,100,100,100,100,100,100,100,190,1").unwrap();
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
        let feature = MotionHistory::calculate(&gesture);

        // Assert
        assert_eq!(feature.deref(), &[32, 128, 256, 512, 1024, 2048, 4096, 8192, 8192]);
    }

    #[test]
    fn test_marshal() {
        // Arrange
        let feature = MotionHistory([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // Act
        let marshaled = feature.marshal();

        // Assert
        assert_eq!(marshaled, String::from("0,1,2,3,4,5,6,7,8"));
    }
}