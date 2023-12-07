use std::sync::atomic::{AtomicBool, Ordering};
use crate::js::{JSContext, JSRuntime, JSObject, JSValue, JSType, ValueConversion, JSArray};
use crate::js::context::Context;
use crate::types::Result;



static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);
static PLATFORM_INITIALIZING: AtomicBool = AtomicBool::new(false);

pub struct V8Engine;

pub struct V8Context {
    pub isolate: v8::OwnedIsolate,
}

impl V8Engine {
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
        Self
    }
}

impl JSRuntime for V8Engine {
    type Context = V8Context;


    fn new_context(&self) -> Result<Context<Self::Context>> {
        todo!()
    }
}


impl JSContext for V8Context {
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
    type Value = V8Value;

    fn set_property(&self, name: &str, value: &str) -> Result<()> {
        todo!()
    }

    fn get_property(&self, name: &str) -> Result<V8Value> {
        todo!()
    }

    fn call_method(&self, name: &str, args: &[&str]) -> Result<V8Value> {
        todo!()
    }

    fn set_method(&self, name: &str, function: &str) -> Result<()> {
        todo!()
    }
}


pub struct V8Value {
    pub value: v8::Global<v8::Value>,
}

impl JSValue for V8Value {
    type Object = V8Object;
    type Array = V8Array;

    fn as_string(&self) -> Result<String> {
        todo!()
    }

    fn as_number(&self) -> Result<f64> {
        todo!()
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

    fn new_array<T: ValueConversion>(value: &[T]) -> Result<Self::Array> {
        todo!()
    }

    fn new_string(value: &str) -> Result<Self> {
        todo!()
    }

    fn new_number(value: f64) -> Result<Self> {
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

pub struct V8Array {
    pub array: v8::Global<v8::Array>,
}

impl JSArray for V8Array {
    type Value = V8Value;

    fn get(&self, index: usize) -> Result<V8Value> {
        todo!()
    }

    fn set(&self, index: usize, value: &str) -> Result<()> {
        todo!()
    }

    fn push(&self, value: &str) -> Result<()> {
        todo!()
    }

    fn pop(&self) -> Result<V8Value> {
        todo!()
    }

    fn length(&self) -> Result<usize> {
        todo!()
    }
}