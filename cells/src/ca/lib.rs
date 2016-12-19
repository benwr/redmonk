use std::collections::HashMap;

pub trait Cell<Value> {
    fn neighborhood() -> Vector<usize>;
    fn value(neighborhood_values: &Vector<Value>) -> Value;
}
