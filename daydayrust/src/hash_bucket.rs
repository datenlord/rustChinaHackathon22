use std::sync::atomic::AtomicU64;

use crate::consts::NUMENTRIES;

/// Entry stored in a hash bucket. Packed into 8 bytes.
/// Each hash bucket entry consists of three parts: 
/// a tag (15 bits), a tentative bit, and the address (48 bits).
// +-+---------------+------------------------------------------------+
// |1|    15 bits    |                   48 bits                      |
// +|+-----|---------+---------------------|--------------------------+
//  |     tag                            address
// tentative
#[repr(C)]
pub(crate) struct AtomicHashBucketEntry {
    inner: AtomicU64
}

/// Atomic hash-bucket overflow entry.
#[repr(C)]
pub(crate) struct AtomicHashBucketOverflowEntry {

}

/// A bucket consisting of 7 hash bucket entries, plus one hash bucket overflow entry. 
/// Fits in a cache line. (We assume a 64-bit machine with 64-byte cache lines)
#[repr(C, align(64))]
pub(crate) struct HashBucket {
    /// The entries.
    pub entries: [AtomicHashBucketEntry;NUMENTRIES],
    // Overflow entry points to next overflow bucket, if any.
    pub overflow_entry: AtomicHashBucketOverflowEntry,
}