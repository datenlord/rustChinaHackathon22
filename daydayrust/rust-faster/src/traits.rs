use serde::{
    de::DeserializeOwned,
    Serialize
};
use std::sync::mpsc::Sender;

use crate::status;

pub trait FasterKey: DeserializeOwned + Serialize {}

pub trait FasterValue: DeserializeOwned + Serialize {}


#[inline(always)]
pub unsafe extern "C" fn read_callback<T>(
    sender: *mut libc::c_void,
    value: *const u8,
    length: u64,
    status: u32,
) where
    T: DeserializeOwned,
{
    let boxed_sender = Box::from_raw(sender as *mut Sender<T>);
    let sender = *boxed_sender;
    if status == status::OK.into() {
        let val = bincode::deserialize(std::slice::from_raw_parts(value, length as usize)).unwrap();
        // TODO: log error
        let _ = sender.send(val);
    }
}
