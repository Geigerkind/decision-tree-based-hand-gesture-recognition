extern crate lib_feature;
extern crate lib_gesture;
extern crate lib_data_set;

use std::process::Command;

use lib_feature::{CenterOfGravityDistributionFloatX, Feature, CenterOfGravityDistributionFloatY};
use std::ops::Deref;

use lib_data_set::data_sets::kubik::KUBIK_TEST;
use lib_data_set::value_objects::ParsingMethod;

fn main() {
    let mut correct = 0;
    let mut total_gestures = 0;
    for data_set_entry in KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref().iter() {
        for gesture in data_set_entry.gestures() {
            let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
            let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
            let mut args = Vec::with_capacity(12);
            args.append(&mut center_of_gravity_x.deref().to_vec());
            args.append(&mut center_of_gravity_y.deref().to_vec());
            total_gestures += 1;

            let decision_tree = Command::new("./../decision_tree")
                .args(&args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>())
                .output()
                .unwrap();
            let recogniced_gesture = decision_tree.status.code().unwrap() as u8;
            if recogniced_gesture == gesture.gesture_type as u8 {
                correct += 1;
            }
        }
    }

    println!("Accuracy: {}", 100.0 * ((correct as f64) / (total_gestures as f64)));
}
