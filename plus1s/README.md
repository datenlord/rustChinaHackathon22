# Ourtree

We implement the given trait by wrapping [crossbeam/skiplist](crossbeam-skiplist).

```rust
use ourtree::{SkiplistIndex, IndexOperate};

fn main() {
    let index = SkiplistIndex::new();

    index.insert_or_update(1, 1);
    assert_eq!(index.get(&1, &1), vec![&1]);
    assert_eq!(index.delete(&1, &1), vec![1]);
    assert!(index.get(&1, &1).is_empty());
}
```
run `cargo bench` when rust version >= 1.63.0 or nightly.

It can be a baseline of the future works based on this trait. But when reaching the reference of indexed values, [`ManuallyDrop`](ManuallyDrop) may be a better choice.

Since there is some overhead in the wrapping layer, someone should beat this work :)

[crossbeam-skiplist]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-skiplist
[ManuallyDrop]: https://doc.rust-lang.org/stable/std/mem/struct.ManuallyDrop.html

