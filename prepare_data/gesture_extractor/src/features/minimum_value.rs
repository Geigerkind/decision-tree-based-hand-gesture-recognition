use std::cmp::min;
use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::Feature;

pub struct MinimumValue([i16; 9]);

impl Deref for MinimumValue {
    type Target = [i16; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for MinimumValue {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let mut result: [i16; 9] = gesture.frames.get(0).unwrap().pixel.clone();
        for frame in gesture.frames.iter().skip(1) {
            for i in 0..9 {
                result[i] = min(result[i], frame.pixel[i]);
            }
        }
        MinimumValue(result)
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(i16::to_string).collect::<Vec<String>>().join(",")
    }
}