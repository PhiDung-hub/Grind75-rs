// Problem: https://leetcode.com/problems/lru-cache/

use std::collections::HashMap;
#[allow(dead_code)]
struct LRUCache {
    capacity: i32,
    cache: HashMap<i32, i32>, // key-value
}

#[allow(dead_code)]
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        match self.cache.get(&key) {
            Some(&value) => {
                // update priority order here
                value
            }
            None => -1,
        }
    }

    #[allow(unused_variables)]
    pub fn put(&mut self, key: i32, value: i32) {}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let mut lru_cache = LRUCache::new(2);

        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);

        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}
