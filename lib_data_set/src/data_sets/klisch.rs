use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::entities::DataSetEntry;
use crate::value_objects::{BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};

lazy_static! {
    pub static ref KLISCH_DATA: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let path_data: String = format!("{}/data/dataKlisch", std::env::var("DATA_PATH").unwrap());
        let mut result = HashMap::new();
        for parsing_method in ParsingMethod::iter() {
            result.insert(parsing_method, vec![
                DataSetEntry::new(format!("{}/{}", path_data, "MyHighContrast_5cm_Annotated.csv"), DataSetName::KlischData,
                    CoveringObject::Unknown, CameraDistance::CM5, BrightnessLevel::High, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_data, "MyLowContrast_5cm_Annotated.csv"), DataSetName::KlischData,
                    CoveringObject::Unknown, CameraDistance::CM5, BrightnessLevel::Low, None, parsing_method),
            ]);
        }
        result
    };
}

lazy_static! {
    // TODO: Consider CameraType?
    pub static ref KLISCH_TEST: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let path_test: String = format!("{}/data/testKlisch", std::env::var("DATA_PATH").unwrap());
        let mut result = HashMap::new();
        for parsing_method in ParsingMethod::iter() {
            result.insert(parsing_method, vec![
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_highcontrast_3cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM3, BrightnessLevel::High, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_highcontrast_15cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM15, BrightnessLevel::High, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_highcontrast_20cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM20, BrightnessLevel::High, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_highcontrast_30cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM30, BrightnessLevel::High, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_lowcontrast_3cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM3, BrightnessLevel::Low, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_lowcontrast_15cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM15, BrightnessLevel::Low, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_lowcontrast_20cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM20, BrightnessLevel::Low, None, parsing_method),
                DataSetEntry::new(format!("{}/{}", path_test, "test_fac_lowcontrast_30cm-annotated.csv"), DataSetName::KlischTest,
                    CoveringObject::Unknown, CameraDistance::CM30, BrightnessLevel::Low, None, parsing_method),
            ]);
        }
        result
    };
}