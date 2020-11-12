use std::ops::Deref;
use crate::features::Feature;
use crate::features::darkness_distribution::{calc_darkness_distribution_float_xy_quadrant};
use crate::entities::Gesture;

pub struct DarknessDistribution6XYQuadrant([usize; 6]);

impl Deref for DarknessDistribution6XYQuadrant {
    type Target = [usize; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Feature for DarknessDistribution6XYQuadrant {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized {
        let result = calc_darkness_distribution_float_xy_quadrant(6, gesture);
        DarknessDistribution6XYQuadrant([result[0], result[1], result[2], result[3], result[4], result[5]])
    }

    fn marshal(&self) -> String {
        self.deref().iter().map(usize::to_string).collect::<Vec<String>>().join(",")
    }
}