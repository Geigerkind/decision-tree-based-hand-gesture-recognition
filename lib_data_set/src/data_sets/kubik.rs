use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::entities::DataSetEntry;
use crate::value_objects::{BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};

const PATH_TEST: &str = "data/testKubik";
const PATH_TRAINING: &str = "data/trainingKubik";

lazy_static! {
    // TODO: Do we consider the CameraType?
    pub static ref KUBIK_TEST: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let mut result = HashMap::new();
        for parsing_method in ParsingMethod::iter() {
            result.insert(parsing_method, vec![
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceil_3cm-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::CM3, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceil_d1-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D1, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceil_d2-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D2, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceil_d3-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D3, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceildim_3cm-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::CM3, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceildim_d1-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D1, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceildim_d2-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D2, BrightnessLevel::Ceiling, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TEST, "test_fac_ceildim_d3-annotated.csv"), DataSetName::KubikTest,
                    CoveringObject::Unknown, CameraDistance::D3, BrightnessLevel::Ceiling, None, parsing_method),
            ]);
        }
        result
    };
}

lazy_static! {
    // TODO: Do we consider the CameraType?
    pub static ref KUBIK_TRAINING: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let mut result = HashMap::new();
        for parsing_method in ParsingMethod::iter() {
            result.insert(parsing_method, vec![
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "LR_fac_train_dim_various-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::Unknown, BrightnessLevel::CeilingVarious, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "LR_train_fac_litceil_5-10cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM5To10, BrightnessLevel::CeilingVarious, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "LR_train_fac_litceil_30cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM30, BrightnessLevel::Wall, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "LR_train_fac_wall_5-10cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM5To10, BrightnessLevel::Wall, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "UD_fac_train_dim_various-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::Unknown, BrightnessLevel::CeilingVarious, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "UD_train_fac_litceil_5-10cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM5To10, BrightnessLevel::CeilingLit, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "UD_train_fac_litceil_30cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM30, BrightnessLevel::CeilingLit, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", PATH_TRAINING, "UD_train_fac_wall_5-10cm-annotated.csv"), DataSetName::KubikTraining,
                    CoveringObject::Unknown, CameraDistance::CM5To10, BrightnessLevel::Wall, None, parsing_method),
            ]);
        }
        result
    };
}