use std::collections::HashMap;

/**
 * 146. LRU cache
 *
 * LRU = Least Recently Used
 * - get(key): Return value if exists, -1 otherwise. Mark as recently used.
 * - put(key, value): Insert/update. If at capacity, remove least recently used.
 */

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, (i32, usize)>, // key -> (value, timestamp)
    timestamp: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            timestamp: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&(value, _)) = self.cache.get(&key) {
            // Update timestamp (mark as recently used)
            self.timestamp += 1;
            self.cache.insert(key, (value, self.timestamp));
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.timestamp += 1;

        // If key exists, just update it
        if self.cache.contains_key(&key) {
            self.cache.insert(key, (value, self.timestamp));
            return;
        }

        // If at capacity, remove LRU item
        if self.cache.len() >= self.capacity {
            // Find key with smallest timestamp
            let lru_key = self
                .cache
                .iter()
                .min_by_key(|(_, &(_, ts))| ts)
                .map(|(&k, _)| k)
                .unwrap();
            self.cache.remove(&lru_key);
        }

        self.cache.insert(key, (value, self.timestamp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // Evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // Evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_update_existing() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10); // Update value
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn test_get_updates_recency() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.get(1); // Make 1 recently used
        cache.put(3, 3); // Should evict 2, not 1
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), -1);
    }
}
