// Problem: https://leetcode.com/problems/time-based-key-value-store/

use std::collections::HashMap;

pub struct TimeMap {
    pub map: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value, timestamp));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        let some_values = self.map.get(&key);
        if some_values.is_none() {
            return "".to_string();
        }
        let values = some_values.unwrap();

        match values.binary_search_by_key(&timestamp, |&(_, time)| time) {
            Ok(idx) => values[idx].0.clone(),
            Err(idx) if idx > 0 => values[idx - 1].0.clone(),
            _ => "".to_string(),
        }
    }
}
impl Default for TimeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let mut map = TimeMap::new();
        map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(map.get("foo".to_string(), 1), "bar");
        assert_eq!(map.get("foo".to_string(), 3), "bar");
        map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(map.get("foo".to_string(), 4), "bar2");
        assert_eq!(map.get("foo".to_string(), 5), "bar2");
    }

    #[test]
    fn test_different_get_time() {
        let mut map = TimeMap::new();
        map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(map.get("foo".to_string(), 1), "bar");
        assert_eq!(map.get("foo".to_string(), 0), "");
        map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(map.get("foo".to_string(), 4), "bar2");
        assert_eq!(map.get("foo".to_string(), 2), "bar");
    }
}
