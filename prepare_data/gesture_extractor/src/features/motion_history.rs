use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::{Feature, AverageAmplitudeChange};
use std::cmp::{max, min};

pub struct MotionHistory([i16; 9]);

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