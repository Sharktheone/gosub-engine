use alloc::rc::Rc;
use std::cell::RefCell;
use crate::js::v8::{Ctx, V8Array, V8Context, V8Object};
use crate::js::{JSError, JSType, JSValue, ValueConversion};
use crate::types::Error;
use v8::{Local, Value};

pub struct V8Value<'a>{
    context: Ctx<'a>,
    value: Local<'a, Value>
}


macro_rules! impl_is {
    ($name:ident) => {
        fn $name(&self) -> bool {
            self.value.$name()
        }
    };
}


impl<'a> JSValue for V8Value<'a> {
    type Object = V8Object<'a>;
    type Array = V8Array<'a>;

    type Context = Ctx<'a>;

    fn as_string(&self) -> crate::types::Result<String> {
        Ok(self.value.to_rust_string_lossy(self.context.borrow_mut().scope()))
    }

    fn as_number(&self) -> crate::types::Result<f64> {
        if let Some(value) = self.value.number_value(self.context.borrow_mut().scope()) {
            Ok(value)
        } else {
            Err(Error::JS(JSError::Conversion(
                "could not convert to number".to_owned(),
            )))
        }
    }

    fn as_bool(&self) -> crate::types::Result<bool> {
        Ok(self.value.boolean_value(self.context.borrow_mut().scope()))
    }

    fn as_object(&self) -> crate::types::Result<Self::Object> {
        if let Some(value) = self.value.to_object(self.context.borrow_mut().scope()) {
            Ok(V8Object::from(value))
        } else {
            Err(Error::JS(JSError::Conversion(
                "could not convert to number".to_owned(),
            )))
        }
    }

    fn as_array(&self) -> crate::types::Result<Self::Array> {
        todo!()
    }

    impl_is!(is_string);
    impl_is!(is_number);
    impl_is!(is_object);
    impl_is!(is_array);
    impl_is!(is_null);
    impl_is!(is_undefined);
    impl_is!(is_function);

    fn is_bool(&self) -> bool {
        self.value.is_boolean()
    }


    fn type_of(&self) -> JSType {
        todo!()
    }

    // fn new_object() -> crate::types::Result<Self::Object> {
    //     todo!()
    // }
    //
    // fn new_array<T: ValueConversion<Self>>(value: &[T]) -> crate::types::Result<Self::Array> {
    //     todo!()
    // }

    fn new_string(ctx: Self::Context, value: &str) -> crate::types::Result<Self> {
        if let Some(value) = v8::String::new(ctx.borrow_mut().scope(), value) {
            Ok(Self {
                context: Rc::clone(&ctx),
                value: Local::from(value),
            })
        } else {
            Err(Error::JS(JSError::Conversion(
                "could not convert to string".to_owned(),
            )))
        }
    }

    fn new_number<N: Into<f64>>(ctx: Self::Context, value: N) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_bool(ctx: Self::Context, value: bool) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_null(ctx: Self::Context) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_undefined(ctx: Self::Context) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_function(ctx: Self::Context, func: &fn()) -> crate::types::Result<Self> {
        todo!()
    }
}

// impl<'a> From<Local<'a, Value>> for V8Value<'a> {
//     fn from(value: Local<'a, Value>) -> Self {
//         Self {
//             context: Rc::new(RefCell::new(V8Context::default().unwrap())),
//             value,
//         }
//     }
// }
