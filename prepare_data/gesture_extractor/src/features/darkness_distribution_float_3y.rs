use std::ops::Deref;
use crate::features::Feature;
use crate::entities::Gesture;
use crate::features::darkness_distribution::calc_darkness_distribution_float_y;

pub struct DarknessDistribution3Y([f64; 3]);

impl Deref for DarknessDistribution3Y {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for DarknessDistribution3Y {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_darkness_distribution_float_y(3, gesture);
        DarknessDistribution3Y([result[0], result[1], result[2]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(f64::to_string).collect::<Vec<String>>().join(",")
    }
}