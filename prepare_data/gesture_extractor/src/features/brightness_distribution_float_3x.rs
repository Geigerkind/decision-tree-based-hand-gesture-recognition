use std::ops::Deref;
use crate::features::Feature;
use crate::entities::Gesture;
use crate::features::brightness_distribution::calc_brightness_distribution_float_x;

pub struct BrightnessDistribution3X([f64; 3]);

impl Deref for BrightnessDistribution3X {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for BrightnessDistribution3X {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_brightness_distribution_float_x(3, gesture);
        BrightnessDistribution3X([result[0], result[1], result[2]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}