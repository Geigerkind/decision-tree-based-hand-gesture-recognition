use crate::entities::Gesture;

pub use self::local_sum_of_slopes_x::LocalSumOfSlopeX;
pub use self::local_sum_of_slopes_y::LocalSumOfSlopeY;
pub use self::darkness_distribution_float_3x::DarknessDistribution3X;
pub use self::darkness_distribution_float_6x::DarknessDistribution6X;
pub use self::darkness_distribution_float_3y::DarknessDistribution3Y;
pub use self::darkness_distribution_float_6y::DarknessDistribution6Y;
pub use self::darkness_distribution_6xy::DarknessDistribution6XY;
pub use self::darkness_distribution_6xy_geom::DarknessDistribution6XYGeom;
pub use self::darkness_distribution_6xy_quadrant::DarknessDistribution6XYQuadrant;
pub use self::brightness_distribution_float_3x::BrightnessDistribution3X;
pub use self::brightness_distribution_float_6x::BrightnessDistribution6X;
pub use self::brightness_distribution_float_3y::BrightnessDistribution3Y;
pub use self::brightness_distribution_float_6y::BrightnessDistribution6Y;
pub use self::brightness_distribution_6xy::BrightnessDistribution6XY;
pub use self::brightness_distribution_6xy_geom::BrightnessDistribution6XYGeom;
pub use self::brightness_distribution_6xy_quadrant::BrightnessDistribution6XYQuadrant;
pub use self::feature_type::FeatureType;
pub use self::motion_history::MotionHistory;
pub use self::mean_value::MeanValue;
pub use self::minimum_value::MinimumValue;
pub use self::maximum_value::MaximumValue;
pub use self::standard_deviation::StandardDeviation;
pub use self::average_amplitude_change::AverageAmplitudeChange;
pub use self::direction_map_x::DirectionMapX;
pub use self::direction_map_y::DirectionMapY;

mod local_sum_of_slopes_x;
mod local_sum_of_slopes_y;
mod darkness_distribution;
mod darkness_distribution_float_3x;
mod darkness_distribution_float_6x;
mod darkness_distribution_float_3y;
mod darkness_distribution_float_6y;
mod darkness_distribution_6xy;
mod darkness_distribution_6xy_geom;
mod darkness_distribution_6xy_quadrant;
mod brightness_distribution;
mod brightness_distribution_float_3x;
mod brightness_distribution_float_6x;
mod brightness_distribution_float_3y;
mod brightness_distribution_float_6y;
mod brightness_distribution_6xy;
mod brightness_distribution_6xy_geom;
mod brightness_distribution_6xy_quadrant;
mod feature_type;
mod motion_history;
mod mean_value;
mod minimum_value;
mod maximum_value;
mod standard_deviation;
mod average_amplitude_change;
mod direction_map_x;
mod direction_map_y;

pub trait Feature {
    fn calculate(gesture: &Gesture) -> Self where Self: Sized;
    fn marshal(&self) -> String;
}
