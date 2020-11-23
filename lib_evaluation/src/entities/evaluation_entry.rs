use crate::value_objects::EvaluationEntryKey;

/// The structure holding for each evaluation entry the necessary information.
#[derive(Debug, Getters)]
pub struct EvaluationEntry {
    key: EvaluationEntryKey,
    true_positive: u32,
    false_negative: u32
}

impl EvaluationEntry {
    /// Creates a new instance.
    pub fn new(key: EvaluationEntryKey) -> Self {
        EvaluationEntry {
            key,
            true_positive: 0,
            false_negative: 0
        }
    }

    /// Increment function for true_positive
    pub fn add_true_positive(&mut self) {
        self.true_positive += 1;
    }

    /// Increment function for false_negative.
    pub fn add_false_negative(&mut self) {
        self.false_negative += 1;
    }

    /// Return the accuracy for this entry.
    /// If nothing was collected, e.g. true_positive + false_negative = 0, return None.
    /// Otherwise return a value between 0 and 1
    pub fn accuracy(&self) -> Option<f64> {
        let total = self.true_positive + self.false_negative;
        if total == 0 {
            return None;
        }
        Some((self.true_positive as f64) / (total as f64))
    }
}