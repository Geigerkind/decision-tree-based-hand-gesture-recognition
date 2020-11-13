use lib_gesture::entities::Gesture;

use crate::{CenterOfGravityDistributionFloatX, CenterOfGravityDistributionFloatY, CenterOfGravityDistributionX, CenterOfGravityDistributionY};
use crate::features::{AverageAmplitudeChange, BrightnessDistribution3X, BrightnessDistribution3Y, BrightnessDistribution6X, BrightnessDistribution6XY, BrightnessDistribution6XYGeom, BrightnessDistribution6XYQuadrant, BrightnessDistribution6Y, DarknessDistribution3X, DarknessDistribution3Y, DarknessDistribution6X, DarknessDistribution6XY, DarknessDistribution6XYGeom, DarknessDistribution6XYQuadrant, DarknessDistribution6Y, DirectionMapX, DirectionMapY, Feature, LocalSumOfSlopeX, LocalSumOfSlopeY, MaximumValue, MeanValue, MinimumValue, MotionHistory, StandardDeviation, SumOfSlopes};

#[derive(Debug, EnumIter, Display)]
pub enum FeatureType {
    LocalSumOfSlopesX,
    LocalSumOfSlopesY,
    DarknessDistribution3X,
    DarknessDistribution6X,
    DarknessDistribution3Y,
    DarknessDistribution6Y,
    DarknessDistribution6XY,
    DarknessDistribution6XYGeom,
    DarknessDistribution6XYQuadrant,
    BrightnessDistribution3X,
    BrightnessDistribution6X,
    BrightnessDistribution3Y,
    BrightnessDistribution6Y,
    BrightnessDistribution6XY,
    BrightnessDistribution6XYGeom,
    BrightnessDistribution6XYQuadrant,
    MotionHistory,
    MeanValue,
    MinimumValue,
    MaximumValue,
    StandardDeviation,
    AverageAmplitudeChange,
    DirectionMapX,
    DirectionMapY,
    SumOfSlopes,
    CenterOfGravityDistributionX,
    CenterOfGravityDistributionFloatX,
    CenterOfGravityDistributionY,
    CenterOfGravityDistributionFloatY,
}

impl FeatureType {
    pub fn to_feature(&self, gesture: &Gesture) -> Box<dyn Feature> {
        match self {
            FeatureType::LocalSumOfSlopesX => Box::new(LocalSumOfSlopeX::calculate(gesture)),
            FeatureType::LocalSumOfSlopesY => Box::new(LocalSumOfSlopeY::calculate(gesture)),
            FeatureType::DarknessDistribution3X => Box::new(DarknessDistribution3X::calculate(gesture)),
            FeatureType::DarknessDistribution6X => Box::new(DarknessDistribution6X::calculate(gesture)),
            FeatureType::DarknessDistribution3Y => Box::new(DarknessDistribution3Y::calculate(gesture)),
            FeatureType::DarknessDistribution6Y => Box::new(DarknessDistribution6Y::calculate(gesture)),
            FeatureType::DarknessDistribution6XY => Box::new(DarknessDistribution6XY::calculate(gesture)),
            FeatureType::DarknessDistribution6XYGeom => Box::new(DarknessDistribution6XYGeom::calculate(gesture)),
            FeatureType::DarknessDistribution6XYQuadrant => Box::new(DarknessDistribution6XYQuadrant::calculate(gesture)),
            FeatureType::BrightnessDistribution3X => Box::new(BrightnessDistribution3X::calculate(gesture)),
            FeatureType::BrightnessDistribution6X => Box::new(BrightnessDistribution6X::calculate(gesture)),
            FeatureType::BrightnessDistribution3Y => Box::new(BrightnessDistribution3Y::calculate(gesture)),
            FeatureType::BrightnessDistribution6Y => Box::new(BrightnessDistribution6Y::calculate(gesture)),
            FeatureType::BrightnessDistribution6XY => Box::new(BrightnessDistribution6XY::calculate(gesture)),
            FeatureType::BrightnessDistribution6XYGeom => Box::new(BrightnessDistribution6XYGeom::calculate(gesture)),
            FeatureType::BrightnessDistribution6XYQuadrant => Box::new(BrightnessDistribution6XYQuadrant::calculate(gesture)),
            FeatureType::MotionHistory => Box::new(MotionHistory::calculate(gesture)),
            FeatureType::MeanValue => Box::new(MeanValue::calculate(gesture)),
            FeatureType::MinimumValue => Box::new(MinimumValue::calculate(gesture)),
            FeatureType::MaximumValue => Box::new(MaximumValue::calculate(gesture)),
            FeatureType::StandardDeviation => Box::new(StandardDeviation::calculate(gesture)),
            FeatureType::AverageAmplitudeChange => Box::new(AverageAmplitudeChange::calculate(gesture)),
            FeatureType::DirectionMapX => Box::new(DirectionMapX::calculate(gesture)),
            FeatureType::DirectionMapY => Box::new(DirectionMapY::calculate(gesture)),
            FeatureType::SumOfSlopes => Box::new(SumOfSlopes::calculate(gesture)),
            FeatureType::CenterOfGravityDistributionX => Box::new(CenterOfGravityDistributionX::calculate(gesture)),
            FeatureType::CenterOfGravityDistributionFloatX => Box::new(CenterOfGravityDistributionFloatX::calculate(gesture)),
            FeatureType::CenterOfGravityDistributionY => Box::new(CenterOfGravityDistributionY::calculate(gesture)),
            FeatureType::CenterOfGravityDistributionFloatY => Box::new(CenterOfGravityDistributionFloatY::calculate(gesture)),
        }
    }
}