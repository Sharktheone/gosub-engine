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
    _marker: std::marker::PhantomData<*mut ()>,
}


struct V8ContextInner<'a> {
    id: usize,
    data: &'a mut ContextScope<'a, HandleScope<'a>>,
    _phantom: std::marker::PhantomData<&'a mut ContextScope<'a, HandleScope<'a>>>,
}


impl Drop for V8ContextInner<'_> {
    fn drop(&mut self) {
        Store::lower_context_scope_count(self.id);
    }
}


impl Drop for V8Context<'_> {
    fn drop(&mut self) {
        Store::drop(self.id);
    }
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
            _marker: std::marker::PhantomData,
        })
    }

    fn default() -> Context<Self> {
        Self::new(Default::default())
    }

    /// You can't move the scope out of this struct for safety reasons.
    /// But you can get a reference to it.
    /// ```not_rust
    /// let s = context::scope();
    /// v8::String::new(s.data, string)
    /// ```
    fn scope(&self) -> V8ContextInner<'static> {
        Store::raise_context_scope_count(self.id);
        let data = Store::get_context_scope(self.id).expect("we have fucked up somewhere in the safety system... \n This should have been prevented. \n This is a bug!");

        V8ContextInner {
            id: self.id,
            data,
            _phantom: std::marker::PhantomData,
        }
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

    fn new_context(&mut self) -> Result<Context<Self::Context>> {
        Ok(Self::Context::default())
    }
}

impl<'a> JSContext for V8Context<'a> {
    type Object = V8Object<'a>;

    fn run(&self, code: &str) -> Result<()> {
        let s = self.scope();

        let code = v8::String::new(s.data, code).unwrap();

        let script = v8::Script::compile(s.data, code, None);

        let Some(script) = script else {
            let try_catch = &mut v8::TryCatch::new(s.data);

            let s = self.scope();

            if let Some(exception) = try_catch.exception() {
                let exception = exception.to_string(s.data).unwrap();

                let exception = exception.to_rust_string_lossy(s.data);

                println!("V8Exception: {}", exception);

                return Err(Error::JS(JSError::Generic(exception)));
            }

            if let Some(stack_trace) = try_catch.stack_trace() {
                let stack_trace = stack_trace.to_string(s.data).unwrap();

                let stack_trace = stack_trace.to_rust_string_lossy(s.data);

                println!("V8StackTrace: {}", stack_trace);
            }

            if let Some(message) = try_catch.message() {
                let message = message.get(s.data);

                let message = message.to_rust_string_lossy(s.data);

                println!("V8Message: {}", message);
            }

            println!("HELLO");


            return Err(Error::JS(JSError::Generic("unknown error".to_owned())));
        };

        let value = script.run(s.data).unwrap();

        println!("{}", value.to_rust_string_lossy(s.data));

        Ok(())
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


mod tests {
    use std::cell::RefCell;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    use crate::js::{JSContext, JSRuntime, Runtime, runtime};
    use crate::js::v8::V8Engine;

    lazy_static!(
    static ref RUNTIME: Mutex<Runtime<V8Engine<'static>>> = Mutex::new(runtime::Runtime::new());
    );


    #[test]
    fn test() {
        let mut rt = RUNTIME.lock().unwrap();

        let context = rt.new_context().unwrap();

        context.run(r#"
            console.log("Hello World!");
            1234
        "#).unwrap();

        println!("dropped context")
    }


    //BREAKPOINTS
    //
    // scope.rs:1294
    // scope.rs:1534
    // scope:rs:1581

    // #[test]
    fn test2() {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut v8::Isolate::new(Default::default());
        let hs = &mut v8::HandleScope::new(isolate);
        let c = v8::Context::new(hs);
        let mut s = &mut v8::ContextScope::new(hs, c);

        let code = v8::String::new(s, "console.log(\"Hello World!\"); 1234").unwrap();

        // let value = v8::Script::compile(s, code, None);

        let value = v8::Script::compile(s, code, None).unwrap().run(s).unwrap();

        println!("{}", value.to_rust_string_lossy(s));
    }
}