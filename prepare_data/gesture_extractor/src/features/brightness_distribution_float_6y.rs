use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::brightness_distribution::calc_brightness_distribution_float_y;
use crate::features::Feature;

pub struct BrightnessDistribution6Y([f64; 6]);

impl Deref for BrightnessDistribution6Y {
    type Target = [f64; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution6Y {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_y(6, gesture);
        BrightnessDistribution6Y([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}