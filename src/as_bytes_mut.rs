use core::slice::from_raw_parts_mut;
use crate::Pod;

pub trait AsBytesMutExt {
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl< T > AsBytesMutExt for T
where
    T: Pod
{
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut _ as *mut u8;
        let len = size_of::< T >();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}

impl< T > AsBytesMutExt for [T]
where
    T: Sized,
{
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.as_ptr() as *mut u8;
        let len = size_of::< T >() * self.len();
        unsafe { from_raw_parts_mut(ptr, len) }
    }
}
