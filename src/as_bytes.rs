use core::slice::from_raw_parts;
use crate::Pod;

pub trait AsBytesExt {
    fn as_bytes(&self) -> &[u8];
}

impl< T > AsBytesExt for T
where
    T: Pod
{
    fn as_bytes(&self) -> &[u8] {
        let ptr = self as *const _ as *const u8;
        let len = size_of::< T >();
        unsafe { from_raw_parts(ptr, len) }
    }
}

impl< T > AsBytesExt for [T]
where
    T: Sized,
{
    fn as_bytes(&self) -> &[u8] {
        let ptr = self.as_ptr() as *const u8;
        let len = size_of::< T >() * self.len();
        unsafe { from_raw_parts(ptr, len) }
    }
}
