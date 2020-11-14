use crate::value_objects::EvaluationEntryKey;

#[derive(Debug)]
pub struct EvaluationEntry {
    key: EvaluationEntryKey,
    true_positive: u32,
    false_negative: u32
}

impl EvaluationEntry {
    pub fn new(key: EvaluationEntryKey) -> Self {
        EvaluationEntry {
            key,
            true_positive: 0,
            false_negative: 0
        }
    }

    pub fn key(&self) -> &EvaluationEntryKey {
        &self.key
    }

    pub fn add_true_positive(&mut self) {
        self.true_positive += 1;
    }

    pub fn add_false_negative(&mut self) {
        self.false_negative += 1;
    }

    pub fn accuracy(&self) -> Option<f64> {
        let total = self.true_positive + self.false_negative;
        if total == 0 {
            return None;
        }
        Some((self.true_positive as f64) / (total as f64))
    }
}