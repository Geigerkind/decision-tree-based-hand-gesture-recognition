use crate::entities::DataSetEntry;
use std::ops::Deref;

/// A data set is a collection of DataSetEntry.
#[derive(Debug)]
pub struct DataSet(Vec<DataSetEntry>);

impl Deref for DataSet {
    type Target = Vec<DataSetEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}