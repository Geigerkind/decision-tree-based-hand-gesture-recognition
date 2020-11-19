/*!
Gesture (candidates) on a 3x3 light sensor array were collected by people before this work. These gestures include LeftToRight,
RightToLeft, TopToBottom and BottomToTop movements and can be found in the `data` directory. A line in such a file contains between 9 and 27 values.
A detailed description of these values can be obtained from Dr. Venzke. The library parses the first 9 as the 3x3 light sensors, and the 10th value
as gesture. It defines what is a `GestureType`, `Frame` and `Gesture`(Candidate). It provides one method to parse a stream of frames
to a gesture (candidate) by the threshold `(GestureReader)`, one method to parse a file by threshold `parse_gestures_by_threshold`, and one
method to parse a file by annotation `parse_gestures_by_annotation`.
*/

#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod entities;
pub mod tools;
pub mod value_objects;