extern crate lib_data_set;
extern crate lib_evaluation;
extern crate lib_feature;
extern crate lib_gesture;

fn main() {
    // TODO: Create application that is fed by Arduino and waits for input
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::process::Command;

    use lib_data_set::data_sets::eva::{EVA_16PIXEL, EVA_9PIXEL};
    use lib_data_set::data_sets::klisch::{KLISCH_DATA, KLISCH_TEST};
    use lib_data_set::data_sets::kubik::{KUBIK_TEST, KUBIK_TRAINING};
    use lib_data_set::data_sets::venzke::VENZKE_TRAINING;
    use lib_data_set::entities::DataSetEntry;
    use lib_data_set::value_objects::{DataSetName, ParsingMethod};
    use lib_evaluation::entities::Evaluation;
    use lib_evaluation::value_objects::EvaluationEntryKey;
    use lib_feature::{CenterOfGravityDistributionFloatX, CenterOfGravityDistributionFloatY, Feature};

    fn evaluate_data_set(data_set: &[DataSetEntry], data_set_name: DataSetName) {
        let mut evaluation = Evaluation::new(data_set_name);
        for data_set_entry in data_set.iter() {
            let evaluation_entry_key = EvaluationEntryKey::new(*data_set_entry.covering_object(), *data_set_entry.camera_distance(),
                                                               *data_set_entry.brightness_level(), *data_set_entry.additional_specification());
            for gesture in data_set_entry.gestures() {
                let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
                let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
                let mut args = Vec::with_capacity(12);
                args.append(&mut center_of_gravity_x.deref().to_vec());
                args.append(&mut center_of_gravity_y.deref().to_vec());

                let decision_tree = Command::new("./../decision_tree")
                    .args(&args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>())
                    .output()
                    .unwrap();
                let recognized_gesture = decision_tree.status.code().unwrap() as u8;
                if recognized_gesture == gesture.gesture_type as u8 {
                    evaluation.add_true_positive(evaluation_entry_key);
                } else {
                    evaluation.add_false_negative(evaluation_entry_key);
                }
            }
        }

        evaluation.print_results();
    }

    #[test]
    fn test_kubik_test_by_annotation() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTest);
    }

    #[test]
    fn test_kubik_test_by_threshold() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTest);
    }

    #[test]
    fn test_kubik_training_by_annotation() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTraining);
    }

    #[test]
    fn test_kubik_training_by_threshold() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTraining);
    }

    #[test]
    fn test_klisch_data_by_annotation() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischData);
    }

    #[test]
    fn test_klisch_data_by_threshold() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischData);
    }

    #[test]
    fn test_klisch_test_by_annotation() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischTest);
    }

    #[test]
    fn test_klisch_test_by_threshold() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischTest);
    }

    #[test]
    fn test_venzke_training_by_annotation() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::VenzkeTraining);
    }

    #[test]
    fn test_venzke_training_by_threshold() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::VenzkeTraining);
    }

    #[test]
    fn test_eva_16_pixel_by_annotation() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva16pixelData);
    }

    #[test]
    fn test_eva_16_pixel_by_threshold() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva16pixelData);
    }

    #[test]
    fn test_eva_9_pixel_by_annotation() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva9pixelData);
    }

    #[test]
    fn test_eva_9_pixel_by_threshold() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva9pixelData);
    }
}
