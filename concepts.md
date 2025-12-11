### Top 10 Rust Interview Questions

1. Explain Rust's ownership model. Why does Rust need it?
   - What's a move?
   - What happens when you assign a String to another variable?
   - Why does Rust not need a garbage collector?

2. What's the difference between ownership, borrowing, and referencing?
   - immutable borrow &T
   - mutable borrow &mut T
   - borrow checker rules

    Follow-up:
   - Why can't you have two mutable references at the same time?

3. What's a lifetime in Rust? why are lifetimes needed?
    Simple answer is enough:
    - They ensure references are always valid
    - 'a is a label used by the compiler
    - Most lifetimes are inferred automatically

    Follow-up:
    - When do you need to write lifetimes explicitly? (e.g. functions returning ref)

4. What's the difference between String and &str?
    Expected points:
    - ownership vs. borrowing
    - heap vs. stack + UTF-8
    - When to use each

5. Explain `Option<T>` and `Result<T, E>`. Why does Rust avoid exceptions?
    Follow-ups:
    - What's the idiomatic way to handle errors?
    - What does ? operator do?

6. What's a trait? How is it different from interfaces in other languages?
    Follow-ups:
    - What is trait bound?
    - What is the difference between `dyn Trait` and `impl Trait`?

7. How does pattern matching work in Rust?
    Expect to explain:
    - match
    - Exhaustiveness
    - Matching enums like `Option` and `Result`

8. Explain the difference between `Vec<T>` and `HashMap<K,V>` and common use cases.
    Follow-up:
    - How does Rust prevent out-of-bounds access?

9. What is the difference between `clone()` and the `Copy` trait?
    The interviewer may ask:
    - Why can i32 be copied but String is moved?

10. Explain concurrency in Rust. What problems does Rust solve compared to other languages?
    You don’t need code, just understand:
    - Rust enforces thread safety at compile time
    - Send and Sync traits
    - Data races prevented by design
    - Channels and Arc<Mutex<T>>
