use std::collections::HashMap;

use crate::entities::DataSetEntry;
use crate::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};

lazy_static! {
    /// Automatically parses the dataDymel data set, once this constant is imported.
    /// Note: This set only supports the ByAnnotation ParsingMethod.
    pub static ref DYMEL_DATA: HashMap<ParsingMethod, Vec<DataSetEntry>> = {
        let path_test: String = format!("{}/data/dataDymel", std::env::var("DATA_PATH").unwrap());
        let mut result = HashMap::new();
        result.insert(ParsingMethod::ByAnnotation, vec![
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highContrast_3_to_10cm.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM3To10, BrightnessLevel::High, None, ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_veryHighContrast_3_to_10cm.csv"), DataSetName::DymelData,
                CoveringObject::GlassesCase, CameraDistance::CM3To10, BrightnessLevel::VeryHigh, None, ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_veryHighContrast_3_to_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::GlassesCase, CameraDistance::CM3To10, BrightnessLevel::VeryHigh, Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highContrast_20cm.csv"), DataSetName::DymelData,
                CoveringObject::GlassesCase, CameraDistance::CM20, BrightnessLevel::High, Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highContrast_5cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highContrast_3cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM3, BrightnessLevel::High, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_veryHighContrast_3cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM3, BrightnessLevel::VeryHigh, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowContrast_25cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_veryHighContrast_3cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM3, BrightnessLevel::VeryHigh, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_veryHighContrast_5cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::VeryHigh, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highContrast_10cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowContrast_15cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM15, BrightnessLevel::Low, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowContrast_25cm_no_padding.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low, Some(AdditionalSpecification::NoPadding),
                ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowContrast_20cm.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low, None, ParsingMethod::ByAnnotation),
        ]);
        result
    };
}