use std::cell::RefCell;
use std::marker::PhantomData;
use crate::types::{Error, Result};

use v8::{ContextScope, HandleScope, OwnedIsolate};
use crate::js::JSError;
pub(super) struct Store<'a> {
    isolates: Vec<(usize, OwnedIsolate)>,
    handle_scopes: Vec<(usize, HandleScope<'a, ()>)>,
    context_scopes: Vec<(usize, ContextScope<'a, HandleScope<'a>>)>,
    _marker: PhantomData<*mut ()>,
}

//TODO: we can get rid of this static value by using #[self_referencing] on the V8Context struct, but currently id doesn't support chain references


thread_local! {
    static STORE: RefCell<Store<'static>> = RefCell::new(Store::new());
}


impl<'a> Default for Store<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Store<'a> {
    const fn new() -> Self {
        Self {
            isolates: Vec::new(),
            handle_scopes: Vec::new(),
            context_scopes: Vec::new(),
            _marker: PhantomData,
        }
    }
    fn _insert_isolate(&mut self, id: usize, isolate: OwnedIsolate) {
        self.isolates.push((id, isolate));
    }

    #[inline(always)]
    fn _insert_handle_scope(&mut self, id: usize, handle_scope: HandleScope<'a, ()>) {
        self.handle_scopes.push((id, handle_scope));
    }

    fn _insert_context_scope(&mut self, id: usize, context_scope: ContextScope<'a, HandleScope<'a>>) {
        self.context_scopes.push((id, context_scope));
    }

    fn _with_isolate<F, R>(&'a mut self, id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut OwnedIsolate) -> Result<R>
    {
        let isolate = self.isolates.iter_mut().find_map(|(i, isolate)| {
            if *i == id {
                Some(isolate)
            } else {
                None
            }
        }).ok_or_else(|| Error::JS(JSError::Generic("isolate not found".to_owned())))?;
        f(isolate)


    }

    fn _with_handle_scope<F, R>(&'a mut self, id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut HandleScope<'a, ()>) -> Result<R>
    {
        let handle_scope = self.handle_scopes.iter_mut().find_map(|(i, handle_scope)| {
            if *i == id {
                Some(handle_scope)
            } else {
                None
            }
        }).ok_or_else(|| Error::JS(JSError::Generic("handle scope not found".to_owned())))?;
        f(handle_scope)
    }

    fn _with_context_scope<F, R>(&'a mut self, id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut ContextScope<'a, HandleScope<'a>>) -> Result<R>
    {
        let context_scope = self.context_scopes.iter_mut().find_map(|(i, context_scope)| {
            if *i == id {
                Some(context_scope)
            } else {
                None
            }
        }).ok_or_else(|| Error::JS(JSError::Generic("context scope not found".to_owned())))?;
        f(context_scope)
    }

    pub fn insert_isolate(id: usize, isolate: OwnedIsolate) -> Result<()> {
        STORE.try_with(|store| {
            store.borrow_mut()._insert_isolate(id, isolate);
        }).map_err(|_| Error::JS(JSError::Generic("failed to insert isolate".to_owned())))?;


        Ok(())

    }

    pub fn insert_handle_scope(id: usize, handle_scope: HandleScope<'static, ()>) -> Result<()> {
        STORE.try_with(|store| {
            store.borrow_mut()._insert_handle_scope(id, handle_scope);
        }).map_err(|_| Error::JS(JSError::Generic("failed to insert handle scope".to_owned())))?;

        Ok(())
    }

    pub fn insert_context_scope(id: usize, context_scope: ContextScope<'static, HandleScope<'a>>) -> Result<()> {
        STORE.try_with(|store| {
            store.borrow_mut()._insert_context_scope(id, context_scope);
        }).map_err(|_| Error::JS(JSError::Generic("failed to insert context scope".to_owned())))?;

        Ok(())
    }

    pub fn with_isolate<F, R>(id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut OwnedIsolate) -> Result<R>
    {
        STORE.try_with(|store| {
            store.borrow_mut()._with_isolate(id, |isolate| {
                Ok(isolate)
            })
        }).map_err(|_| Error::JS(JSError::Generic("failed to get isolate".to_owned())))?
            .and_then(|isolate| f(isolate))
    }

    pub fn with_handle_scope<F, R>(id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut HandleScope<'a, ()>) -> Result<R>
    {
        STORE.try_with(|store| {
            store.borrow_mut()._with_handle_scope(id, |handle_scope| {
                Ok(handle_scope)
            })
        }).map_err(|_| Error::JS(JSError::Generic("failed to get handle scope".to_owned())))?
            .and_then(|handle_scope| f(handle_scope))
    }

    pub fn with_context_scope<F, R>(id: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut ContextScope<'a, HandleScope<'a>>) -> Result<R>
    {
        STORE.try_with(|store| {
            store.borrow_mut()._with_context_scope(id, |context_scope| {
                Ok(context_scope)
            })
        }).map_err(|_| Error::JS(JSError::Generic("failed to get context scope".to_owned())))?
            .and_then(|context_scope| f(context_scope))
    }

    pub fn isolate<F, R>(id: usize, isolate: OwnedIsolate, f: F) -> Result<R>
    where
        F: FnOnce(&mut HandleScope<'a, ()>) -> Result<R>
    {
        Self::insert_isolate(id, isolate)?;
        Self::with_handle_scope(id, f)
    }

    pub fn handle_scope<F, R>(id: usize, handle_scope: HandleScope<'a, ()>, f: F) -> Result<R>
    where
        F: FnOnce(&mut ContextScope<'a, HandleScope<'a>>) -> Result<R>
    {
        Self::insert_handle_scope(id, handle_scope)?;
        Self::with_context_scope(id, f)
    }

    pub fn context_scope<F, R>(id: usize, context_scope: ContextScope<'a, HandleScope<'a>>, f: F) -> Result<R>
    where
        F: FnOnce(&mut ContextScope<'a, HandleScope<'a>>) -> Result<R>
    {
        Self::insert_context_scope(id, context_scope)?;
        Self::with_context_scope(id, f)
    }

    pub fn drop(id: usize) -> Result<()> {
        STORE.try_with(|store| {
            store.borrow_mut().isolates.retain(|(i, _)| *i != id);
            store.borrow_mut().handle_scopes.retain(|(i, _)| *i != id);
            store.borrow_mut().context_scopes.retain(|(i, _)| *i != id);
        }).map_err(|_| Error::JS(JSError::Generic("failed to drop isolate".to_owned())))
    }
}