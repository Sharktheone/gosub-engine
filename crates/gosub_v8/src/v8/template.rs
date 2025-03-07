use gosub_webexecutor::js::{WebGetterCallback, WebObject, WebRuntime, WebValue, WebObjectTemplate};
use crate::{V8Context, V8Engine, V8Function, V8FunctionVariadic, V8Object, V8Value};
use gosub_shared::types::Result;

#[derive(Clone)]
pub struct V8ObjectTemplate(pub V8Object);

impl V8ObjectTemplate {
    pub fn new(ctx: V8Context) -> Result<Self> {
        Ok(Self(V8Object::new(ctx)?))
    }
}

impl WebObjectTemplate for V8ObjectTemplate {
    type RT = V8Engine;
    fn set_property(&self, name: &str, value: &V8Value) -> Result<()> {
        self.0.set_property(name, value)
    }

    fn set_method(&self, name: &str, func: &V8Function) -> Result<()> {
        self.0.set_method(name, func)
    }

    fn set_method_variadic(&self, name: &str, func: &V8FunctionVariadic) -> Result<()> {
        self.0.set_method_variadic(name, func)
    }

    fn inherit_from(&self, parent: &<Self::RT as WebRuntime>::ObjectTemplate) -> Result<()> {
        todo!()
    }

    fn set_property_accessor(
        &self,
        name: &str,
        getter: Box<dyn Fn(&mut <Self::RT as WebRuntime>::GetterCB)>,
        setter: Box<dyn Fn(&mut <Self::RT as WebRuntime>::SetterCB)>,
    ) -> Result<()> {
        self.0.set_property_accessor(name, getter, setter)
    }

    fn new(ctx: &<Self::RT as WebRuntime>::Context) -> Result<Self> {
        Self::new(ctx.clone())
    }

    fn new_instance(&self) -> Result<<Self::RT as WebRuntime>::Object> {
        
        todo!()
        
        
        // let scope = &mut self.0.ctx.scope();
        // let obj = self.0.value.open(scope).new_instance(scope)
        //     .ok_or_else(|| Error::JS(JSError::Generic("failed to create a new instance".to_owned())))?;
        
        // Ok(V8Object {
        //     ctx: self.0.ctx.clone(),
        //     value: Global::new(scope, obj),
        //     _marker: std::marker::PhantomData,
        // })
    }
}