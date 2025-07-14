use core::{
    mem  ::size_of,
    slice::{ from_raw_parts, from_raw_parts_mut } };

pub trait AsU8Slice {
    fn as_u8_slice    (&    self) -> &    [u8];
    fn as_mut_u8_slice(&mut self) -> &mut [u8];
}

impl< T > AsU8Slice for T
where
    T: Sized,
{
    fn as_u8_slice(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = size_of::< T >();
        unsafe { from_raw_parts(ptr, len) }
    }

    fn as_mut_u8_slice(&mut self) -> &mut [u8] {
        let ptr = self as *mut _ as *mut u8;
        let len = size_of::< T >();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}

impl< T > AsU8Slice for [T]
where
    T: Sized,
{
    fn as_u8_slice(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;
        let len = size_of::< T >() * self.len();
        unsafe { from_raw_parts(ptr, len) }
    }

    fn as_mut_u8_slice(&mut self) -> &mut [u8] {
        let ptr = self.as_ptr() as *mut u8;
        let len = size_of::< T >() * self.len();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}
