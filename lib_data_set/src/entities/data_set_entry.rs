use lib_gesture::entities::Gesture;

use crate::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};
use std::path::Path;
use lib_gesture::tools::{parse_gestures_by_annotation, parse_gestures_by_threshold};

/// Defines a single entry in a data set, e.g. LRRL_Hand_10cm_highBrightness_fast.
#[derive(Debug, Getters, Clone)]
pub struct DataSetEntry {
    data_set_name: DataSetName,
    covering_object: CoveringObject,
    camera_distance: CameraDistance,
    brightness_level: BrightnessLevel,
    additional_specification: Option<AdditionalSpecification>,
    pub gestures: Vec<Gesture>,
    #[getter(skip)]
    file_path: String,
    parsing_method: ParsingMethod,
    scaling: Option<i32>,
    offset: Option<i16>
}

unsafe impl Sync for DataSetEntry {}
unsafe impl Send for DataSetEntry {}

impl DataSetEntry {
    /// Creates a new instance ob this structure and parses the data set specified in the path implicitly.
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
            parsing_method,
            scaling: None,
            offset: None
        };
        entry.parse();
        entry
    }

    /// Create a custom data set entry, e.g. for synthetic data
    pub fn custom(data_set_name: DataSetName, covering_object: CoveringObject, camera_distance: CameraDistance,
               brightness_level: BrightnessLevel, additional_specification: Option<AdditionalSpecification>, gestures: Vec<Gesture>) -> Self {
        DataSetEntry {
            data_set_name,
            covering_object,
            camera_distance,
            brightness_level,
            additional_specification,
            gestures,
            file_path: String::new(),
            parsing_method: ParsingMethod::ByAnnotation,
            scaling: None,
            offset: None
        }
    }

    pub fn take_percent_until(&mut self, percent: f32) {
        self.gestures = self.gestures[..(((self.gestures.len() as f32) * percent) as usize)].to_vec();
    }

    pub fn take_percent_from(&mut self, percent: f32) {
        self.gestures = self.gestures[(((self.gestures.len() as f32) * percent) as usize)..].to_vec();
    }

    pub fn scale_by(&mut self, factor: f32) {
        self.scaling = Some((factor * 100.0) as i32);
        for gesture in self.gestures.iter_mut() {
            for frame in gesture.frames.iter_mut() {
                for i in 0..9 {
                    frame.pixel[i] = 1023.min(((frame.pixel[i] as f32) * factor) as i16);
                }
            }
        }
    }

    pub fn add_offset(&mut self, offset: i16) {
        self.offset = Some(offset);
        for gesture in self.gestures.iter_mut() {
            for frame in gesture.frames.iter_mut() {
                for i in 0..9 {
                    frame.pixel[i] = 1023.min(frame.pixel[i] + offset);
                }
            }
        }
    }

    /// Checks if the file exists and if so parses it either ByAnnotation or ByThreshold.
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