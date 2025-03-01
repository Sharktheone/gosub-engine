use std::ffi::c_void;
use gosub_webexecutor::js::{JSError, WebGetterCallback, WebObject, WebRuntime, WebValue, WebObjectTemplate};
use v8::{AccessorConfiguration, External, Global, HandleScope, Local, Name, ObjectTemplate, PropertyCallbackArguments, ReturnValue, Value};
use gosub_webexecutor::Error;
use crate::{GetterCallback, GetterSetter, SetterCallback, V8Context, V8Engine, V8Function, V8FunctionVariadic, V8Object, V8Value};
use gosub_shared::types::Result;

#[derive(Clone)]
pub struct V8ObjectTemplate {
    pub ctx: V8Context,
    pub value: Global<ObjectTemplate>,
}

impl V8ObjectTemplate {
    pub fn new(ctx: V8Context) -> Result<Self> {
        let mut scope = ctx.scope();
        let value = ObjectTemplate::new(&mut scope);

        let value = Global::new(&mut scope, value);

        drop(scope);

        Ok(Self { ctx, value })
    }
}

impl WebObjectTemplate for V8ObjectTemplate {
    type RT = V8Engine;
    fn set_property(&self, name: &str, value: &V8Value) -> Result<()> {
        let scope = &mut self.ctx.scope();


        let Some(name) = v8::String::new(scope, name) else {
            return Err(Error::JS(JSError::Generic("failed to create a string".to_owned())).into());
        };

        let obj = self.value.open(scope);
        let value = Local::new(scope, value.value.clone());

        obj.set(name.into(), value.into());
        
        Ok(())
    }

    fn set_method(&self, name: &str, func: &V8Function) -> Result<()> {
        let scope = &mut self.ctx.scope();

        let Some(name) = v8::String::new(scope, name) else {
            return Err(Error::JS(JSError::Generic("failed to create a string".to_owned())).into());
        };

        let f = Local::new(scope, func.function.clone());

        let obj = self.value.open(scope);

        obj.set(name.into(), f.into());
        
        Ok(())
    }

    fn set_method_variadic(&self, name: &str, func: &V8FunctionVariadic) -> Result<()> {
        let scope = &mut self.ctx.scope();

        let Some(name) = v8::String::new(scope, name) else {
            return Err(Error::JS(JSError::Generic("failed to create a string".to_owned())).into());
        };

        let f = Local::new(scope, func.function.clone());

        let obj = self.value.open(scope);

        obj.set(name.into(), f.into());
        
        Ok(())
    }

    fn set_property_accessor(
        &self,
        name: &str,
        getter: Box<dyn Fn(&mut <Self::RT as WebRuntime>::GetterCB)>,
        setter: Box<dyn Fn(&mut <Self::RT as WebRuntime>::SetterCB)>,
    ) -> Result<()> {
        let scope = &mut self.ctx.scope();
        let name = v8::String::new(scope, name)
            .ok_or_else(|| Error::JS(JSError::Generic("failed to create a string".to_owned())))?;

        let gs = Box::new(GetterSetter {
            ctx: self.ctx.clone(),
            getter,
            setter,
        });

        let data = External::new(scope, Box::into_raw(gs) as *mut c_void);

        let config = AccessorConfiguration::new(
            |scope: &mut HandleScope, _name: Local<Name>, args: PropertyCallbackArguments, mut rv: ReturnValue| {
                let external = match Local::<External>::try_from(args.data()) {
                    Ok(external) => external,
                    Err(e) => {
                        let Some(e) = V8Context::create_exception(scope, e) else {
                            eprintln!("failed to create exception string\nexception was: {e}");
                            return;
                        };
                        scope.throw_exception(e);
                        return;
                    }
                };

                let gs = unsafe { &*(external.value() as *const GetterSetter) };

                let ret = match V8Value::new_undefined(gs.ctx.clone()) {
                    Ok(ret) => ret,
                    Err(e) => {
                        gs.ctx.error(e);
                        return;
                    }
                };

                let sg = gs.ctx.set_parent_scope(HandleScope::new(scope));

                let mut gc = GetterCallback {
                    ctx: gs.ctx.clone(),
                    ret,
                };
                //TODO: do we need to drop the scope here?
                (gs.getter)(&mut gc);

                drop(sg);

                rv.set(Local::new(scope, gc.ret.value));
            },
        )
            .setter(
                |scope: &mut HandleScope,
                 _name: Local<Name>,
                 value: Local<Value>,
                 args: PropertyCallbackArguments,
                 _rv: ReturnValue<()>| {
                    let external = match Local::<External>::try_from(args.data()) {
                        Ok(external) => external,
                        Err(e) => {
                            let Some(e) = V8Context::create_exception(scope, e) else {
                                eprintln!("failed to create exception string\nexception was: {e}");
                                return;
                            };
                            scope.throw_exception(e);
                            return;
                        }
                    };

                    let gs = unsafe { &*(external.value() as *const GetterSetter) };

                    let val = V8Value::from_value(gs.ctx.clone(), Global::new(scope, value));

                    let sg = gs.ctx.set_parent_scope(HandleScope::new(scope));

                    let mut sc = SetterCallback {
                        ctx: gs.ctx.clone(),
                        value: val,
                    };

                    (gs.setter)(&mut sc);

                    drop(sg);
                },
            )
            .data(Local::from(data));

        let obj = self.value.open(scope);

        obj.set_accessor_with_configuration(name.into(), config);

        Ok(())
    }

    fn new(ctx: &<Self::RT as WebRuntime>::Context) -> Result<Self> {
        Self::new(ctx.clone())
    }

    fn new_instance(&self) -> Result<<Self::RT as WebRuntime>::Object> {
        let scope = &mut self.ctx.scope();
        let obj = self.value.open(scope).new_instance(scope)
            .ok_or_else(|| Error::JS(JSError::Generic("failed to create a new instance".to_owned())))?;
        
        Ok(V8Object {
            ctx: self.ctx.clone(),
            value: Global::new(scope, obj),
            _marker: std::marker::PhantomData,
        })
    }
}