use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub use array::V8Array;
pub use context::V8Context;
pub use object::V8Object;
pub use value::V8Value;

use crate::js::{
    JSArray, JSContext, JSObject, JSRuntime, JSValue, ValueConversion,
};
use crate::js::context::Context;
use crate::types::Result;

mod array;
mod context;
mod context_store;
mod object;
mod value;

static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);
static PLATFORM_INITIALIZING: AtomicBool = AtomicBool::new(false);

pub struct V8Engine<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
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

    fn new_context(&mut self) -> Result<Context<Self::Context>> {
        Ok(Self::Context::default())
    }
}

mod tests {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use crate::js::{JSContext, JSRuntime, JSValue, runtime, Runtime};
    use crate::js::v8::V8Engine;
    use crate::types::Error;

    lazy_static! {
        static ref RUNTIME: Mutex<Runtime<V8Engine<'static>>> = Mutex::new(runtime::Runtime::new());
    }

    #[test]
    fn v8_bindings_test() {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut v8::Isolate::new(Default::default());
        let hs = &mut v8::HandleScope::new(isolate);
        let c = v8::Context::new(hs);
        let s = &mut v8::ContextScope::new(hs, c);

        let code = v8::String::new(s, "console.log(\"Hello World!\"); 1234").unwrap();

        let value = v8::Script::compile(s, code, None).unwrap().run(s).unwrap();

        println!("{}", value.to_rust_string_lossy(s));
    }

    #[test]
    fn execution() {
        let mut rt = RUNTIME.lock().unwrap();

        let context = rt.new_context().unwrap();

        let value = context.run(
            r#"
            console.log("Hello World!");
            1234
        "#,
        ).unwrap();

        assert!(value.is_number());
        assert_eq!(value.as_number().unwrap(), 1234.0);
    }

    #[test]
    fn invalid_syntax() {
        let mut rt = RUNTIME.lock().unwrap();

        let context = rt.new_context().unwrap();

        let result = context.run(r#"
        console.log(Hello World!);
        1234
        "#);

        assert!(result.is_err());

        assert!(matches!(result, Err(Error::JS(crate::js::JSError::Compile(_)))));
    }
}
