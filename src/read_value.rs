use std::{
    io ::{ Read, Result },
    mem::MaybeUninit };
use crate::AsBytesMutExt;

pub trait ReadValueExt: Read {
    fn read_value< T >(&mut self) -> Result< T >
    where
        T: AsBytesMutExt
    {
        let mut value = unsafe { MaybeUninit::< T >::uninit().assume_init() };
        let     slice = unsafe { value.as_bytes_mut() };
        self.read_exact(slice).map(|_| value)
    }
}
