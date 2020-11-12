use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::{Feature, MeanValue};

pub struct StandardDeviation([f64; 9]);

impl Deref for StandardDeviation {
    type Target = [f64; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for StandardDeviation {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [f64; 9] = [0.0; 9];
        let average = MeanValue::calculate(gesture);

        for frame in gesture.frames.iter() {
            for i in 0..9 {
                let difference = (frame.pixel[i] as i32 - average.deref()[i]) as f64;
                result[i] += difference * difference;
            }
        }

        let result_len = gesture.frames.len() as f64;
        for i in 0..9 {
            result[i] = ((1.0 / result_len) * (result[i] as f64)).sqrt();
        }

        StandardDeviation(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}