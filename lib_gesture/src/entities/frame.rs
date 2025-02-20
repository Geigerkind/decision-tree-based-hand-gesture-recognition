use std::str::FromStr;

use num_traits::FromPrimitive;

use crate::value_objects::{ExtractorFailure, GestureType};

/// Each frame in the data sets contain a value for each pixel and its gesture type.
#[derive(Debug, Clone)]
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

    /// Parses a frame of the form p20,p21,p22,p10,p11,p12,p00,p01,p02(,gesture_type).
    /// If the gesture type is not given, it is categorized as "NonLabeled".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_end_matches('\n').split(",").collect();
        if parts.len() < 9 {
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
            gesture_type: if parts.len() == 9 { GestureType::NotLabeled } else { parts[9].parse::<i16>().map_err(|_| ExtractorFailure::ParseFrame)
                .map(|number| FromPrimitive::from_i16(number).expect("Gesture should have been defined!"))? },
        })
    }
}

impl Frame {
    /// Helper function to calculate the mean of all pixel.
    /// Note: the original implementation normalized the weights between 0 and 1, this implementation does not
    pub fn mean(&self) -> f64 {
        self.pixel.iter().map(|value| (*value as f64) / 1024.0).sum::<f64>() / 9.0
    }

    /// Checks if there is any pixel, whose difference to the other surpasses the threshold
    /// Note: the original implementation normalized the weights between 0 and 1, this implementation does not
    pub fn any_pixel_difference_higher_than_threshold(&self, other: &Self, threshold: f64) -> bool {
        self.pixel.iter().enumerate().any(|(index, value)| (((*value as f64) / 1024.0) - ((other.pixel[index] as f64) / 1024.0)).abs() > threshold)
    }

    /// Helper function to print the frame.
    pub fn print(&self) {
        println!("{:?}", self.pixel);
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
        let sample = "642,751,838,444,630,815,247,424";

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