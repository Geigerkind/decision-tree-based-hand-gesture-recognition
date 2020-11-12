use std::ops::Deref;

use crate::entities::Gesture;
use crate::features::brightness_distribution::calc_brightness_distribution_float_y;
use crate::features::Feature;

pub struct BrightnessDistribution3Y([f64; 3]);

impl Deref for BrightnessDistribution3Y {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution3Y {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_y(3, gesture);
        BrightnessDistribution3Y([result[0], result[1], result[2]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}