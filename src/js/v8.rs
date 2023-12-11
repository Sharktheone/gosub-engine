use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

use v8::{ContextScope, CreateParams, HandleScope, Isolate, Local};

use crate::js::{JSArray, JSContext, JSError, JSObject, JSRuntime, JSType, JSValue, ValueConversion};
use crate::js::context::Context;
use crate::js::v8::context_store::Store;
use crate::types::{Error, Result};

mod context_store;

static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);
static PLATFORM_INITIALIZING: AtomicBool = AtomicBool::new(false);


pub struct V8Engine<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}


pub struct V8Context<'a> {
    id: usize,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> V8Context<'a> {
    fn new(params: CreateParams) -> Context<Self> {
        let id = rand::random();

        let isolate = Store::isolate(id, Isolate::new(params));

        let hs = Store::handle_scope(id, HandleScope::new(isolate));

        let ctx = v8::Context::new(hs);

        Store::insert_context_scope(id, ContextScope::new(hs, ctx));

        Context(Self {
            id,
            _phantom: std::marker::PhantomData,
        })
    }

    fn default() -> Context<Self> {
        Self::new(Default::default())
    }

    fn scope(&self) -> &'a mut ContextScope<'static, HandleScope<'a>> {
        let Some(scope) = Store::get_context_scope(self.id)
            else {
                Self::new(Default::default());
                return Store::get_context_scope(self.id).expect("we should not be here. Created a new context but it is not in the store");
            };

        scope
    }
}

impl V8Engine<'_> {
    pub fn initialize() {
        if PLATFORM_INITIALIZED.load(Ordering::SeqCst) {
            return;
        }

        if PLATFORM_INITIALIZING.load(Ordering::SeqCst) {
            while !PLATFORM_INITIALIZED.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            return;
        }

        PLATFORM_INITIALIZING.store(true, Ordering::SeqCst);


        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        PLATFORM_INITIALIZED.store(true, Ordering::SeqCst);
        PLATFORM_INITIALIZING.store(false, Ordering::SeqCst);
    }

    pub fn new() -> Self {
        Self::initialize();
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a> JSRuntime for V8Engine<'a> {
    type Context = V8Context<'a>;


    //let isolate = &mut Isolate::new(Default::default());
    //let hs = &mut HandleScope::new(isolate);
    //let c = Context::new(hs);
    //let s = &mut ContextScope::new(hs, c);

    fn new_context(&'static mut self) -> Result<Context<Self::Context>> {
        Ok(Self::Context::default())
    }
}

impl<'a> JSContext for V8Context<'a> {
    type Object = V8Object<'a>;

    fn run(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn compile(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn run_compiled(&self) -> Result<()> {
        todo!()
    }

    fn add_global_object(&self, name: &str) -> Result<Self::Object> {
        todo!()
    }
}

impl<'a> JSContext for V8Engine<'a> {
    type Object = V8Object<'a>;

    fn run(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn compile(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn run_compiled(&self) -> Result<()> {
        todo!()
    }

    fn add_global_object(&self, name: &str) -> Result<Self::Object> {
        todo!()
    }
}

pub struct V8Object<'a>(Local<'a, v8::Object>);

impl<'a> JSObject for V8Object<'a> {
    type Value = V8Value<'a>;

    fn set_property(&self, name: &str, value: &str) -> Result<()> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Result<Self::Value> {
        todo!()
    }

    fn call_method(&self, name: &str, args: &[&str]) -> Result<Self::Value> {
        todo!()
    }

    fn set_method(&self, name: &str, function: &str) -> Result<()> {
        todo!()
    }
}

pub struct V8Value<'a>(Local<'a, v8::Value>);

impl<'a> JSValue for V8Value<'a> {
    type Object = V8Object<'a>;
    type Array = V8Array<'a>;

    fn as_string(&self) -> Result<String> {
        todo!()
    }

    fn as_number(&self) -> Result<f64> {
        let mut scope: HandleScope = todo!();

        if let Some(value) = self.0.number_value(&mut scope) {
            return Ok(value);
        } else {
            return Err(Error::JS(JSError::Conversion("could not convert to number".to_owned())));
        }
    }

    fn as_bool(&self) -> Result<bool> {
        todo!()
    }

    fn as_object(&self) -> Result<Self::Object> {
        todo!()
    }

    fn as_array(&self) -> Result<Self::Array> {
        todo!()
    }

    fn is_string(&self) -> bool {
        todo!()
    }

    fn is_number(&self) -> bool {
        todo!()
    }

    fn is_bool(&self) -> bool {
        todo!()
    }

    fn is_object(&self) -> bool {
        todo!()
    }

    fn is_array(&self) -> bool {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }

    fn is_undefined(&self) -> bool {
        todo!()
    }

    fn is_function(&self) -> bool {
        todo!()
    }

    fn type_of(&self) -> JSType {
        todo!()
    }

    fn new_object() -> Result<Self::Object> {
        todo!()
    }

    fn new_array<T: ValueConversion<Self>>(value: &[T]) -> Result<Self::Array> {
        todo!()
    }

    fn new_string(value: &str) -> Result<Self> {
        todo!()
    }

    fn new_number<N: Into<f64>>(value: N) -> Result<Self> {
        todo!()
    }

    fn new_bool(value: bool) -> Result<Self> {
        todo!()
    }

    fn new_null() -> Result<Self> {
        todo!()
    }

    fn new_undefined() -> Result<Self> {
        todo!()
    }

    fn new_function(func: &fn()) -> Result<Self> {
        todo!()
    }
}

pub struct V8Array<'a> (v8::Local<'a, v8::Array>);

impl<'a> JSArray for V8Array<'a> {
    type Value = V8Value<'a>;

    fn get(&self, index: usize) -> Result<Self::Value> {
        todo!()
    }

    fn set(&self, index: usize, value: &str) -> Result<()> {
        todo!()
    }

    fn push(&self, value: &str) -> Result<()> {
        todo!()
    }

    fn pop(&self) -> Result<Self::Value> {
        todo!()
    }

    fn length(&self) -> Result<usize> {
        todo!()
    }
}
