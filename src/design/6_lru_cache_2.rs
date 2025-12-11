use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/**
 * 146. LRU cache with O(1) operations
 *
 * Uses doubly-linked list + HashMap
 * - HashMap: key -> Node (for O(1) lookup)
 * - Doubly-linked list: maintains LRU order (most recent at head)
 */

#[derive(Clone)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>, // Most recently used
    tail: Option<Rc<RefCell<Node>>>, // Least recently used
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    // Move node to front (mark as most recently used)
    fn move_to_front(&mut self, node: Rc<RefCell<Node>>) {
        // If already at head, do nothing
        if let Some(ref head) = self.head {
            if Rc::ptr_eq(&node, head) {
                return;
            }
        }

        // Remove node from current position
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(ref p) = prev {
            p.borrow_mut().next = next.clone();
        }
        if let Some(ref n) = next {
            n.borrow_mut().prev = prev.clone();
        }

        // If node was tail, update tail
        if let Some(ref tail) = self.tail {
            if Rc::ptr_eq(&node, tail) {
                self.tail = prev;
            }
        }

        // Insert at head
        node.borrow_mut().prev = None;
        node.borrow_mut().next = self.head.clone();

        if let Some(ref old_head) = self.head {
            old_head.borrow_mut().prev = Some(node.clone());
        }

        self.head = Some(node.clone());

        // If list was empty, node is also tail
        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    // Add new node to front
    fn add_to_front(&mut self, node: Rc<RefCell<Node>>) {
        node.borrow_mut().prev = None;
        node.borrow_mut().next = self.head.clone();

        if let Some(ref old_head) = self.head {
            old_head.borrow_mut().prev = Some(node.clone());
        }

        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    // Remove tail (LRU item)
    fn remove_tail(&mut self) -> Option<i32> {
        self.tail.take().map(|tail_node| {
            let key = tail_node.borrow().key;

            if let Some(ref prev) = tail_node.borrow().prev {
                prev.borrow_mut().next = None;
                self.tail = Some(prev.clone());
            } else {
                // List becomes empty
                self.head = None;
                self.tail = None;
            }

            key
        })
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key).cloned() {
            let value = node.borrow().value;
            self.move_to_front(node);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // If key exists, update value and move to front
        if let Some(node) = self.cache.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.move_to_front(node);
            return;
        }

        // Create new node
        let new_node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));

        // If at capacity, remove LRU
        if self.cache.len() >= self.capacity {
            if let Some(lru_key) = self.remove_tail() {
                self.cache.remove(&lru_key);
            }
        }

        // Add new node to front
        self.add_to_front(new_node.clone());
        self.cache.insert(key, new_node);
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
        assert_eq!(cache.get(2), 2);
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
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn test_capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }
}
