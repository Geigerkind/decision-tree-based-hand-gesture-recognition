extern crate lib_data_set;
extern crate lib_evaluation;
extern crate lib_feature;
extern crate lib_gesture;

use std::ops::Deref;
use std::process::Command;

use lib_data_set::data_sets::kubik::KUBIK_TEST;
use lib_data_set::value_objects::{ParsingMethod, DataSetName};
use lib_evaluation::entities::Evaluation;
use lib_evaluation::value_objects::EvaluationEntryKey;
use lib_feature::{CenterOfGravityDistributionFloatX, CenterOfGravityDistributionFloatY, Feature};

fn main() {
    let mut evaluation = Evaluation::new(DataSetName::KubikTest);
    for data_set_entry in KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref().iter() {
        let evaluation_entry_key = EvaluationEntryKey::new(*data_set_entry.covering_object(), *data_set_entry.camera_distance(),
                                                           *data_set_entry.brightness_level(), *data_set_entry.additional_specification());
        for gesture in data_set_entry.gestures() {
            let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
            let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
            let mut args = Vec::with_capacity(12);
            args.append(&mut center_of_gravity_x.deref().to_vec());
            args.append(&mut center_of_gravity_y.deref().to_vec());

            let decision_tree = Command::new("./decision_tree")
                .args(&args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>())
                .output()
                .unwrap();
            let recogniced_gesture = decision_tree.status.code().unwrap() as u8;
            if recogniced_gesture == gesture.gesture_type as u8 {
                evaluation.add_true_positive(evaluation_entry_key);
            } else {
                evaluation.add_false_negative(evaluation_entry_key);
            }
        }
    }

    evaluation.print_results();
}
