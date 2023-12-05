use std::sync::atomic::{AtomicBool, Ordering};
use crate::js::{JSContext, JSRuntime};
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
    fn run(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn compile(&self, code: &str) -> Result<()> {
        todo!()
    }

    fn run_compiled(&self) -> Result<()> {
        todo!()
    }

    fn add_global_object(&self, name: &str, object: &str) -> Result<()> {
        todo!()
    }

    fn add_function_to_object(&self, object: &str, name: &str, function: &str) -> Result<()> {
        todo!()
    }
}