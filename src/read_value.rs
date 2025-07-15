use std::{
    io ::{ Read, Result },
    mem::MaybeUninit };
use crate::AsBytesExt;

pub unsafe fn uninitialized_vec< T >(len: usize) -> Vec< T > {
    let mut vec = Vec::with_capacity(len);
    unsafe { vec.set_len(len) };
    vec
}

pub trait ReadValueExt: Read {
    fn read_value_to< T >(
        &mut self,
        value: &mut T
    ) -> Result< () >
    where
        T: AsBytesExt + ?Sized
    {
        let slice = unsafe { value.as_bytes_mut() };
        self.read_exact(slice)
    }

    fn read_value< T >(&mut self) -> Result< T >
    where
        T: AsBytesExt
    {
        let mut value = unsafe { MaybeUninit::< T >::uninit().assume_init() };
        self.read_value_to(&mut value).map(|_| value)
    }

    fn read_values< T >(
        &mut self,
        n: usize
    ) -> Result< Vec< T > >
    where
        [T]: AsBytesExt
    {
        let mut values = unsafe { uninitialized_vec::< T >(n) };
        self.read_value_to(&mut values[..]).map(|_| values)
    }
}
