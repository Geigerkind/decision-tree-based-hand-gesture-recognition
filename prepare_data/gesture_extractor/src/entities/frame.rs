use std::str::FromStr;

use num_traits::FromPrimitive;

use crate::value_objects::{ExtractorFailure, GestureType};

#[derive(Debug)]
pub struct Frame {
    pub pixel: [i16; 9],
    pub gesture_type: GestureType,
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            pixel: [0; 9],
            gesture_type: GestureType::None,
        }
    }
}

impl FromStr for Frame {
    type Err = ExtractorFailure;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_end_matches('\n').split(",").collect();
        if parts.len() != 10 {
            return Err(ExtractorFailure::ParseFrame);
        }

        Ok(Frame {
            pixel: [
                parts[0].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[1].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[2].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[3].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[4].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[5].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[6].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[7].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
                parts[8].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)?,
            ],
            gesture_type: parts[9].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)
                .map(|number| FromPrimitive::from_i16(number).expect("Gesture should have been defined!"))?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::entities::Frame;
    use std::str::FromStr;
    use crate::value_objects::GestureType;

    #[test]
    fn test_from_str_success() {
        // Arrange
        let sample = "642,751,838,444,630,815,247,424,565,3";

        // Act
        let frame = Frame::from_str(sample);

        // Assert
        assert!(frame.is_ok());
        let frame = frame.unwrap();
        assert_eq!(frame.pixel, [642, 751, 838, 444, 630, 815, 247, 424, 565]);
        assert_eq!(frame.gesture_type, GestureType::TopToBottom);
    }

    #[test]
    fn test_from_str_too_short() {
        // Arrange
        let sample = "642,751,838,444,630,815,247,424,565";

        // Act
        let frame = Frame::from_str(sample);

        // Assert
        assert!(frame.is_err());
    }

    #[test]
    fn test_from_str_not_a_number() {
        // Arrange
        let sample = "642,751a,838,444,630,815,247,424,565";

        // Act
        let frame = Frame::from_str(sample);

        // Assert
        assert!(frame.is_err());
    }

    #[test]
    #[should_panic]
    fn test_from_str_invalid_gesture() {
        // Arrange
        let sample = "642,751,838,444,630,815,247,424,565,42";

        // Act
        let _frame = Frame::from_str(sample);
    }
}