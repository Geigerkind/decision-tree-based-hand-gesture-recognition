use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::entities::{Frame, Gesture, GestureReader};

pub fn parse_gestures_by_threshold(path: &String) -> io::Result<Vec<Gesture>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::with_capacity(1500);
    let mut gesture_reader = GestureReader::new(0.01, 0.01, 0.02, true);
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(frame) = Frame::from_str(&line) {
                if let Some(gesture) = gesture_reader.feed_frame(frame) {
                    result.push(gesture);
                }
            }
        }
    }

    Ok(result)
}