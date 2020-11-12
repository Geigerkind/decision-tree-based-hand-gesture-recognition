#![allow(unused_imports)]

#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate plotters;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::thread_rng;
use rand::seq::SliceRandom;

use plotters::prelude::*;

use tools::parse_gestures;

use crate::entities::Gesture;
use crate::strum::IntoEnumIterator;
use crate::value_objects::GestureType;
use crate::features::{LocalSumOfSlopeX, Feature, FeatureType};

mod value_objects;
mod entities;
mod tools;
mod features;

fn main() {
    let folder_fix = vec![9, 16];
    let direction_fix = vec!["LRRL", "TBBT"];
    let object_fix = vec!["hand", "finger"];
    let distance_fix = vec![3, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150];
    let brightness_fix = vec!["_lowBrightness", "_highBrightness", "_fullBrightness", "_halfBrightness", "_monotop", "_monotop_60"];
    let additional_fix = vec!["", "_fast", "_white", "_slow"];
    let number_fix = vec!["", "_0", "_1", "_2", "_3"];

    let start = Instant::now();
    let mut gestures: Vec<Gesture> = Vec::with_capacity(500);
    for folder in &folder_fix {
        for direction in &direction_fix {
            for object in &object_fix {
                for distance in &distance_fix {
                    for brightness in &brightness_fix {
                        for additonal in &additional_fix {
                            for number in &number_fix {
                                if let Ok(mut parsed_gestures) = parse_gestures(&format!("../dataEva{}pixel/{}_{}_{}cm{}{}{}-annotated.csv", folder, direction, object, distance, brightness, additonal, number)) {
                                    gestures.append(&mut parsed_gestures);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Elapsed: {}ms", start.elapsed().as_millis());
    println!("Gestures found: {}", gestures.len());
    /*
    println!("\nFeatures:");
    create_chart_local_sum_of_slopes_x(&gestures);
    create_chart_local_sum_of_slopes_y(&gestures);
    create_chart_local_sum_of_slopes_avg_x_y(&gestures);
     */
    println!("Exporting features");
    gestures.shuffle(&mut thread_rng());
    let amount_train = gestures.len() / 2;
    let train_gestures: Vec<&Gesture> = gestures.iter().take(amount_train).collect();
    let validate_gestures: Vec<&Gesture> = gestures.iter().skip(amount_train).collect();
    for feature_type in FeatureType::iter() {
        let mut train_file = File::create(&format!("model_data/train/{}", feature_type)).unwrap();
        for gesture in &train_gestures {
            train_file.write_all(feature_type.to_feature(gesture).marshal().as_bytes()).unwrap();
            train_file.write_all(&[10]).unwrap();
        }

        let mut validate_file = File::create(&format!("model_data/validate/{}", feature_type)).unwrap();
        for gesture in &validate_gestures {
            validate_file.write_all(feature_type.to_feature(gesture).marshal().as_bytes()).unwrap();
            validate_file.write_all(&[10]).unwrap();
        }
    }
    let mut train_file = File::create("model_data/train/result").unwrap();
    let mut validate_file = File::create("model_data/validate/result").unwrap();
    for gesture in &train_gestures {
        train_file.write_all(format!("{}", gesture.gesture_type as u8).as_bytes()).unwrap();
        train_file.write_all(&[10]).unwrap();
    }
    for gesture in &validate_gestures {
        validate_file.write_all(format!("{}", gesture.gesture_type as u8).as_bytes()).unwrap();
        validate_file.write_all(&[10]).unwrap();
    }
}
/*
fn get_chart_color_for_gesture(gesture_type: GestureType) -> ShapeStyle {
    match gesture_type {
        GestureType::LeftToRight => GREEN.filled(),
        GestureType::RightToLeft => RED.filled(),
        GestureType::TopToBottom => BLUE.filled(),
        GestureType::BottomToTop => YELLOW.filled(),
        _ => BLACK.filled()
    }
}

fn create_chart_local_sum_of_slopes_x(gestures: &Vec<Gesture>) {
    println!("Local Sum of Slopes - x");
    let root = BitMapBackend::new("plots/local_sum_of_slopes_x.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Local Sum of Slopes - x", ("sans-serif", 40))
        .build_cartesian_3d(-100..100, -100..100, -100..100)
        .unwrap();
    for gesture_type in GestureType::iter() {
        if gesture_type == GestureType::NotGesture || gesture_type == GestureType::NotGesture
            || gesture_type == GestureType::TopToBottom || gesture_type == GestureType::BottomToTop {
            continue;
        }

        let points: Vec<[i16; 3]> = gestures.iter().filter(|gesture| gesture.gesture_type == gesture_type)
            .map(|gesture| gesture.calc_feature_sum_of_slopes_x()).collect();
        chart.draw_series(points.iter().map(|[x1, x2, x3]|
            Circle::new((*x1 as i32, *x2 as i32, *x3 as i32), 2, get_chart_color_for_gesture(gesture_type)))).unwrap();
    }
    chart.configure_axes().draw().unwrap();
}

fn create_chart_local_sum_of_slopes_y(gestures: &Vec<Gesture>) {
    println!("Local Sum of Slopes - y");
    let root = BitMapBackend::new("plots/local_sum_of_slopes_y.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Local Sum of Slopes - y", ("sans-serif", 40))
        .build_cartesian_3d(-100..100, -100..100, -100..100)
        .unwrap();
    for gesture_type in GestureType::iter() {
        if gesture_type == GestureType::NotGesture || gesture_type == GestureType::NotGesture
            || gesture_type == GestureType::LeftToRight || gesture_type == GestureType::RightToLeft {
            continue;
        }

        let points: Vec<[i16; 3]> = gestures.iter().filter(|gesture| gesture.gesture_type == gesture_type)
            .map(|gesture| gesture.calc_feature_sum_of_slopes_y()).collect();
        chart.draw_series(points.iter().map(|[x1, x2, x3]|
            Circle::new((*x1 as i32, *x2 as i32, *x3 as i32), 2, get_chart_color_for_gesture(gesture_type)))).unwrap();
    }
    chart.configure_axes().draw().unwrap();
}

fn create_chart_local_sum_of_slopes_avg_x_y(gestures: &Vec<Gesture>) {
    println!("Local Sum of Slopes Avg - X vs. Y ");
    let root = BitMapBackend::new("plots/local_sum_of_slopes_avg_x_y.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Local Sum of Slopes Avg. - X vs. Y", ("sans-serif", 40))
        .build_cartesian_2d(-100..100, -100..100)
        .unwrap();
    for gesture_type in GestureType::iter() {
        if gesture_type == GestureType::NotGesture || gesture_type == GestureType::NotGesture {
            continue;
        }

        let mut result: Vec<(i16, i16)> = Vec::new();
        let points_x: Vec<i16> = gestures.iter().filter(|gesture| gesture.gesture_type == gesture_type)
            .map(|gesture| gesture.calc_feature_sum_of_slopes_x()).map(|[x1, x2, x3]| (x1 + x2 + x3) / 3).collect();
        let points_y: Vec<i16> = gestures.iter().filter(|gesture| gesture.gesture_type == gesture_type)
            .map(|gesture| gesture.calc_feature_sum_of_slopes_y()).map(|[x1, x2, x3]| (x1 + x2 + x3) / 3).collect();
        for i in 0..points_x.len() {
            result.push((points_x[i], points_y[i]));
        }

        chart.draw_series(result.into_iter().map(|(x, y)|
            Circle::new((x as i32, y as i32), 2, get_chart_color_for_gesture(gesture_type)))).unwrap();
    }
    chart.configure_mesh().draw().unwrap();
}
 */