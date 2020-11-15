extern crate lib_feature;
extern crate lib_gesture;
extern crate lib_data_set;
extern crate strum;
extern crate rand;

use std::fs::File;
use std::io::Write;

use lib_feature::FeatureType;
use lib_gesture::entities::Gesture;
use lib_data_set::data_sets::eva::{EVA_16PIXEL, EVA_9PIXEL};
use lib_data_set::data_sets::kubik::{KUBIK_TRAINING, KUBIK_TEST};
use lib_data_set::data_sets::klisch::{KLISCH_TEST, KLISCH_DATA};

use crate::strum::IntoEnumIterator;
use lib_data_set::value_objects::ParsingMethod;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let data_sets = vec![
        EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap(),
        EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap(),
        KUBIK_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap(),
        KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap(),
        KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap(),
        KLISCH_DATA.get(&ParsingMethod::ByAnnotation).unwrap(),
    ];

    let mut gestures: Vec<Gesture> = Vec::new();
    let mut synthetic_rotations: Vec<Gesture> = Vec::new();
    let mut synthetic_garbage: Vec<Gesture> = Vec::new();
    for data_set in data_sets {
        for data_set_entry in data_set {
            gestures.append(&mut data_set_entry.gestures().clone());
            for gesture in data_set_entry.gestures() {
                synthetic_rotations.append(&mut gesture.infer_rotations());
                synthetic_garbage.append(&mut gesture.infer_garbage());
            }
        }
    }

    synthetic_rotations.shuffle(&mut thread_rng());
    synthetic_garbage.shuffle(&mut thread_rng());

    //gestures.append(&mut synthetic_garbage[0..(gestures.len() / 4)].to_vec());

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