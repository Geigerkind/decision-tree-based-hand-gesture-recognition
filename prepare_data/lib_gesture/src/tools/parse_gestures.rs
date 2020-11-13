use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::entities::{Frame, Gesture};
use crate::value_objects::GestureType;
use std::cmp::Ordering;

pub fn parse_gestures(path: &String) -> io::Result<Vec<Gesture>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::with_capacity(1500);
    let mut current_gesture = Gesture::default();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(frame) = Frame::from_str(&line) {
                match current_gesture.gesture_type.cmp(&frame.gesture_type) {
                    Ordering::Equal => {
                        if current_gesture.gesture_type == GestureType::None {
                            continue
                        }
                        current_gesture.frames.push(frame);
                    }
                    _ => {
                        if current_gesture.gesture_type == GestureType::None {
                            current_gesture.gesture_type = frame.gesture_type;
                            current_gesture.frames.push(frame);
                        } else {
                            let mut new_gesture = Gesture::default();
                            std::mem::swap(&mut current_gesture, &mut new_gesture);
                            result.push(new_gesture);
                        }
                    }
                }
            }
        }
    }

    if current_gesture.gesture_type != GestureType::None {
        result.push(current_gesture);
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::tools::parse_gestures;
    use crate::value_objects::GestureType;

    #[test]
    fn test_parse_gestures_none() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_none"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert!(gestures.is_empty());
    }

    #[test]
    fn test_parse_gestures_left_to_right() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_left_to_right"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert_eq!(gestures.len(), 1);
        let gesture = gestures.get(0).unwrap();
        assert_eq!(gesture.frames.len(), 10);
        assert_eq!(gesture.gesture_type, GestureType::LeftToRight);
    }

    #[test]
    fn test_parse_gestures_right_to_left() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_right_to_left"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert_eq!(gestures.len(), 1);
        let gesture = gestures.get(0).unwrap();
        assert_eq!(gesture.frames.len(), 8);
        assert_eq!(gesture.gesture_type, GestureType::RightToLeft);
    }

    #[test]
    fn test_parse_gestures_top_to_bottom() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_top_to_bottom"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert_eq!(gestures.len(), 1);
        let gesture = gestures.get(0).unwrap();
        assert_eq!(gesture.frames.len(), 14);
        assert_eq!(gesture.gesture_type, GestureType::TopToBottom);
    }

    #[test]
    fn test_parse_gestures_bottom_to_top() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_bottom_to_top"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert_eq!(gestures.len(), 1);
        let gesture = gestures.get(0).unwrap();
        assert_eq!(gesture.frames.len(), 13);
        assert_eq!(gesture.gesture_type, GestureType::BottomToTop);
    }

    #[test]
    fn test_parse_gestures_note_gesture() {
        // Act
        let gestures = parse_gestures(&String::from("test/parse_not_gesture"));

        // Assert
        assert!(gestures.is_ok());
        let gestures = gestures.unwrap();
        assert_eq!(gestures.len(), 1);
        let gesture = gestures.get(0).unwrap();
        assert_eq!(gesture.frames.len(), 4);
        assert_eq!(gesture.gesture_type, GestureType::NotGesture);
    }

}