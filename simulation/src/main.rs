extern crate lib_feature;
extern crate lib_gesture;

use std::process::Command;

use lib_gesture::tools::parse_gestures_by_threshold;
use lib_feature::{CenterOfGravityDistributionFloatX, Feature, CenterOfGravityDistributionFloatY};
use std::ops::Deref;

fn main() {
    let gestures = parse_gestures_by_threshold(&String::from("./../prepare_data/dataEva9pixel/LRRL_finger_3cm_highBrightness-annotated.csv")).unwrap();
    let mut correct = 0;
    let total_gestures = gestures.len();
    for gesture in gestures {
        let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
        let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
        let mut args = Vec::with_capacity(12);
        args.append(&mut center_of_gravity_x.deref().to_vec());
        args.append(&mut center_of_gravity_y.deref().to_vec());

        let decision_tree = Command::new("./../decision_tree")
            .args(&args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>())
            .output()
            .unwrap();
        let recogniced_gesture = decision_tree.status.code().unwrap() as u8;
        if recogniced_gesture == gesture.gesture_type as u8 {
            correct += 1;
        }
    }

    println!("Accuracy: {}", 100.0 * ((correct as f64) / (total_gestures as f64)));
}
