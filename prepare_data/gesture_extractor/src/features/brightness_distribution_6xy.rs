use std::ops::Deref;
use crate::features::Feature;
use crate::entities::Gesture;
use crate::features::brightness_distribution::{calc_brightness_distribution_float_xy};

pub struct BrightnessDistribution6XY([usize; 6]);

impl Deref for BrightnessDistribution6XY {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution6XY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_xy(6, gesture);
        BrightnessDistribution6XY([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(usize::to_string).collect::<Vec<String>>().join(",")
    }
}