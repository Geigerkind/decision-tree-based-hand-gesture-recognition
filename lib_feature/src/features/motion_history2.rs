use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{AverageAmplitudeChange, Feature};

/// TODO
pub struct MotionHistory2(pub [f32; 9]);

impl Deref for MotionHistory2 {
    type Target = [f32; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MotionHistory2 {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let max_value: f32 = 1.0;
        let decay: f32 = max_value / (gesture.frames.len() as f32);
        let mut result: [f32; 9] = [0.0; 9];

        let average_amplitude_change = *AverageAmplitudeChange::calculate(gesture).deref() as i16;
        let mut last_frame = gesture.frames.get(0).unwrap();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                if (last_frame.pixel[i] - frame.pixel[i]).abs() < average_amplitude_change {
                    result[i] = (0.0f32).max(result[i] - decay);
                } else {
                    result[i] = max_value;
                }
            }
            last_frame = frame;
        }
        MotionHistory2(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f32::to_string).collect::<Vec<String>>().join(",")
    }
}