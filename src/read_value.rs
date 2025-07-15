use std::{
    io ::{ Read, Result },
    mem::MaybeUninit };
use crate::AsBytesMutExt;

pub trait ReadValue< T > {
    fn read_value(&mut self) -> Result< T >;
}

impl< R, T > ReadValue< T > for R
where
    R: Read,
    T: AsBytesMutExt
{
    fn read_value(&mut self) -> Result< T > {
        let mut value = unsafe { MaybeUninit::< T >::uninit().assume_init() };
        let     slice = value.as_bytes_mut();
        self.read_exact(slice).map(|_| value)
    }
}
