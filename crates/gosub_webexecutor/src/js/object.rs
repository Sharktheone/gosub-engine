use core::fmt::Display;
use std::ops::Deref;
use gosub_shared::types::Result;
use crate::js::gc::GarbageCollectable;
use crate::js::WebRuntime;

pub trait WebObject<RT: WebRuntime, I: GarbageCollectable = ()>: Into<RT::Value> + Clone {
    fn get_inner(&self) -> impl Deref<Target = I>;

    fn set_property(&self, name: &str, value: &RT::Value) -> Result<()>;

    fn get_property(&self, name: &str) -> Result<RT::Value>;

    fn call_method(
        &self,
        name: &str,
        args: &[&RT::Value],
    ) -> Result<RT::Value>;

    fn set_method(&self, name: &str, func: &RT::Function) -> Result<()>;

    fn set_method_variadic(&self, name: &str, func: &RT::FunctionVariadic) -> Result<()>;

    #[allow(clippy::type_complexity)]
    fn set_property_accessor(
        &self,
        name: &str,
        getter: Box<dyn Fn(&mut RT::GetterCB)>,
        setter: Box<dyn Fn(&mut RT::SetterCB)>,
    ) -> Result<()>;

    fn with_val(ctx: &RT::Context, val: I) -> Result<Self>;

    fn new(ctx: &RT::Context) -> Result<Self> where I: Default {
        Self::with_val(ctx, I::default())
    }
}



pub trait WebObjectTemplate<RT: WebRuntime>: Clone {

    fn set_property(&self, name: &str, value: &RT::Value) -> Result<()>;

    fn set_method(&self, name: &str, func: &RT::Function) -> Result<()>;

    fn set_method_variadic(&self, name: &str, func: &RT::FunctionVariadic) -> Result<()>;
    
    fn inherit_from(&self, parent: &RT::ObjectTemplate) -> Result<()>;

    #[allow(clippy::type_complexity)]
    fn set_property_accessor(
        &self,
        name: &str,
        getter: Box<dyn Fn(&mut RT::GetterCB)>,
        setter: Box<dyn Fn(&mut RT::SetterCB)>,
    ) -> Result<()>;

    fn new(ctx: &RT::Context) -> Result<Self>;
    fn new_instance(&self) -> Result<RT::Object>;
}


pub trait WebGetterCallback<RT: WebRuntime> {
    fn context(&mut self) -> &mut RT::Context;

    fn error(&mut self, error: impl Display);

    fn ret(&mut self, value: RT::Value);
}

pub trait WebSetterCallback<RT: WebRuntime> {
    fn context(&mut self) -> &mut RT::Context;

    fn error(&mut self, error: impl Display);

    fn value(&mut self) -> &RT::Value;
}
