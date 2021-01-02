use lib_data_set::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject};

#[derive(Debug, Hash, Copy, Clone, Getters, Eq, PartialEq)]
pub struct EvaluationEntryKey {
    pub covering_object: CoveringObject,
    pub camera_distance: CameraDistance,
    pub brightness_level: BrightnessLevel,
    pub additional_specification: Option<AdditionalSpecification>,
    pub offset: Option<i16>,
    pub scaling: Option<i32>
}

impl EvaluationEntryKey {
    pub fn new(covering_object: CoveringObject, camera_distance: CameraDistance, brightness_level: BrightnessLevel,
               additional_specification: Option<AdditionalSpecification>, offset: Option<i16>, scaling: Option<i32>) -> Self {
        EvaluationEntryKey {
            covering_object,
            camera_distance,
            brightness_level,
            additional_specification,
            offset,
            scaling
        }
    }
}