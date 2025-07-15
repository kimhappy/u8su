use core::num::{ NonZero, ZeroablePrimitive };

pub trait Pod: Sized {
    fn as_byte_ptr(&self) -> *const u8 {
        self as *const _ as *const u8
    }

    fn num_bytes() -> usize {
        size_of::< Self >()
    }
}

impl Pod for u8    {}
impl Pod for u16   {}
impl Pod for u32   {}
impl Pod for u64   {}
impl Pod for u128  {}
impl Pod for usize {}
impl Pod for i8    {}
impl Pod for i16   {}
impl Pod for i32   {}
impl Pod for i64   {}
impl Pod for i128  {}
impl Pod for isize {}
impl Pod for f32   {}
impl Pod for f64   {}

impl< T > Pod for Option< NonZero< T > >
where
    T: ZeroablePrimitive
{}

impl< T, const N: usize > Pod for [T; N]
where
    T: Pod
{}
