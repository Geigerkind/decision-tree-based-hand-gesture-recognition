use lib_gesture::entities::Gesture;

use crate::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};
use std::path::Path;
use lib_gesture::tools::{parse_gestures_by_annotation, parse_gestures_by_threshold};

#[derive(Debug, Getters)]
pub struct DataSetEntry {
    data_set_name: DataSetName,
    covering_object: CoveringObject,
    camera_distance: CameraDistance,
    brightness_level: BrightnessLevel,
    additional_specification: Option<AdditionalSpecification>,
    gestures: Vec<Gesture>,
    #[getter(skip)]
    file_path: String,
    parsing_method: ParsingMethod
}

impl DataSetEntry {
    pub fn new(file_path: String, data_set_name: DataSetName, covering_object: CoveringObject, camera_distance: CameraDistance,
               brightness_level: BrightnessLevel, additional_specification: Option<AdditionalSpecification>, parsing_method: ParsingMethod) -> Self {
        let mut entry = DataSetEntry {
            data_set_name,
            covering_object,
            camera_distance,
            brightness_level,
            additional_specification,
            gestures: Vec::new(),
            file_path,
            parsing_method
        };
        entry.parse();
        entry
    }

    fn parse(&mut self) {
        if !Path::new(&self.file_path).exists() {
            panic!("DataSetEntry does not exist: {:?}", self);
        }

        match &self.parsing_method {
            ParsingMethod::ByAnnotation => {
                self.gestures = parse_gestures_by_annotation(&self.file_path).expect("File should have expected format!")
            }
            ParsingMethod::ByThreshold => {
                self.gestures = parse_gestures_by_threshold(&self.file_path).expect("File should have expected format!")
            }
        };
    }
}