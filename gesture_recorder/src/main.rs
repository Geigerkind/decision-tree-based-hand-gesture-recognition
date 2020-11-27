extern crate serialport;
extern crate lib_gesture;

use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use lib_gesture::entities::{Frame, GestureReader};
use std::str::FromStr;
use lib_gesture::value_objects::GestureType;

const ASCII_NEW_LINE: u8 = 10;
const DATA_DIRECTORY: &str = "data/dataDymel";

fn main() {
    println!("Data directory path: {}", DATA_DIRECTORY);
    println!("Enter file name:");
    let mut input = String::new();
    let mut reader = std::io::stdin();
    let _ = reader.read_line(&mut input);
    println!("Creating: {}/{}.csv", DATA_DIRECTORY, input.trim_end_matches("\n"));
    let file = std::fs::File::create(Path::new(&format!("{}/{}.csv", DATA_DIRECTORY, input.trim_end_matches("\n"))));
    if file.is_err() {
        panic!("Could not create file!");
    }
    let mut file = file.unwrap();

    println!("Auto mode? (y/n):");
    let mut input = String::new();
    let _ = reader.read_line(&mut input);
    let auto_mode = input.starts_with("y");

    let mut gesture_type = GestureType::None;
    if auto_mode {
        loop {
            println!("Enter start gesture type (1|2|3|4):");
            let mut input = String::new();
            let _ = reader.read_line(&mut input);
            gesture_type = match input.trim_end_matches("\n") {
                "1" => GestureType::LeftToRight,
                "2" => GestureType::RightToLeft,
                "3" => GestureType::TopToBottom,
                "4" => GestureType::BottomToTop,
                _ => continue
            };
            break;
        }
        println!("Start gesture is {:?}", gesture_type);
    }

    println!("Establishing connection to serial output...");
    // The Arduino serial sends to the /dev/ttyACM0 port.
    let mut port = serialport::posix::TTYPort::open(&Path::new("/dev/ttyACM0"), &SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    }).expect("Failed to open port");
    let mut gesture_reader = GestureReader::new(0.05, 0.01, 0.2, true);
    let mut line = Vec::with_capacity(28);
    let mut serial_buf: Vec<u8> = vec![0; 1];
    loop {
        if port.read(&mut serial_buf).is_ok() {
            line.push(serial_buf[0]);
            if serial_buf[0] == ASCII_NEW_LINE {
                if let Ok(line) = std::str::from_utf8(&line) {
                    if let Ok(frame) = Frame::from_str(line.trim_end_matches("\r\n")) {
                        if let Some(gesture) = gesture_reader.feed_frame(frame) {
                            println!("Gesture found!");
                            if !auto_mode {
                                println!("Enter arrows (Right|Left|Down|Up) and hit enter:");
                                loop {
                                    let mut input = [0; 4];
                                    let _ = reader.read_exact(&mut input);
                                    gesture_type = match &input {
                                        [27, 91, 67, 10] => GestureType::LeftToRight,
                                        [27, 91, 68, 10] => GestureType::RightToLeft,
                                        [27, 91, 66, 10] => GestureType::TopToBottom,
                                        [27, 91, 65, 10] => GestureType::BottomToTop,
                                        _ => continue
                                    };
                                    break;
                                }
                            }
                            println!("Noting gesture as {:?}", gesture_type);
                            for frame in gesture.frames {
                                let _ = file.write_all(format!("{},{}\n", frame.pixel.iter().map(i16::to_string).collect::<Vec<String>>().join(","), gesture_type as u8).as_bytes());
                            }

                            if auto_mode {
                                gesture_type = match gesture_type {
                                    GestureType::LeftToRight => GestureType::RightToLeft,
                                    GestureType::RightToLeft => GestureType::LeftToRight,
                                    GestureType::TopToBottom => GestureType::BottomToTop,
                                    GestureType::BottomToTop => GestureType::TopToBottom,
                                    _ => GestureType::None
                                };
                                println!("Next gesture must be {:?}", gesture_type);
                            }
                        }
                    }
                }
                line.clear();
            }
        }
    }
}