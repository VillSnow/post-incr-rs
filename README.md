# post-incr-rs

Just add traits for post increment and decrement.


```rust
use post_incr::PostIncr as _;
use post_incr::PostDecr as _;

let mut x: i32 = 0;
assert_eq!(x.post_incr(), 0);
assert_eq!(x.post_incr(), 1);
assert_eq!(x.post_incr(), 2);

assert_eq!(x.post_decr(), 3);
assert_eq!(x.post_decr(), 2);
assert_eq!(x.post_decr(), 1);
```
