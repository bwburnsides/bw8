use std::collections::HashMap;

use crate::*;

pub struct Trace {
    executed: HashMap<Instruction, usize>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            executed: HashMap::new(),
        }
    }

    pub fn add(&mut self, inst: &Instruction) {
        *self.executed.entry(*inst).or_insert(0) += 1;
    }

    pub fn total_execution_count(&self) -> usize {
        let mut count = 0;
        for pair in self.executed.iter() {
            count += pair.1;
        }
        count
    }
}

impl IntoIterator for Trace {
    type Item = (Instruction, usize);

    type IntoIter = <HashMap<Instruction, usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.executed.into_iter()
    }
}
