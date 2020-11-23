use std::collections::HashMap;

use lib_data_set::value_objects::DataSetName;

use crate::entities::EvaluationEntry;
use crate::value_objects::EvaluationEntryKey;

/// The main structure to hold all necessary information for an evaluation of a data set.
#[derive(Debug, Getters)]
pub struct Evaluation {
    data_set_name: DataSetName,
    entries: HashMap<EvaluationEntryKey, EvaluationEntry>,
}

impl Evaluation {
    /// Creates a new instance.
    pub fn new(data_set_name: DataSetName) -> Self {
        Evaluation {
            data_set_name,
            entries: Default::default(),
        }
    }

    /// Calculates the accuracy over all entries
    pub fn accuracy(&self) -> Option<f64> {
        let total = self.entries.iter().fold(0, |acc, (_, entry)| acc + entry.true_positive() + entry.false_negative());
        let true_positive = self.entries.iter().fold(0, |acc, (_, entry)| acc + entry.true_positive());
        if total == 0 {
            return None;
        }
        return Some((true_positive as f64) / (total as f64));
    }

    /// When evaluating, add a true positive find to the evaluation
    pub fn add_true_positive(&mut self, key: EvaluationEntryKey) {
        let entry = self.entries.entry(key).or_insert_with(|| EvaluationEntry::new(key));
        entry.add_true_positive();
    }

    /// When evaluating, add a false negative find to the evaluation
    pub fn add_false_negative(&mut self, key: EvaluationEntryKey) {
        let entry = self.entries.entry(key).or_insert_with(|| EvaluationEntry::new(key));
        entry.add_false_negative();
    }

    /// Helper function to print results
    pub fn print_results(&self) {
        println!("Results for {:?}", self.data_set_name);
        for (key, entry) in self.entries.iter() {
            println!("{:?}", key);
            println!("Accuracy: {:?}", entry.accuracy());
        }
        println!("Total accuracy: {:?}", self.accuracy());
    }
}

