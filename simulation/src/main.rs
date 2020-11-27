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

use std::io::{Write, Read};
use std::path::Path;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use lib_gesture::entities::{GestureReader, Frame, Gesture};
use std::str::FromStr;
use std::process::Command;
use lib_feature::{CenterOfGravityDistributionFloatX, CenterOfGravityDistributionFloatY, Feature, DarknessDistribution6XYGeom, BrightnessDistribution6XYGeom, MotionHistory, CenterOfGravityDistributionY, CenterOfGravityDistributionX};
use std::ops::Deref;
use lib_gesture::value_objects::GestureType;
use num_traits::FromPrimitive;

const ASCII_NEW_LINE: u8 = 10;

/// This function calculates the currently selected features that are used by the decision tree and decision forest.
fn calculate_features(gesture: &Gesture) -> Vec<i32> {
    /*
    let mut args: Vec<f32> = Vec::with_capacity(33);
    let darkness_dist_geom = DarknessDistribution6XYGeom::calculate(gesture);
    let brightness_dist_geom = BrightnessDistribution6XYGeom::calculate(gesture);
    let motion_history = MotionHistory::calculate(gesture);

    args.append(&mut darkness_dist_geom.deref().iter().map(|val| *val as f32).collect());
    args.append(&mut brightness_dist_geom.deref().iter().map(|val| *val as f32).collect());
    args.append(&mut motion_history.deref().iter().map(|val| *val as f32).collect());
     */

    /*
    let mut args: Vec<f32> = Vec::with_capacity(12);
    let center_of_gravity_x = CenterOfGravityDistributionFloatX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionFloatY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());
     */
    let mut args: Vec<i32> = Vec::with_capacity(12);
    let center_of_gravity_x = CenterOfGravityDistributionX::calculate(&gesture);
    let center_of_gravity_y = CenterOfGravityDistributionY::calculate(&gesture);
    args.append(&mut center_of_gravity_x.deref().to_vec());
    args.append(&mut center_of_gravity_y.deref().to_vec());
    args
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
                            let args = calculate_features(&gesture);
                            let decision_tree = Command::new("./decision_forest")
                                .args(&args.into_iter().map(|value| value.to_string()).collect::<Vec<String>>())
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

    use lib_data_set::data_sets::eva::{EVA_16PIXEL, EVA_9PIXEL};
    use lib_data_set::data_sets::klisch::{KLISCH_DATA, KLISCH_TEST};
    use lib_data_set::data_sets::kubik::{KUBIK_TEST, KUBIK_TRAINING};
    use lib_data_set::data_sets::venzke::VENZKE_TRAINING;
    use lib_data_set::entities::DataSetEntry;
    use lib_data_set::value_objects::{DataSetName, ParsingMethod};
    use lib_evaluation::entities::Evaluation;
    use lib_evaluation::value_objects::EvaluationEntryKey;
    use crate::calculate_features;

    fn evaluate_data_set(data_set: &[DataSetEntry], data_set_name: DataSetName, program: &str) {
        let mut evaluation = Evaluation::new(data_set_name);
        for data_set_entry in data_set.iter() {
            let evaluation_entry_key = EvaluationEntryKey::new(*data_set_entry.covering_object(), *data_set_entry.camera_distance(),
                                                               *data_set_entry.brightness_level(), *data_set_entry.additional_specification());
            for gesture in data_set_entry.gestures() {
                let args = calculate_features(&gesture);
                let decision_tree = Command::new(&format!("./../{}", program))
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
}
