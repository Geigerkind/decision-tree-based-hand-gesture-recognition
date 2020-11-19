use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::entities::DataSetEntry;
use crate::value_objects::{BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};

lazy_static! {
    /// Automatically parses the trainingVenzke data set, once this constant is imported.
    pub static ref VENZKE_TRAINING: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let path_training: String = format!("{}/data/trainingVenzke", std::env::var("DATA_PATH").unwrap());
        let mut result = HashMap::new();
        for parsing_method in ParsingMethod::iter() {
            result.insert(parsing_method, vec![
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_Garbage_25cm_190117_Annotated.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Unknown, CameraDistance::CM25, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_Garbage_181128_Annotated.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Unknown, CameraDistance::Unknown, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_LRRL_Arm_25cm_Annotated_190117.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Arm, CameraDistance::CM25, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_LRRL_Finger_3cm_Annotated_181128.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Finger, CameraDistance::CM3, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_LRRL_Hand_5cm_Annotated_181128.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_TBBT_Arm_25cm_190117_Annotated.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Arm, CameraDistance::CM25, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_TBBT_Finger_3cm_Annotated_181128.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Finger, CameraDistance::CM3, BrightnessLevel::Unknown, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_training, "Compound_TBBT_Hand_5cm_Annotated_181128.csv"), DataSetName::VenzkeTraining,
                    CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Unknown, None, parsing_method),
            ]);
        }
        result
    };
}