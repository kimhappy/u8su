use std::io::{ Write, Result };
use crate::AsU8Slice;

pub trait WriteValue< T > {
    fn write_value(&mut self, value: &T) -> Result< () >;
}

impl< W, T > WriteValue< T > for W
where
    W: Write,
    T: AsU8Slice
{
    fn write_value(&mut self, value: &T) -> Result< () > {
        let slice = value.as_u8_slice();
        self.write_all(slice)
    }
}
