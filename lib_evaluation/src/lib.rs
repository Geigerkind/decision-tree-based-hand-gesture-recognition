/*!
This library provides a utility structure called `Evaluation`. It contains `EvaluationEntry` and uses the value objects defined in `lib_data_set` to
construct an evaluation that can be as detailed as it can be and as undetailed as just printing out the accuracy for a whole data set.
*/

extern crate lib_data_set;
#[macro_use]
extern crate derive_getters;

pub mod entities;
pub mod value_objects;