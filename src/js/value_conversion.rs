use crate::js::JSValue;


pub trait ValueConversion<V: JSValue> {

    type Value: JSValue;
    fn to_js_value(&self) -> Option<Self::Value>;
}

macro_rules! impl_value_conversion {
    (number, $type:ty) => {
        impl_value_conversion!(new_number, $type);
    };

    (string, $type:ty) => {
        impl_value_conversion!(new_string, $type);
    };

    (bool, $type:ty) => {
        impl_value_conversion!(new_bool, $type);
    };

    (array, $type:ty) => {
        impl_value_conversion!(new_array, $type);
    };

    (function, $type:ty) => {
        impl_value_conversion!(new_function, $type);
    };

    ($func:ident, $type:ty) => {
        impl<V: JSValue> ValueConversion<V> for $type {
            type Value = V;

            fn to_js_value(&self) -> Option<Self::Value> {
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


impl <V: JSValue, T> ValueConversion<V> for T {
    type Value = V;

    fn to_js_value(&self) -> Option<Self::Value> {
        Some(self.clone())
    }
}

impl<V: JSValue> ValueConversion<V> for String {
    type Value = V;

    fn to_js_value(&self) -> Option<Self::Value> {
        Some(self.clone())
    }
}





//TODO: implement this for different rust types