pub mod runtime_types;

use std::sync::atomic::{AtomicBool, Ordering};
use crate::types::Result;
use crate::js::{JSContext, JSObject, JSRuntime};


static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct V8Engine;

pub struct V8Context {
    pub isolate: v8::OwnedIsolate,
}

pub struct V8Value {
    pub value: v8::Global<v8::Value>,
}

pub struct V8Array {
    pub array: v8::Global<v8::Array>,
}

impl V8Engine {
    pub fn initialize() {
        if PLATFORM_INITIALIZED.load(Ordering::SeqCst) {
            return;
        }


        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        PLATFORM_INITIALIZED.store(true, Ordering::SeqCst);
    }

    pub fn new() -> Self {
        Self::initialize();
        Self
    }
}

impl JSRuntime for V8Engine {
    type Context = V8Context;
    type Object = V8Object;
    type Value = V8Value;
    type Array = V8Array;

    fn new() -> Result<Self> where Self: Sized {
        todo!()
    }

    fn new_context(&self) -> Result<Self::Context> {
        todo!()
    }
}


impl JSContext for V8Engine {

    type Object = V8Object;

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

pub struct V8Object {
    pub object: v8::Global<v8::Object>,
}

impl JSObject for V8Object {
    fn set_property(&self, name: &str, value: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn get_property(&self, name: &str) -> crate::types::Result<String> {
        todo!()
    }

    fn call_method(&self, name: &str, args: &[&str]) -> crate::types::Result<String> {
        todo!()
    }

    fn set_method(&self, name: &str, function: &str) -> crate::types::Result<()> {
        todo!()
    }
}