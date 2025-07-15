use std::io::{ Write, Result };
use crate::AsBytesExt;

pub trait WriteValue< T > {
    fn write_value(&mut self, value: &T) -> Result< () >;
}

impl< W, T > WriteValue< T > for W
where
    W: Write,
    T: AsBytesExt
{
    fn write_value(&mut self, value: &T) -> Result< () > {
        let slice = value.as_bytes();
        self.write_all(slice)
    }
}
