use crate::entities::DataSetEntry;
use crate::value_objects::{AdditionalSpecification, BrightnessLevel, CameraDistance, CoveringObject, DataSetName, ParsingMethod};

lazy_static! {
    pub static ref DYMEL_NULL: Vec<DataSetEntry> = {
        let path_test: String = format!("{}/data/dataDymel", std::env::var("DATA_PATH").unwrap());
        let mut result = vec![
            // LowBrightness
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_lowBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_lowBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            // MediumBrightness
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_mediumBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_mediumBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            // HighBrightness
            /*
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_Corner_highBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureCorner), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "NULL_SameInAndOut_highBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::NullGestureSameInAndOut), ParsingMethod::ByAnnotation),
             */
        ];

        // Infer rotations for null gestures
        let mut additional_entries = Vec::new();
        for entry in result.iter().filter(|entry| *entry.additional_specification() == Some(AdditionalSpecification::NullGestureCorner)
            || *entry.additional_specification() == Some(AdditionalSpecification::NullGestureSameInAndOut)) {
            let mut additional_gestures = Vec::new();
            for gesture in entry.gestures() {
                additional_gestures.append(&mut gesture.infer_rotations());
            }

            additional_entries.push(DataSetEntry::custom(DataSetName::DymelData, CoveringObject::Hand, *entry.camera_distance(),
                *entry.brightness_level(), *entry.additional_specification(), additional_gestures));
        }
        result.append(&mut additional_entries);
        result
    };

    pub static ref DYMEL_NULL_TRAINING: Vec<DataSetEntry> = {
        let mut data = DYMEL_NULL.clone();
        data.iter_mut().for_each(|entry| entry.take_percent_until(0.125));
        data
    };

    pub static ref DYMEL_NULL_TEST: Vec<DataSetEntry> = {
        let mut data = DYMEL_NULL.clone();
        data.iter_mut().for_each(|entry| entry.take_percent_from(0.125));
        data
    };

    /// Automatically parses the dataDymel data set, once this constant is imported.
    /// Note: This set only supports the ByAnnotation ParsingMethod.
    pub static ref DYMEL_GESTURE: Vec<DataSetEntry> = {
        let path_test: String = format!("{}/data/dataDymel", std::env::var("DATA_PATH").unwrap());
        vec![
            // LowBrightness
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_lowBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_lowBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Low,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            // MediumBrightness
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_mediumBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_mediumBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::Medium,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            // HighBrightness
            /*
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "LRRL_highBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_5cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_5cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM5, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_10cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_10cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM10, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_20cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_20cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM20, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_25cm_fast.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::Fast), ParsingMethod::ByAnnotation),
            DataSetEntry::new(format!("{}/{}", path_test, "TBBT_highBrightness_25cm_slow.csv"), DataSetName::DymelData,
                CoveringObject::Hand, CameraDistance::CM25, BrightnessLevel::High,
                Some(AdditionalSpecification::Slow), ParsingMethod::ByAnnotation),
             */
        ]
    };

    pub static ref DYMEL_GESTURE_TRAINING: Vec<DataSetEntry> = {
        let mut data = DYMEL_GESTURE.clone();
        data.iter_mut().for_each(|entry| entry.take_percent_until(0.25));
        data
    };

    pub static ref DYMEL_GESTURE_TEST: Vec<DataSetEntry> = {
        let mut data = DYMEL_GESTURE.clone();
        data.iter_mut().for_each(|entry| entry.take_percent_from(0.25));
        data
    };

    pub static ref DYMEL_LIGHT_TEST: Vec<DataSetEntry> = {
        let mut data: Vec<DataSetEntry> = DYMEL_GESTURE.clone().into_iter().filter(|entry| *entry.brightness_level() == BrightnessLevel::Low).collect();
        let mut additional_entries = Vec::new();
        // Infer lighting conditions by scaling and offsets
        for entry in data.iter() {
            for i in 1..17 {
                let mut offset = entry.clone();
                offset.add_offset((i as i16) * 50);
                additional_entries.push(offset);

                if 1.0 + (i as f32) * 0.5 < 7.5 {
                    let mut scaled = entry.clone();
                    scaled.scale_by(1.0 + (i as f32) * 0.5);
                    additional_entries.push(scaled);
                }
            }
        }
        data.append(&mut additional_entries);
        data
    };

    pub static ref DYMEL_VANISHING_CONTRAST_TEST: Vec<DataSetEntry> = {
        let mut data: Vec<DataSetEntry> = DYMEL_GESTURE.clone().into_iter().filter(|entry| *entry.brightness_level() == BrightnessLevel::Medium).collect();
        let mut additional_entries = Vec::new();

        // Maintain same brightness but reduce contrast
        for entry in data.iter() {
            for i in 1..20 {
                let mut new_entry = entry.clone();
                let pixel_sum_before: Vec<Vec<i16>> = entry.gestures().iter().map(|gesture| gesture.frames.iter().map(|frame| frame.pixel.iter().sum()).collect::<Vec<i16>>()).collect();
                new_entry.scale_by(((1.0f32 - (i as f32) * 0.05f32) * 100.0f32).round() / 100.0f32);
                for gesture_count in 0..new_entry.gestures.len() {
                    for frame_count in 0..new_entry.gestures[gesture_count].frames.len() {
                        let frame = new_entry.gestures[gesture_count].frames.get_mut(frame_count).unwrap();
                        let pixel_sum: i16 = frame.pixel.iter().sum();
                        let pixel_diff_avg = (pixel_sum_before[gesture_count][frame_count] - pixel_sum) / 9;
                        for pixel in frame.pixel.iter_mut() {
                            *pixel += pixel_diff_avg;
                        }
                    }
                }
                additional_entries.push(new_entry);
            }
        }
        data.append(&mut additional_entries);
        data
    };
}