use std::io::{ Write, Result };
use crate::AsBytesExt;

pub trait WriteValueExt: Write {
    fn write_value< T >(
        &mut self,
        value: &T
    ) -> Result< () >
    where
        T: AsBytesExt + ?Sized
    {
        self.write_all(value.as_bytes())
    }
}

impl< T > WriteValueExt for T
where
    T: Write
{}
