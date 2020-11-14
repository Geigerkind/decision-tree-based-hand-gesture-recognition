extern crate lib_feature;
extern crate lib_gesture;
extern crate strum;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

use lib_feature::FeatureType;
use lib_gesture::entities::Gesture;
use lib_gesture::tools::parse_gestures_by_annotation;

use crate::strum::IntoEnumIterator;

fn main() {
    let folder_fix = vec!["Eva9pixel", "Eva16pixel"];
    let direction_fix = vec!["LRRL", "TBBT", "Garbage"];
    let object_fix = vec!["hand", "finger", "lt"];
    let distance_fix = vec![3, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150];
    let brightness_fix = vec!["_lowBrightness", "_highBrightness", "_fullBrightness", "_halfBrightness", "_monotop", "_monotop_60"];
    let additional_fix = vec!["", "_fast", "_white", "_slow"];
    let number_fix = vec!["", "_0", "_1", "_2", "_3"];

    let start = Instant::now();
    let mut gestures: Vec<Gesture> = Vec::with_capacity(4500);
    for folder in &folder_fix {
        for direction in &direction_fix {
            for object in &object_fix {
                for distance in &distance_fix {
                    for brightness in &brightness_fix {
                        for additonal in &additional_fix {
                            for number in &number_fix {
                                if let Ok(mut parsed_gestures) = parse_gestures_by_annotation(&format!("./prepare_data/data{}/{}_{}_{}cm{}{}{}-annotated.csv", folder, direction, object, distance, brightness, additonal, number)) {
                                    gestures.append(&mut parsed_gestures);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // TODO: Create Garbage data from normal data, e.g. split at half and repeat in reverse
    // TODO: Syntetic data, i.e. scaling, rotation(?), noise(?), offsets?

    println!("Elapsed: {}ms", start.elapsed().as_millis());
    println!("Gestures found: {}", gestures.len());
    println!("Exporting features");
    let _ = std::fs::create_dir("./model_data");
    for feature_type in FeatureType::iter() {
        let mut file = File::create(&format!("model_data/{}", feature_type)).unwrap();
        for gesture in gestures.iter() {
            file.write_all(feature_type.to_feature(gesture).marshal().as_bytes()).unwrap();
            file.write_all(&[10]).unwrap();
        }
    }
    let mut file = File::create("model_data/result").unwrap();
    for gesture in gestures.iter() {
        file.write_all(format!("{}", gesture.gesture_type as u8).as_bytes()).unwrap();
        file.write_all(&[10]).unwrap();
    }
}