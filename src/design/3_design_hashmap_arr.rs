/**
 * 706. design hashmap
 *
 * Design a HashMap without using any built-in hash table libraries.
 *
 */

const BUCKETS: usize = 1009; // prime number

#[derive(Debug)]
struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        let mut buckets = Vec::with_capacity(BUCKETS);
        for _ in 0..BUCKETS {
            buckets.push(Vec::new());
        }

        MyHashMap { buckets }
    }

    fn hash(&self, key: i32) -> usize {
        (key as usize) % BUCKETS
    }

    // value will always be non-negative
    fn put(&mut self, key: i32, value: i32) {
        let idx = self.hash(key);
        let bucket = &mut self.buckets[idx];

        for (k, v) in bucket.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }

        // otherwise, insert new pair
        bucket.push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let idx = self.hash(key);
        let bucket = &self.buckets[idx];

        for &(k, v) in bucket.iter() {
            if k == key {
                return v;
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let idx = self.hash(key);
        let bucket = &mut self.buckets[idx];

        if let Some(pos) = bucket.iter().position(|&(k, _)| k == key) {
            // swap with last then pop
            bucket.swap_remove(pos); // swap_remove is O(1)
        }
    }
}

fn main() {
    let mut hashmap = MyHashMap::new();
    hashmap.put(10, 1);
    hashmap.put(10, 2);
    hashmap.put(20, 3);
    hashmap.remove(10);
    hashmap.get(10);

    dbg!(hashmap.get(10));
}
