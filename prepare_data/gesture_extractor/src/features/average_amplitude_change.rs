use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::Feature;

pub struct AverageAmplitudeChange(i32);

impl Deref for AverageAmplitudeChange {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for AverageAmplitudeChange {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: i32 = 0;
        let mut last_frame = gesture.frames.get(0).unwrap();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                result += (last_frame.pixel[i] - frame.pixel[i]).abs() as i32;
            }
            last_frame = frame;
        }
        let len = (gesture.frames.len() * 9) as f64;
        result = ((result as f64) / len) as i32;
        AverageAmplitudeChange(result)
    }

    fn marshal(&self) -> String {
        self.deref().to_string()
    }
}