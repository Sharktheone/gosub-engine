use crate::js::JSValue;
use crate::types::Result;

pub trait ValueConversion<V: JSValue> {
    type Value: JSValue;
    fn to_js_value(&self) -> Result<Self::Value>;
}

macro_rules! impl_value_conversion {
    (number, $type:ty) => {
        impl<V: JSValue> ValueConversion<V> for $type {
            type Value = V;

            fn to_js_value(&self) -> Result<Self::Value> {
                Self::Value::new_number(*self as f64)
            }
        }
    };

    (string, $type:ty) => {
        impl_value_conversion!(new_string, $type, deref);
    };

    (bool, $type:ty) => {
        impl_value_conversion!(new_bool, $type, deref);
    };

    (array, $type:ty) => {
        impl_value_conversion!(new_array, $type, deref);
    };

    (function, $type:ty) => {
        impl_value_conversion!(new_function, $type);
    };

    ($func:ident, $type:ty, deref) => {
        impl<V: JSValue> ValueConversion<V> for $type {
            type Value = V;

            fn to_js_value(&self) -> Result<Self::Value> {
                Self::Value::$func(*self)
            }
        }
    };

    ($func:ident, $type:ty) => {
        impl<V: JSValue> ValueConversion<V> for $type {
            type Value = V;

            fn to_js_value(&self) -> Result<Self::Value> {
                Self::Value::$func(self)
            }
        }
    };
}

impl_value_conversion!(number, i8);
impl_value_conversion!(number, i16);
impl_value_conversion!(number, i32);
impl_value_conversion!(number, i64);
impl_value_conversion!(number, isize);
impl_value_conversion!(number, u8);
impl_value_conversion!(number, u16);
impl_value_conversion!(number, u32);
impl_value_conversion!(number, u64);
impl_value_conversion!(number, usize);
impl_value_conversion!(number, f32);
impl_value_conversion!(number, f64);

impl_value_conversion!(string, &str);

impl_value_conversion!(bool, bool);

impl_value_conversion!(function, fn());

impl<V: JSValue> ValueConversion<V> for String {
    type Value = V;

    fn to_js_value(&self) -> Result<Self::Value> {
        Self::Value::new_string(self)
    }
}

//TODO: implement this for different rust types
