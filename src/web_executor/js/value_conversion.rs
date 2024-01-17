use crate::css3::parser_config::Context;
use crate::types::Result;
use crate::web_executor::js::{JSContext, JSValue};


//trait to easily convert Rust types to JS values (just call .to_js_value() on the type)
pub trait ValueConversion<C: JSContext> {
    type Context: JSContext<Value = C::Value>;

    fn to_js_value(&self, ctx: C) -> Result<C::Value>;
}

macro_rules! impl_value_conversion {
    (number, $type:ty) => {
        impl<C: JSContext> ValueConversion<C> for $type {
            type Context = C;

            fn to_js_value(&self, ctx: C) -> Result<C::Value> {
                C::Value::new_number(ctx, *self as f64)
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

    ($func:ident, $type:ty, deref) => {
        impl<C: JSContext> ValueConversion<C> for $type {
            type Context = C;

            fn to_js_value(&self, ctx: C) -> Result<C::Value> {
                C::Value::$func(ctx, *self)
            }
        }
    };

    ($func:ident, $type:ty) => {
        impl<C: JSContext> ValueConversion<C> for $type {
            type Context = C;

            fn to_js_value(&self, ctx: C) -> Result<C::Value> {
                C::Value::$func(ctx, self)
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

impl<C: JSContext> ValueConversion<C> for String {
    type Context = C;

    fn to_js_value(&self, ctx: C) -> Result<C::Value> {
        C::Value::new_string(ctx, self)
    }
}

impl<C: JSContext> ValueConversion<C> for () {
    type Context = C;
    fn to_js_value(&self, ctx: C) -> Result<C::Value> {
    C::Value::new_undefined(ctx)
    }
}