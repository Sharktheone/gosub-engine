use crate::js::v8::context_store::Store;
use crate::js::v8::{V8Object, V8Value};
use crate::js::{Context, JSContext, JSError};
use crate::types::Error;
use v8::{ContextScope, CreateParams, HandleScope, Isolate, Local, Message, Value};

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
        Store::raise_context_scope_count(self.id);
        let data = Store::get_context_scope(self.id).expect("we have fucked up somewhere in the safety system... \n This should have been prevented. \n This is a bug!");

        V8ContextInner {
            id: self.id,
            data,
            _phantom: std::marker::PhantomData,
        }
    }

    extern "C" fn listener(&self, message: Local<Message>, value: Local<Value>) {


    }
}

impl<'a> JSContext for V8Context<'a> {
    type Object = V8Object<'a>;
    type Value = V8Value<'a>;

    fn run(&self, code: &str) -> crate::types::Result<Self::Value> {


        let s = self.scope();

        let code = v8::String::new(s.data, code).unwrap();

        let script = v8::Script::compile(s.data, code, None);

        let Some(script) = script else {
            let try_catch = &mut v8::TryCatch::new(s.data);

            let s = self.scope();

            return Err(Error::JS(JSError::Generic("unknown error".to_owned())));
        };

        let value = script.run(s.data).unwrap();

        Ok(V8Value::from(value))
    }

    fn compile(&self, code: &str) -> crate::types::Result<()> {


        todo!()
    }

    fn run_compiled(&self) -> crate::types::Result<Self::Value> {
        todo!()
    }

    fn new_global_object(&self, name: &str) -> crate::types::Result<Self::Object> {
        todo!()
    }
}