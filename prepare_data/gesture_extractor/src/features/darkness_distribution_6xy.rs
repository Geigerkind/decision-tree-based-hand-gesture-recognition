use std::ops::Deref;
use crate::features::Feature;
use crate::features::darkness_distribution::{calc_darkness_distribution_float_xy};
use crate::entities::Gesture;

pub struct DarknessDistribution6XY([usize; 6]);

impl Deref for DarknessDistribution6XY {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for DarknessDistribution6XY {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_darkness_distribution_float_xy(6, gesture);
        DarknessDistribution6XY([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(usize::to_string).collect::<Vec<String>>().join(",")
    }
}