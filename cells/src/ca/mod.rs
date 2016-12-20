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
    pub rule: u8,
    pub size: usize,
    pub wrap: bool,
    pub default: bool,
}

impl Rule<bool> for ElementaryCellularAutomaton {
    fn neighborhood(&self, index: usize) -> Vec<Option<usize>> {
        vec![
            if index <= 0 {
                if self.wrap {
                    Some(self.size - 1)
                } else {
                    None
                }
            } else {
                Some(index - 1)
            },
            Some(index),
            if index >= self.size - 1 {
                if self.wrap {
                    Some(0)
                } else {
                    None
                }
            } else {
                Some(index + 1)
            }
        ]
    }

    fn value(&self, index: usize, neighbor_values: &[Option<bool>]) -> bool {
        let mut rule_index = 0u8;
        for i in 0..3 {
            let val = match neighbor_values.get(i) {
                None => self.default,
                Some(&None) => self.default,
                Some(&Some(n)) => n
            };
            rule_index <<= 1;
            rule_index |= val as u8;
        };
        ((self.rule >> rule_index) & 1) == 1
    }
}
