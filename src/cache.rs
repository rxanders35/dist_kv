use core::time;
use std::fmt::Error;

pub trait Cache {
    fn set(key: Vec<u8>, value: Vec<u8>, t: time::Duration) -> Error;
    fn has(key: Vec<u8>) -> bool;
    fn get(key: Vec<u8>) -> Result<Vec<u8>, Error>;
    fn delete(key: Vec<u8>) -> Error;
}