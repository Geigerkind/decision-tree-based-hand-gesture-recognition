use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::Feature;

pub struct MeanValue([i32; 9]);

impl Deref for MeanValue {
    type Target = [i32; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MeanValue {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [i32; 9] = [0; 9];
        for frame in gesture.frames.iter() {
            for i in 0..9 {
                result[i] += frame.pixel[i] as i32;
            }
        }
        let len = gesture.frames.len() as f64;
        for i in 0..9 {
            result[i] = ((result[i] as f64) / len) as i32;
        }
        MeanValue(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i32::to_string).collect::<Vec<String>>().join(",")
    }
}