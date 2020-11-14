use lib_data_set::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject};

#[derive(Debug, Hash, Copy, Clone, Getters, Eq, PartialEq)]
pub struct EvaluationEntryKey {
    covering_object: CoveringObject,
    camera_distance: CameraDistance,
    brightness_level: BrightnessLevel,
    additional_specification: Option<AdditionalSpecification>,
}

impl EvaluationEntryKey {
    pub fn new(covering_object: CoveringObject, camera_distance: CameraDistance, brightness_level: BrightnessLevel,
               additional_specification: Option<AdditionalSpecification>) -> Self {
        EvaluationEntryKey {
            covering_object,
            camera_distance,
            brightness_level,
            additional_specification
        }
    }
}