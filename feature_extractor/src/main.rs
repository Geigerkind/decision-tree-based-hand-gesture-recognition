/*!
This is a program that listens to the serial port `/dev/ttyACMO` (depending on your operating system, you need to change this) and waits for a stream of
frames from the Arduino Uno. It then feeds the frames into the `GestureReader`, and calculates, once a `Gesture`(Candidate) is recognized the feature vector,
and finally feeds it into the decision tree or decision forest that is generated in `model/decision_tree.py`.
Furthermore, it tests the aforementioned tree classifier on the data sets and prints out `Evaluation`s for each data set parsed `ByAnnotation` and `ByThreshold`.
*/

#![allow(unused_imports)]

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

const ASCII_NEW_LINE: u8 = 10;

fn main() {
    // All data sets that are going to be processed
    let data_sets = vec![
        EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap(),
        EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap(),
        KUBIK_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap(),
        //KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap(),
        //KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap(),
        //KLISCH_DATA.get(&ParsingMethod::ByAnnotation).unwrap(),
    ];

    let mut gestures: Vec<Gesture> = Vec::new();
    let mut synthetic_rotations: Vec<Gesture> = Vec::new();
    let mut synthetic_garbage: Vec<Gesture> = Vec::new();
    let mut synthetic_shifted: Vec<Gesture> = Vec::new();
    let mut synthetic_diagonal: Vec<Gesture> = Vec::new();
    for data_set in data_sets {
        for data_set_entry in data_set {
            gestures.append(&mut data_set_entry.gestures().clone());
            // Also create synthetic data, rotations and garbage
            for gesture in data_set_entry.gestures() {
                synthetic_rotations.append(&mut gesture.infer_rotations());
                synthetic_garbage.append(&mut gesture.infer_garbage());
                synthetic_shifted.append(&mut gesture.infer_shifting());
                synthetic_diagonal.append(&mut gesture.infer_diagonal());
            }
        }
    }

    // We dont want to create a bias if we only add a certain amount of it
    synthetic_rotations.shuffle(&mut thread_rng());
    synthetic_garbage.shuffle(&mut thread_rng());
    synthetic_shifted.shuffle(&mut thread_rng());
    synthetic_diagonal.shuffle(&mut thread_rng());

    // If we add garbage or rotations, we dont want to add too many
    // gestures.append(&mut synthetic_rotations[0..(gestures.len() / 5)].to_vec());
    // gestures.append(&mut synthetic_garbage[0..(gestures.len() / 4)].to_vec());
    // gestures.append(&mut synthetic_shifted[0..(gestures.len() / 4)].to_vec());
    // gestures.append(&mut synthetic_diagonal[0..(gestures.len() / 5)].to_vec());

    // This creates for each feature a file in model_data
    println!("Gestures found: {}", gestures.len());
    println!("Exporting features");
    let _ = std::fs::create_dir("./model_data");
    for feature_type in FeatureType::iter() {
        let mut file = File::create(&format!("model_data/{}", feature_type)).unwrap();
        for gesture in gestures.iter() {
            file.write_all(feature_type.to_feature(gesture).marshal().as_bytes()).unwrap();
            file.write_all(&[ASCII_NEW_LINE]).unwrap();
        }
    }
    let mut file = File::create("model_data/result").unwrap();
    for gesture in gestures.iter() {
        file.write_all(format!("{}", gesture.gesture_type as u8).as_bytes()).unwrap();
        file.write_all(&[ASCII_NEW_LINE]).unwrap();
    }
}