use std::collections::HashMap;

use lib_data_set::value_objects::DataSetName;

use crate::entities::EvaluationEntry;
use crate::value_objects::EvaluationEntryKey;

#[derive(Debug, Getters)]
pub struct Evaluation {
    data_set_name: DataSetName,
    entries: HashMap<EvaluationEntryKey, EvaluationEntry>,
}

impl Evaluation {
    pub fn new(data_set_name: DataSetName) -> Self {
        Evaluation {
            data_set_name,
            entries: Default::default(),
        }
    }

    pub fn add_true_positive(&mut self, key: EvaluationEntryKey) {
        let entry = self.entries.entry(key).or_insert_with(|| EvaluationEntry::new(key));
        entry.add_true_positive();
    }

    pub fn add_false_negative(&mut self, key: EvaluationEntryKey) {
        let entry = self.entries.entry(key).or_insert_with(|| EvaluationEntry::new(key));
        entry.add_false_negative();
    }

    pub fn print_results(&self) {
        println!("Results for {:?}", self.data_set_name);
        for (key, entry) in self.entries.iter() {
            println!("{:?}", key);
            println!("Accuracy: {:?}", entry.accuracy());
        }
    }
}

