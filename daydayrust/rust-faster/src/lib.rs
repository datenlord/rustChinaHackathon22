#![allow(unused)]
use faster_sys as ffi;
use std::sync::mpsc::{channel, Receiver, Sender};

mod traits;
mod status;
mod builder;
mod error;

pub use traits::*;
pub use builder::FasterKvBuilder;
pub use error::FasterError;

pub struct FasterKv {
    faster_t: *mut ffi::faster_t,
    storage_dir: Option<String>,
}

impl FasterKv {
    pub fn upsert<K, V>(&self, key: &K, value: &V, monotonic_serial_number: u64) -> u8
    where
        K: FasterKey,
        V: FasterValue,
    {
        let mut encoded_key = bincode::serialize(key).unwrap();
        let encoded_key_length = encoded_key.len();
        let encoded_key_ptr = encoded_key.as_mut_ptr();
        let mut encoded_value = bincode::serialize(value).unwrap();
        let encoded_value_length = encoded_value.len();
        let encoded_value_ptr = encoded_value.as_mut_ptr();
        std::mem::forget(encoded_key);
        std::mem::forget(encoded_value);
        unsafe {
            ffi::faster_upsert(
                self.faster_t,
                encoded_key_ptr,
                encoded_key_length as u64,
                encoded_value_ptr,
                encoded_value_length as u64,
                monotonic_serial_number,
            )
        }
    }


    pub fn read<K, V>(&self, key: &K, monotonic_serial_number: u64) -> (u8, Receiver<V>)
    where
        K: FasterKey,
        V: FasterValue,
    {
        let mut encoded_key = bincode::serialize(key).unwrap();
        let encoded_key_length = encoded_key.len();
        let encoded_key_ptr = encoded_key.as_mut_ptr();
        let (sender, receiver) = channel();
        let sender_ptr: *mut Sender<V> = Box::into_raw(Box::new(sender));
        std::mem::forget(encoded_key);
        let status = unsafe {
            ffi::faster_read(
                self.faster_t,
                encoded_key_ptr,
                encoded_key_length as u64,
                monotonic_serial_number,
                Some(read_callback::<V>),
                sender_ptr as *mut libc::c_void,
            )
        };
        (status, receiver)
    }

    /// Deletes a previously inserted key.
    ///
    /// Returns [NOT_FOUND](status/constant.NOT_FOUND.html) for un-inserted keys.
    ///
    /// # Example
    /// ```
    /// use faster_rs::{FasterKv, status};
    /// let store = FasterKv::default();
    ///
    /// let key = 1;
    /// let value = 42;
    ///
    /// // Insert key-value
    /// store.upsert(&key, &value, 1);
    ///
    /// // Read key-value
    /// let (res, recv) = store.read(&key, 1);
    /// assert_eq!(status::OK, res);
    /// assert_eq!(value, recv.recv().unwrap());
    ///
    /// // Delete key-value
    /// store.delete(&key, 1);
    ///
    /// // Re-read key-value and confirm deleted
    /// let (res, recv) = store.read::<i32, i32>(&key, 1);
    /// assert_eq!(status::NOT_FOUND, res);
    /// assert!(recv.recv().is_err());
    /// ```
    pub fn delete<K>(&self, key: &K, monotonic_serial_number: u64) -> u8
    where
        K: FasterKey,
    {
        let mut encoded_key = bincode::serialize(key).unwrap();
        let encoded_key_length = encoded_key.len();
        let encoded_key_ptr = encoded_key.as_mut_ptr();
        std::mem::forget(encoded_key);
        unsafe {
            ffi::faster_delete(
                self.faster_t,
                encoded_key_ptr,
                encoded_key_length as u64,
                monotonic_serial_number,
            )
        }
    }

    fn destroy(&self) -> () {
        unsafe {
            ffi::faster_destroy(self.faster_t);
        }
    }

}


impl Default for FasterKv {
    fn default() -> Self {
        FasterKvBuilder::new(1 << 15, 1024 * 1024 * 1024)
            .build()
            .unwrap()
    }
}


// In order to make sure we release the resources the C interface has allocated for the store
impl Drop for FasterKv {
    fn drop(&mut self) {
        self.destroy();
    }
}

unsafe impl Send for FasterKv {}
unsafe impl Sync for FasterKv {}
