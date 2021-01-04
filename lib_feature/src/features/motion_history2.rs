use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{AverageAmplitudeChange, Feature};

/// TODO
pub struct MotionHistory2(pub [u8; 9]);

impl Deref for MotionHistory2 {
    type Target = [u8; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MotionHistory2 {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let max_value: u8 = 100;
        let decay: u8 = max_value / (gesture.frames.len() as u8);
        let mut result: [u8; 9] = [0; 9];

        let average_amplitude_change = *AverageAmplitudeChange::calculate(gesture).deref() as i16;
        let mut last_frame = gesture.frames.get(0).unwrap();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                if (last_frame.pixel[i] - frame.pixel[i]).abs() < average_amplitude_change {
                    if decay > result[i] {
                        result[i] = 0;
                    } else {
                        result[i] = (0).max(result[i] - decay);
                    }
                } else {
                    result[i] = max_value;
                }
            }
            last_frame = frame;
        }
        MotionHistory2(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(u8::to_string).collect::<Vec<String>>().join(",")
    }
}