use std::collections::HashMap;
use std::iter::FromIterator;

pub trait Rule<Value: Copy> {
    fn neighborhood(&self, index: usize) -> Vec<Option<usize>>;
    fn value(&self, index: usize, neighbor_values: &[Option<Value>]) -> Value;

    fn successor(&self, state: &[Value]) -> Vec<Value> {
        Vec::from_iter((0..state.len()).map(|i| {
            self.value(i, &Vec::from_iter(self.neighborhood(i).iter().map(|parent| {
                if let &Some(p) = parent {
                    if let Some(&p) = state.get(p) {
                        Some(p)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })))
        }))
    }
}

pub struct ElementaryCellularAutomaton {
    rule_no: u8,
    size: usize,
    wrap: bool,
    default: bool,
}

impl Rule<bool> for ElementaryCellularAutomaton {
    fn neighborhood(&self, index: usize) -> Vec<Option<usize>> {
        vec![
            if index % self.size == 0 {
                None
            } else {
                Some(index - 1)
            },
            Some(index),
            if index % self.size == self.size - 1 {
                None
            } else {
                Some(index + 1)
            }
        ]
    }

    fn value(&self, index: usize, neighbor_values: &[Option<bool>]) -> bool {
        if let Some(p) = neighbor_values[0] {
            p
        } else {
            false
        }
    }
}
