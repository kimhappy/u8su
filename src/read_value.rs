use std::{
    io ::{ Read, Result },
    mem::MaybeUninit };
use crate::AsU8Slice;

pub trait ReadValue< T > {
    fn read_value(&mut self) -> Result< T >;
}

impl< R, T > ReadValue< T > for R
where
    R: Read,
    T: AsU8Slice
{
    fn read_value(&mut self) -> Result< T > {
        let mut value = unsafe { MaybeUninit::< T >::uninit().assume_init() };
        let     slice = value.as_mut_u8_slice();
        self.read_exact(slice).map(|_| value)
    }
}
