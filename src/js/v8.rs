use std::sync::atomic::{AtomicBool, Ordering};
use crate::js::{JSContext, JSRuntime};


static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct V8Engine;

pub struct V8Context {
    pub isolate: v8::OwnedIsolate,
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

    fn new() -> crate::types::Result<Self> where Self: Sized {
        todo!()
    }

    fn new_context(&self) -> crate::types::Result<Self::Context> {
        todo!()
    }
}


impl JSContext for V8Engine {
    fn run(&self, code: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn compile(&self, code: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn run_compiled(&self) -> crate::types::Result<()> {
        todo!()
    }

    fn add_global_object(&self, name: &str, object: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn add_function_to_object(&self, object: &str, name: &str, function: &str) -> crate::types::Result<()> {
        todo!()
    }
}