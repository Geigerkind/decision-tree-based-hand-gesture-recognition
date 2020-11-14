use crate::entities::DataSetEntry;
use std::ops::Deref;

#[derive(Debug)]
pub struct DataSet(Vec<DataSetEntry>);

impl Deref for DataSet {
    type Target = Vec<DataSetEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}