/**
 * 1. create a new vec
 * 2. modify a vec
 *      - push / pop
 *      - insert / remove
 *      - truncate / clear
 *      - extend / append / concat
 *      - reverse / sort / dedup / retain / swap_remove
 *
 * 3. access elements (4)
 *      - index: v[i]
 *      - .get() / .get_mut()   returns Option<&T>
 *      - .first()  / .last()
 *      - .split_at()
 *
 *  4. search / position
 *      - v.iter().position(|x| ...)
 *      - v.iter().rposition(|x| ...)
 *      - v.iter().enumerate()
 *      - v.binary_search(&x)       v should be sorted
 *
 * 5. vec-level check
 *      - .is_empty()
 *      - .len()
 *      - .contains(&x)
 *      - .starts_with(&[...])  / .ends_with(&[...])
 *      - .all() / .any()
 *
 * 6. iterate (4)
 *
 */
fn main() {
    // 1. create
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let v3 = vec![0; 10]; // 10 zeroes
    let mut v4: Vec<i32> = Vec::with_capacity(100);
    let v5: Vec<i32> = (0..4).collect(); // [0, 1, 2, 3]

    // 2. modify
    let mut v = vec![1, 2, 3];

    // push / pop
    v.push(4);
    let last: Option<i32> = v.pop(); // Some(4) or None

    // insert / remove
    v.insert(1, 10); // [1,10,2,3]
    let removed = v.remove(2); // remove at idx, shift left

    // truncate / clear
    v.truncate(2); // keep first 2
    v.clear(); // empty vec

    // extend / append
    let mut a = vec![1, 2];
    let mut b = vec![3, 4];
    a.extend(b.iter()); // a: [1,2,3,4], b not moved
    a.append(&mut b); // a: [1,2,3,4], b: []

    // reverse / sort / dedup / retain / swap_remove
    let mut v = vec![3, 1, 2, 2];
    v.reverse(); // [2,2,1,3]
    v.sort(); // [1,2,2,3]
    v.dedup(); // [1,2,3]
    v.retain(|&x| x % 2 == 1); // keep odd -> [1,3]
    v.swap_remove(0); // remove idx 0 by swapping with last

    // 3. access
    let v = vec![10, 20, 30];

    let x = v[1]; // 20, panics if out of bounds

    if let Some(val) = v.get(5) {
        // safe access
        println!("{}", val);
    }

    if let Some(first) = v.first() {
        // &T
        println!("{}", first);
    }
    if let Some(last) = v.last() {
        println!("{}", last);
    }

    let (left, right) = v.split_at(1); // &[10], &[20,30]

    // 4. search / position
    let v = vec![5, 3, 7, 3];

    // first position
    let idx = v.iter().position(|&x| x == 3); // Some(1)

    // last position
    let last_idx = v.iter().rposition(|&x| x == 3); // Some(3)

    // enumerate
    for (i, x) in v.iter().enumerate() {
        println!("{i} -> {x}");
    }

    // binary search (requires sorted)
    let mut v = vec![1, 3, 5, 7];
    match v.binary_search(&5) {
        Ok(i) => println!("found at {i}"),
        Err(pos) => println!("insert at {pos}"),
    }

    // 5. vec-level checks
    let v = vec![1, 2, 3];

    v.is_empty(); // false
    v.len(); // 3
    v.contains(&2); // true
    v.starts_with(&[1, 2]); // true
    v.ends_with(&[2, 3]); // true

    v.iter().all(|&x| x > 0); // true
    v.iter().any(|&x| x == 2); // true

    // 6. iterate
    let mut v = vec![1, 2, 3];

    // read-only
    for x in &v {
        println!("{}", x); // &i32
    }

    // modify in place
    for x in &mut v {
        *x *= 2;
    }

    // take ownership (consuming)
    for x in v {
        println!("{}", x); // i32
    }

    // 7. transform
    let v = vec![1, 2, 3];

    // map
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

    // filter
    let nums = vec![1, 2, 3, 4, 5];
    let evens: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 0).collect();

    // slice & copy
    let v = vec![0, 1, 2, 3, 4];
    let sub: Vec<i32> = v[1..4].to_vec(); // [1,2,3]

    // windows / chunks (good for LeetCode)
    for w in v.windows(3) {
        // &[i32]
        println!("{:?}", w);
    }

    for c in v.chunks(2) {
        // &[i32]
        println!("{:?}", c);
    }

    // 2D vec
    let grid = vec![vec![0; 3]; 4]; // 4 x 3 matrix of zeros

    // 8. Vec <-> other types
    use std::collections::{HashMap, HashSet};

    // Vec -> slice
    let v = vec![1, 2, 3];
    let s: &[i32] = v.as_slice();

    // slice -> Vec
    let slice: &[i32] = &[4, 5, 6];
    let v2 = slice.to_vec();

    // Vec -> array
    let v3 = vec![1, 2, 3];
    let arr: [i32; 3] = v3.clone().try_into().unwrap();

    // Vec -> HashSet (remove duplicates)
    let v = vec![1, 2, 2, 3];
    let set: HashSet<i32> = v.into_iter().collect();

    // Vec<(K,V)> -> HashMap<K,V>
    let pairs = vec![("a", 1), ("b", 2)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
}
