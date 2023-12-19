use std::ptr::NonNull;

use v8::{ContextScope, CreateParams, HandleScope, Isolate, OwnedIsolate, TryCatch};

use crate::js::{Context, JSContext, JSError};
use crate::js::v8::{V8Object, V8Value};
use crate::js::v8::context_store::{Inner, Store};
use crate::types::{Error, Result};

pub struct V8Context<'a> {
    inner: &'static mut Inner<ContextScope<'static, HandleScope<'static>>>,
    _marker: std::marker::PhantomData<&'a ()>,
}

struct V8ContextInner<'a> {
    id: usize,
    data: &'a mut ContextScope<'a, HandleScope<'a>>,
}

impl Drop for V8ContextInner<'_> {
    fn drop(&mut self) {
        Store::lower_context_scope_count(self.id);
    }
}

impl Drop for V8Context<'_> {
    fn drop(&mut self) {
        Store::drop(self.inner.id);
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
            inner: Store::get_inner_context_scope(id).expect("Context not found"),
            _marker: std::marker::PhantomData,
        })
    }

    pub(super) fn default() -> Context<Self> {
        Self::new(Default::default())
    }

    /// You can't move the scope out of this struct for safety reasons.
    /// But you can get a reference to it.
    /// ```not_rust
    /// let s = context::scope();
    /// v8::String::new(s.data, string)
    /// ```
    fn scope(&self) -> V8ContextInner<'static> {
        V8ContextInner {
            id: self.inner.id,
            data: Store::get_context_scope(self.inner.id).expect("Something weird happened... We've fucked up somewhere in the safety system. This is a bug!"), //TODO: Handle error,
        }
    }

    fn report_exception(try_catch: &mut TryCatch<HandleScope>) -> Error {
        if let Some(exception) = try_catch.exception() {
            let e = exception.to_rust_string_lossy(try_catch);

            return Error::JS(JSError::Compile(e));
        }

        if let Some(m) = try_catch.message() {
            let message = m.get(try_catch).to_rust_string_lossy(try_catch);

            return Error::JS(JSError::Compile(message));
        }

        Error::JS(JSError::Compile("unknown error".to_owned()))
    }
}

impl<'a> JSContext for V8Context<'a> {
    type Object = V8Object<'a>;
    type Value = V8Value<'a>;

    fn run(&mut self, code: &str) -> Result<Self::Value> {
        let s = self.scope();

        let try_catch = &mut v8::TryCatch::new(s.data);

        let code = v8::String::new(try_catch, code).unwrap();

        let script = v8::Script::compile(try_catch, code, None);

        let Some(script) = script else {
            return Err(Self::report_exception(try_catch));
        };

        let Some(value) = script.run(try_catch) else {
            return Err(Self::report_exception(try_catch));
        };

        Ok(V8Value::from(value))
    }

    fn compile(&mut self, code: &str) -> Result<()> {
        todo!()
    }

    fn run_compiled(&mut self) -> Result<Self::Value> {
        todo!()
    }

    fn new_global_object(&mut self, name: &str) -> Result<Self::Object> {
        todo!()
    }
}


struct Test<'a> {
    isolate: NonNull<OwnedIsolate>,
    handle_scope: NonNull<HandleScope<'a, ()>>,
    context_scope: NonNull<ContextScope<'a, HandleScope<'a>>>,
    init: bool,
}

impl<'a> Test<'a> {
    fn new() -> Test<'a> {
        Self {
            isolate: NonNull::dangling(),
            handle_scope: NonNull::dangling(),
            context_scope: NonNull::dangling(),
            init: false,
        }
    }
    fn initialize(&mut self) -> Result<()> {
        let isolate = Box::new(Isolate::new(Default::default()));

        let Some(isolate) = NonNull::new(Box::into_raw(isolate)) else {
            return Err(Error::JS(JSError::Compile("Failed to create isolate".to_owned())));
        };
        self.isolate = isolate;

        let handle_scope = Box::new(HandleScope::new(unsafe {
            self.isolate.as_mut()
        }));

        let Some(handle_scope) = NonNull::new(Box::into_raw(handle_scope)) else {
            return Err(Error::JS(JSError::Compile("Failed to create handle scope".to_owned())));
        };

        self.handle_scope = handle_scope;

        let ctx = v8::Context::new(unsafe {
            self.handle_scope.as_mut()
        });

        let ctx_scope = Box::new(ContextScope::new(unsafe {
            self.handle_scope.as_mut()
        }, ctx));

        let Some(ctx_scope) = NonNull::new(Box::into_raw(ctx_scope)) else {
            return Err(Error::JS(JSError::Compile("Failed to create context scope".to_owned())));
        };

        self.context_scope = ctx_scope;

        self.init = true;

        Ok(())
    }

    fn scope(&mut self) -> Result<&'a mut ContextScope<'a, HandleScope<'a>>> {
        if !self.init {
            self.initialize()?;
        }

        Ok(unsafe {
            self.context_scope.as_mut()
        })
    }
}



#[test]
fn t() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let mut t = Test::new();
    t.initialize();

    let s = t.scope().expect("Failed to get scope");

    let code = v8::String::new(s, "console.log(\"Hello World!\"); 1234").unwrap();

    let script = v8::Script::compile(s, code, None);

    let script = script.unwrap();

    let value = script.run(s).unwrap();

    println!("{}", value.to_string(s).unwrap().to_rust_string_lossy(s));

    move_test(t);
}

fn move_test(mut t: Test) {
    let mut closure = move || {
        let s = t.scope().expect("Failed to get scope");

        let code = v8::String::new(s, "console.log(\"Hello World!\"); 1234").unwrap();

        let script = v8::Script::compile(s, code, None);

        let script = script.unwrap();

        let value = script.run(s).unwrap();

        println!("{}", value.to_string(s).unwrap().to_rust_string_lossy(s));
    };

    closure();
}