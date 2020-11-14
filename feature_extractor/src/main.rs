extern crate lib_feature;
extern crate lib_gesture;
extern crate lib_data_set;
extern crate strum;

use std::fs::File;
use std::io::Write;

use lib_feature::FeatureType;
use lib_gesture::entities::Gesture;
use lib_data_set::data_sets::eva::{EVA_16PIXEL, EVA_9PIXEL};

use crate::strum::IntoEnumIterator;
use lib_data_set::value_objects::ParsingMethod;

fn main() {
    let data_sets = vec![
        EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap(),
        EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap()
    ];

    let mut gestures: Vec<Gesture> = Vec::new();
    for data_set in data_sets {
        for data_set_entry in data_set {
            gestures.append(&mut data_set_entry.gestures().clone());
        }
    }

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