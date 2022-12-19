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

## benchmark

* multithread read write 
    - xline  
        - time:   [53.497 ms 55.282 ms 57.193 ms]
        - Found 4 outliers among 100 measurements (4.00%)
            - 4 (4.00%) high mild
    - crossbeam 
        - time:   [36.894 ms 37.705 ms 38.582 ms]
        - Found 8 outliers among 100 measurements (8.00%)
            - 7 (7.00%) high mild
            - 1 (1.00%) high severe
* read only 
    - xline         
        - time:   [6.9177 ms 6.9497 ms 6.9824 ms]
        - Found 3 outliers among 100 measurements (3.00%)
            - 3 (3.00%) high mild
    - crossbeam     
        - time:   [6.7929 ms 6.8322 ms 6.8711 ms]
        - Found 1 outliers among 100 measurements (1.00%)
            - 1 (1.00%) low mild
* read write 
    - xline        
        - time:   [53.724 ms 54.904 ms 56.136 ms]
        - Found 5 outliers among 100 measurements (5.00%)
            - 5 (5.00%) high mild
    - crossbeam    
        - time:   [40.920 ms 41.554 ms 42.201 ms]

