use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<T: Eq + Hash + Clone, U: Clone>(keys: &[T], values: &[U]) -> HashMap<T, U> {
    keys.iter().cloned().zip(values.iter().cloned()).collect()
}
