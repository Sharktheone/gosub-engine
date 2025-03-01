
pub trait GarbageCollectable: 'static {
    //TODO: a visit method (probably)
}


macro_rules! impl_gc {
    ($ty:ty) => {
        impl GarbageCollectable for $ty {}
    };
}

impl_gc!(());
impl_gc!(u8);
impl_gc!(u16);
impl_gc!(u32);
impl_gc!(u64);
impl_gc!(u128);
impl_gc!(usize);
impl_gc!(i8);
impl_gc!(i16);
impl_gc!(i32);
impl_gc!(i64);
impl_gc!(i128);
impl_gc!(isize);
impl_gc!(f32);
impl_gc!(f64);
impl_gc!(bool);
impl_gc!(char);
impl_gc!(String);