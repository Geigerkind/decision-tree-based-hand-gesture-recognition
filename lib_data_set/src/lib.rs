/*!
This library standardizes what a data set is and provides utility constants to automatically get a collection of the desired data set to work with.
A `DataSet` is a collection of `DataSetEntry` with different `CoveringObject`s (e.g. Hand, Finger), `CameraDistance`s, `BrightnessLevel`s and
`AdditionalSpecification`s like fast or slow. A set can be used by importing the data set and using it as a constant, e.g.
`EVA_9PIXEL.get(&ParsingMethod::ByAnnotation).unwrap()`.
*/

extern crate lib_gesture;
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate lazy_static;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod entities;
pub mod value_objects;
pub mod data_sets;