use gosub_shared::types::Result;

use crate::js::{AsArray, IntoWebValue, JSType, WebRuntime};

pub trait WebValue<RT: WebRuntime>:
    Sized + From<RT::Object> + From<RT::Array> + AsArray<RT>
where
    Self: Sized,
{

    fn as_string(&self) -> Result<String>;

    fn as_number(&self) -> Result<f64>;

    fn as_bool(&self) -> Result<bool>;

    fn as_object(&self) -> Result<RT::Object>;

    fn as_array(&self) -> Result<RT::Array>;

    fn is_string(&self) -> bool;

    fn is_number(&self) -> bool;

    fn is_bool(&self) -> bool;

    fn is_object(&self) -> bool;

    fn is_array(&self) -> bool;

    fn is_null(&self) -> bool;

    fn is_undefined(&self) -> bool;

    fn is_function(&self) -> bool;

    fn type_of(&self) -> JSType;

    fn new_object(ctx: RT::Context) -> Result<RT::Object>;

    fn new_array<T: IntoWebValue<Self, Value = Self>>(
        ctx: RT::Context,
        value: &[T],
    ) -> Result<RT::Array>;

    fn new_empty_array(ctx: RT::Context) -> Result<RT::Array>;

    fn new_string(ctx: RT::Context, value: &str) -> Result<Self>;

    fn new_number<N: Into<f64>>(context: RT::Context, value: N) -> Result<Self>;

    fn new_bool(ctx: RT::Context, value: bool) -> Result<Self>;

    fn new_null(ctx: RT::Context) -> Result<Self>;

    fn new_undefined(ctx: RT::Context) -> Result<Self>;
}
