/*!
A utility using above libraries to extract all defined features from specified data sets and puts them into `model_data`. For each feature a file
is created with the feature's structure name. `result` is the `GestureType` that was annotated in the test data.
*/

#![allow(unused_imports)]

extern crate lib_data_set;
extern crate lib_evaluation;
extern crate lib_feature;
extern crate lib_gesture;
extern crate serialport;
extern crate num_traits;
extern crate rayon;

use std::io::{Write, Read};
use std::path::Path;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use lib_gesture::entities::{GestureReader, Frame, Gesture};
use std::str::FromStr;
use std::process::Command;
use lib_feature::{CenterOfGravityDistributionFloatX, CenterOfGravityDistributionFloatY, Feature, DarknessDistribution6XYGeom, BrightnessDistribution6XYGeom, MotionHistory, CenterOfGravityDistributionY, CenterOfGravityDistributionX, SumOfSlopes, MotionHistory2};
use std::ops::Deref;
use lib_gesture::value_objects::GestureType;
use num_traits::FromPrimitive;

const ASCII_NEW_LINE: u8 = 10;
const DO_FEATURE_REORDERING: bool = false;

/// This function calculates the currently selected features that are used by the decision tree and decision forest.
#[cfg(feature="feature_set1")]
fn calculate_features(gesture: &Gesture) -> Vec<f32> {
    let mut args: Vec<f32> = Vec::new();
    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());

    // Feature reordering
    if DO_FEATURE_REORDERING {
        let mut new_args = Vec::new();
        let half = args.len() / 2;
        for i in 0..half {
            new_args.push(args[i]);
            new_args.push(args[half + i]);
        }
        args = new_args;
    }

    args
}

#[cfg(feature="feature_set2")]
fn calculate_features(gesture: &Gesture) -> Vec<i32> {
    let mut args: Vec<i32> = Vec::new();
    let center_of_gravity_x = CenterOfGravityDistributionX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());
    args
}

#[cfg(feature="feature_set3")]
fn calculate_features(gesture: &Gesture) -> Vec<i32> {
    let mut args: Vec<i32> = Vec::new();
    let motion_history = MotionHistory::calculate(gesture);
    args.append(&mut motion_history.deref().iter().map(|val| *val as i32).collect());
    args
}

#[cfg(feature="feature_set4")]
fn calculate_features(gesture: &Gesture) -> Vec<i32> {
    let mut args: Vec<i32> = Vec::new();
    let darkness_dist_geom = DarknessDistribution6XYGeom::calculate(gesture);
    let brightness_dist_geom = BrightnessDistribution6XYGeom::calculate(gesture);
    args.append(&mut darkness_dist_geom.deref().iter().map(|val| *val as i32).collect());
    args.append(&mut brightness_dist_geom.deref().iter().map(|val| *val as i32).collect());
    args
}

#[cfg(feature="feature_set5")]
fn calculate_features(gesture: &Gesture) -> Vec<i32> {
    let mut args: Vec<i32> = Vec::new();
    let motion_history = MotionHistory::calculate(gesture);
    let darkness_dist_geom = DarknessDistribution6XYGeom::calculate(gesture);
    let brightness_dist_geom = BrightnessDistribution6XYGeom::calculate(gesture);
    args.append(&mut darkness_dist_geom.deref().iter().map(|val| *val as i32).collect());
    args.append(&mut brightness_dist_geom.deref().iter().map(|val| *val as i32).collect());
    args.append(&mut motion_history.deref().iter().map(|val| *val as i32).collect());
    args
}

/*
// DEPRECATED IN FAVOR OF FEATURE_SET10
#[cfg(feature="feature_set6")]
fn calculate_features(gesture: &Gesture) -> (Vec<f32>, Vec<i32>) {
    let mut f_args: Vec<f32> = Vec::new();
    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    f_args.append(&mut center_of_gravity_x.deref().to_vec());
    f_args.append(&mut center_of_gravity_y.deref().to_vec());

    let mut l_args: Vec<i32> = Vec::new();
    let center_of_gravity_x = CenterOfGravityDistributionX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionY::calculate(&gesture);
    l_args.append(&mut center_of_gravity_x.deref().to_vec());
    l_args.append(&mut center_of_gravity_y.deref().to_vec());
    (f_args, l_args)
}
 */

#[cfg(feature="feature_set7")]
fn calculate_features(gesture: &Gesture) -> Vec<u8> {
    let mut args: Vec<u8> = Vec::new();
    let motion_history_2 = MotionHistory2::calculate(&gesture);
    args.append(&mut motion_history_2.deref().to_vec());
    args
}

#[cfg(feature="feature_set8")]
fn calculate_features(gesture: &Gesture) -> Vec<f32> {
    unimplemented!()
    // DEPRECATED
    /*
    let mut args: Vec<f32> = Vec::new();
    let motion_history_2 = MotionHistory2::calculate(&gesture);
    args.append(&mut motion_history_2.deref().to_vec());
    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());
    args
     */
}

#[cfg(feature="feature_set9")]
fn calculate_features(gesture: &Gesture) -> Vec<f32> {
    let mut args: Vec<f32> = Vec::new();
    let motion_history_2 = MotionHistory2::calculate(&gesture);
    args.append(&mut motion_history_2.deref().to_vec());
    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());
    args
}

#[cfg(feature="feature_set10")]
fn calculate_features(gesture: &Gesture) -> (Vec<i32>, Vec<f32>) {
    let mut f_args: Vec<f32> = Vec::new();

    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    f_args.append(&mut center_of_gravity_x.deref().to_vec());
    f_args.append(&mut center_of_gravity_y.deref().to_vec());

    let mut l_args: Vec<i32> = Vec::new();
    let center_of_gravity_x = CenterOfGravityDistributionX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionY::calculate(&gesture);
    l_args.append(&mut center_of_gravity_x.deref().to_vec());
    l_args.append(&mut center_of_gravity_y.deref().to_vec());
    (l_args, f_args)
}

fn main() {
    // The Arduino serial sends to the /dev/ttyACM0 port.
    let mut port = serialport::posix::TTYPort::open(&Path::new("/dev/ttyACM0"), &SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    }).expect("Failed to open port");

    // We read each byte individually and clear the buffer once we encounter a newline
    // If so, we assume its a frame and attempt to parse it.
    // This frame is then fed to the gesture reader to find a gesture candidate
    // Which is then fed to the decision tree.
    let mut serial_buf: Vec<u8> = vec![0; 1];
    let _ = port.flush();
    // Read until first end of line
    loop {
        if port.read(&mut serial_buf).is_ok() {
            if serial_buf[0] == ASCII_NEW_LINE {
                break;
            }
        }
    }

    let mut gesture_reader = GestureReader::new(0.05, 0.01, 0.2, true);
    let mut line = Vec::with_capacity(28);
    loop {
        if port.read(&mut serial_buf).is_ok() {
            line.push(serial_buf[0]);
            if serial_buf[0] == ASCII_NEW_LINE {
                if let Ok(line) = std::str::from_utf8(&line) {
                    if let Ok(frame) = Frame::from_str(line.trim_end_matches("\r\n")) {
                        if let Some(gesture) = gesture_reader.feed_frame(frame) {
                            println!("#Frames: {}", gesture.frames.len());

                            #[cfg(feature="feature_set10")]
                            let program_args: Vec<String> = {
                                let (l_args, f_args) = calculate_features(&gesture);
                                let mut args = l_args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>();
                                args.append(&mut f_args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>());
                                args
                            };

                            #[cfg(not(feature="feature_set10"))]
                            let program_args: Vec<String> = calculate_features(&gesture).into_iter().map(|value| value.to_string()).collect::<Vec<String>>();

                            let decision_tree = Command::new("./decision_forest")
                                .args(&program_args)
                                .output()
                                .unwrap();
                            let gesture_type: GestureType = FromPrimitive::from_i32(decision_tree.status.code().unwrap()).unwrap();
                            //gesture.print();
                            println!("Recognized gesture: {:?}", gesture_type);
                        }
                    }
                }
                line.clear();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::process::Command;

    use lib_data_set::data_sets::dymel::{DYMEL_GESTURE_TEST, DYMEL_NULL_TEST, DYMEL_LIGHT_TEST, DYMEL_VANISHING_CONTRAST_TEST};
    use lib_data_set::data_sets::eva::{EVA_16PIXEL, EVA_9PIXEL};
    use lib_data_set::data_sets::klisch::{KLISCH_DATA, KLISCH_TEST};
    use lib_data_set::data_sets::kubik::{KUBIK_TEST, KUBIK_TRAINING};
    use lib_data_set::data_sets::venzke::VENZKE_TRAINING;
    use lib_data_set::entities::DataSetEntry;
    use lib_data_set::value_objects::{DataSetName, ParsingMethod, CoveringObject, CameraDistance, BrightnessLevel};
    use lib_evaluation::entities::Evaluation;
    use lib_evaluation::value_objects::EvaluationEntryKey;
    use crate::calculate_features;
    use lib_gesture::value_objects::GestureType;
    use rayon::iter::ParallelIterator;
    use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, IntoParallelIterator};
    use std::fs::File;
    use std::io::Write;

    fn evaluate_data_set(data_set: &[DataSetEntry], data_set_name: DataSetName, program: &str) -> Evaluation {
        let evaluations: Vec<Evaluation> = data_set.clone().into_par_iter().map(|data_set_entry| {
            let mut evaluation = Evaluation::new(data_set_name);
            let evaluation_entry_key = EvaluationEntryKey::new(*data_set_entry.covering_object(), *data_set_entry.camera_distance(),
                                                               *data_set_entry.brightness_level(), *data_set_entry.additional_specification(),
                                                               *data_set_entry.offset(), *data_set_entry.scaling());

            let path = std::env::var("PROGRAM_PATH").unwrap();
            for gesture in data_set_entry.gestures() {
                #[cfg(feature="feature_set10")]
                    let program_args: Vec<String> = {
                    let (f_args, l_args) = calculate_features(&gesture);
                    let mut args = f_args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>();
                    args.append(&mut l_args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>());
                    args
                };

                #[cfg(not(feature="feature_set10"))]
                let program_args: Vec<String> = calculate_features(&gesture).into_iter().map(|value| value.to_string()).collect::<Vec<String>>();
                let decision_tree = Command::new(&format!("{}/{}", path, program))
                    .args(&program_args)
                    .output()
                    .unwrap();
                let recognized_gesture = decision_tree.status.code().unwrap() as u8;
                if recognized_gesture == gesture.gesture_type as u8 {
                    evaluation.add_true_positive(evaluation_entry_key);
                } else {
                    evaluation.add_false_negative(evaluation_entry_key);
                }
            }
            evaluation
        }).collect();

        let res_eval = Evaluation::merge(evaluations);
        res_eval.print_results();
        res_eval
    }

    #[test]
    fn test_kubik_test_by_annotation_decision_tree() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTest, "decision_tree");
    }

    #[test]
    fn test_kubik_test_by_threshold_decision_tree() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTest, "decision_tree");
    }

    #[test]
    fn test_kubik_training_by_annotation_decision_tree() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTraining, "decision_tree");
    }

    #[test]
    fn test_kubik_training_by_threshold_decision_tree() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTraining, "decision_tree");
    }

    #[test]
    fn test_klisch_data_by_annotation_decision_tree() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischData, "decision_tree");
    }

    #[test]
    fn test_klisch_data_by_threshold_decision_tree() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischData, "decision_tree");
    }

    #[test]
    fn test_klisch_test_by_annotation_decision_tree() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischTest, "decision_tree");
    }

    #[test]
    fn test_klisch_test_by_threshold_decision_tree() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischTest, "decision_tree");
    }

    #[test]
    fn test_venzke_training_by_annotation_decision_tree() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::VenzkeTraining, "decision_tree");
    }

    #[test]
    fn test_venzke_training_by_threshold_decision_tree() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::VenzkeTraining, "decision_tree");
    }

    #[test]
    fn test_eva_16_pixel_by_annotation_decision_tree() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva16pixelData, "decision_tree");
    }

    #[test]
    fn test_eva_16_pixel_by_threshold_decision_tree() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva16pixelData, "decision_tree");
    }

    #[test]
    fn test_eva_9_pixel_by_annotation_decision_tree() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva9pixelData, "decision_tree");
    }

    #[test]
    fn test_eva_9_pixel_by_threshold_decision_tree() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva9pixelData, "decision_tree");
    }

    #[test]
    fn test_kubik_test_by_annotation_decision_forest() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTest, "decision_forest");
    }

    #[test]
    fn test_kubik_test_by_threshold_decision_forest() {
        evaluate_data_set(KUBIK_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTest, "decision_forest");
    }

    #[test]
    fn test_kubik_training_by_annotation_decision_forest() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KubikTraining, "decision_forest");
    }

    #[test]
    fn test_kubik_training_by_threshold_decision_forest() {
        evaluate_data_set(KUBIK_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KubikTraining, "decision_forest");
    }

    #[test]
    fn test_klisch_data_by_annotation_decision_forest() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischData, "decision_forest");
    }

    #[test]
    fn test_klisch_data_by_threshold_decision_forest() {
        evaluate_data_set(KLISCH_DATA.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischData, "decision_forest");
    }

    #[test]
    fn test_klisch_test_by_annotation_decision_forest() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::KlischTest, "decision_forest");
    }

    #[test]
    fn test_klisch_test_by_threshold_decision_forest() {
        evaluate_data_set(KLISCH_TEST.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::KlischTest, "decision_forest");
    }

    #[test]
    #[ignore]
    fn test_klisch_test_by_annotation_decision_forest_with_synthetic_data() {
        let mut data_set = KLISCH_TEST.get(&ParsingMethod::ByAnnotation).unwrap().clone();
        let mut synthetic = Vec::new();
        for entry in data_set.iter() {
            for gesture in entry.gestures() {
                synthetic.append(&mut gesture.infer_garbage());
                //synthetic.append(&mut gesture.infer_diagonal());
                //synthetic.append(&mut gesture.infer_shifting());
            }
        }
        data_set.push(DataSetEntry::custom(DataSetName::KlischTest, CoveringObject::Unknown, CameraDistance::Unknown, BrightnessLevel::Unknown, None, synthetic));
        evaluate_data_set(&data_set, DataSetName::KlischTest, "decision_forest");
    }

    #[test]
    fn test_venzke_training_by_annotation_decision_forest() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::VenzkeTraining, "decision_forest");
    }

    #[test]
    fn test_venzke_training_by_threshold_decision_forest() {
        evaluate_data_set(VENZKE_TRAINING.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::VenzkeTraining, "decision_forest");
    }

    #[test]
    fn test_eva_16_pixel_by_annotation_decision_forest() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva16pixelData, "decision_forest");
    }

    #[test]
    fn test_eva_16_pixel_by_threshold_decision_forest() {
        evaluate_data_set(EVA_16PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva16pixelData, "decision_forest");
    }

    #[test]
    fn test_eva_9_pixel_by_annotation_decision_forest() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap().deref(), DataSetName::Eva9pixelData, "decision_forest");
    }

    #[test]
    fn test_eva_9_pixel_by_threshold_decision_forest() {
        evaluate_data_set(EVA_9PIXEL.get(&ParsingMethod::ByThreshold).unwrap().deref(), DataSetName::Eva9pixelData, "decision_forest");
    }

    #[test]
    fn test_dymel_test_gesture_by_annotation_decision_forest() {
        evaluate_data_set(&DYMEL_GESTURE_TEST, DataSetName::DymelData, "decision_forest");
    }

    #[test]
    fn test_dymel_test_null_by_annotation_decision_forest() {
        evaluate_data_set(&DYMEL_NULL_TEST, DataSetName::DymelData, "decision_forest");
    }

    #[test]
    fn test_dymel_test_light_by_annotation_decision_forest() {
        let evaluation = evaluate_data_set(&DYMEL_LIGHT_TEST, DataSetName::DymelData, "decision_forest");

        let mut file = File::create("../light_test.csv").unwrap();
        let _ =file.write_all(b"ansatz,offset,scaling,accuracy");
        for (key, entry) in evaluation.entries.iter() {
            let offset = key.offset.unwrap_or(0);
            let scaling = key.scaling.unwrap_or(0);
            let _ =file.write_all(b"\n");
            let _ =file.write_all(format!("2,{},{},{}", offset, scaling, entry.accuracy().unwrap_or(0.0)).as_bytes());
        }
    }

    #[test]
    fn test_dymel_test_vanashing_contrast_by_annotation_decision_forest() {
        let evaluation = evaluate_data_set(&DYMEL_VANISHING_CONTRAST_TEST, DataSetName::DymelData, "decision_forest");

        let mut file = File::create("../light_test2.csv").unwrap();
        let _ =file.write_all(b"ansatz,offset,scaling,accuracy");
        for (key, entry) in evaluation.entries.iter() {
            let scaling = key.scaling.unwrap_or(0);
            let _ =file.write_all(b"\n");
            let _ =file.write_all(format!("2,0,{},{}", scaling, entry.accuracy().unwrap_or(0.0)).as_bytes());
        }
    }
}
