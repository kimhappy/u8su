use core::{ mem::size_of, slice::{ from_raw_parts, from_raw_parts_mut } };
use crate::Pod;

pub trait AsBytesExt {
    fn as_byte_ptr(&self) -> *const u8;
    fn num_bytes(&self) -> usize;

    fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_byte_ptr();
        let len = self.num_bytes();
        unsafe { from_raw_parts(ptr, len) }
    }

    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.as_byte_ptr() as *mut u8;
        let len = self.num_bytes();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}

impl< T > AsBytesExt for T
where
    T: Pod
{
    fn as_byte_ptr(&self) -> *const u8 {
        self.as_byte_ptr()
    }

    fn num_bytes(&self) -> usize {
        Self::num_bytes()
    }
}

impl< T > AsBytesExt for [T]
where
    T: Pod
{
    fn as_byte_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }

    fn num_bytes(&self) -> usize {
        size_of::< T >() * self.len()
    }
}
