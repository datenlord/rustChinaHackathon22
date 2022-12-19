# Rust Hackathon Case Study: Design of Efficient Concurrent Index Data Structure

The current implementation of the index is [Lock + Btree](https://github.com/datenlord/Xline/blob/master/xline/src/storage/index.rs). However, this has some repercussions(inhibiting scalability, allowing for deadlocks, etc.). The first thing that came to my mind was to use a [reader-writer lock](https://docs.rs/parking_lot/latest/parking_lot/type.RwLock.html) instead. But in scenarios with high write frequency, I hope to find a way that allows concurrent writes to progress without mutual exclusion (use interior mutability). That is where the [crossbeam_skiplist](https://docs.rs/crossbeam-skiplist/latest/crossbeam_skiplist/) comes in.

## There's Something About Skip List

> *Skip lists are a data structure that can be used in place of balanced trees. Skip lists use probabilistic balancing rather than strictly enforced balancing and as a result the algorithms for insertion and deletion in skip lists are much simpler and significantly faster than equivalent algorithms for balanced trees.*  
> *–William Pugh*

If that was a lot to take in, don't worry. It's not that important. If you really want to get it, [Open Data Structures](http://opendatastructures.org/ods-python/4_1_Basic_Structure.html) has a way more detailed description, code, and diagrams.

## Faster
Note that every insertion in skiplist triggers an allocation. Allocations are generally regarded as a slow thing to do, so that's something we'd like to avoid if possible!
[Faster](https://www.microsoft.com/en-us/research/uploads/prod/2018/03/faster-sigmod18.pdf), on the other hand, is like "lets put some arrays in there; computers love arrays". The sad thing is, I don't have time to implement it in pure rust. To add insult to injury, I have trouble [generating bindings to C++](https://rust-lang.github.io/rust-bindgen/cpp.html). I can only port an older version, but its API is not compatible with the interface we require.

## Okay But Seriously the Implementation

`TODO`