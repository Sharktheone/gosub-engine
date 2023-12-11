use std::marker::PhantomData;

use v8::{ContextScope, HandleScope, OwnedIsolate};

pub(super) struct Store<'a> {
    isolates: Vec<(usize, OwnedIsolate)>,
    handle_scopes: Vec<(usize, HandleScope<'a, ()>)>,
    context_scopes: Vec<(usize, ContextScope<'a, HandleScope<'a>>)>,
    _marker: PhantomData<*mut ()>,
}




//TODO: we can get rid of this static value by using #[self_referencing] on the V8Context struct, but currently id doesn't support chain references
static mut STORE: Store<'static> = Store::new();


macro_rules! context {
    ($value: expr) => {
        STORE
    };
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

    fn _get_isolate(&'a mut self, id: usize) -> Option<&'a mut OwnedIsolate> {
        self.isolates.iter_mut().find_map(|(i, isolate)| {
            if *i == id {
                Some(isolate)
            } else {
                None
            }
        })
    }

    fn _get_handle_scope(&'a mut self, id: usize) -> Option<&'static mut HandleScope<'a, ()>> {
        self.handle_scopes.iter_mut().find_map(|(i, handle_scope)| {
            if *i == id {
                Some(handle_scope)
            } else {
                None
            }
        })
    }

    fn _get_context_scope(&'a mut self, id: usize) -> Option<&'static mut ContextScope<'a, HandleScope<'a>>> {
        self.context_scopes.iter_mut().find_map(|(i, context_scope)| {
            if *i == id {
                Some(context_scope)
            } else {
                None
            }
        })
    }

    pub fn insert_isolate(id: usize, isolate: OwnedIsolate) {
        unsafe {
            STORE._insert_isolate(id, isolate);
        }
    }

    pub fn insert_handle_scope(id: usize, handle_scope: HandleScope<'static, ()>) {
        unsafe {
            STORE._insert_handle_scope(id, handle_scope);
        }
    }

    pub fn insert_context_scope(id: usize, context_scope: ContextScope<'static, HandleScope<'a>>) {
        unsafe {
            STORE._insert_context_scope(id, context_scope);
        }
    }

    pub fn get_isolate(id: usize) -> Option<&'a mut OwnedIsolate> {
        unsafe { STORE._get_isolate(id) }
    }

    pub fn get_handle_scope(id: usize) -> Option<&'a mut HandleScope<'static, ()>> {
        unsafe { STORE._get_handle_scope(id) }
    }

    pub fn get_context_scope(id: usize) -> Option<&'a mut ContextScope<'static, HandleScope<'a>>> {
        unsafe { STORE._get_context_scope(id) }
    }

    pub fn isolate(id: usize, isolate: OwnedIsolate) -> &'a mut OwnedIsolate {
        Self::insert_isolate(id, isolate);
        Self::get_isolate(id).expect("something very weird jus happened...") //We can unwrap here because we just inserted it
    }

    pub fn handle_scope(id: usize, handle_scope: HandleScope<'static, ()>) -> &'a mut HandleScope<'static, ()> {
        Self::insert_handle_scope(id, handle_scope);
        Self::get_handle_scope(id).expect("something very weird jus happened...") //We can unwrap here because we just inserted it
    }

    pub fn context_scope(id: usize, context_scope: ContextScope<'a, HandleScope<'a>>) -> &'a mut ContextScope<'static, HandleScope<'a>> {
        Self::insert_context_scope(id, context_scope);
        Self::get_context_scope(id).expect("something very weird jus happened...") //We can unwrap here because we just inserted it
    }

    pub fn drop(id: usize) {
        //use correct drop order here. First context scope, then handle scope and at last the isolate, because they depend on each other
        unsafe {
            STORE.context_scopes.retain(|(i, _)| *i != id);
            STORE.handle_scopes.retain(|(i, _)| *i != id);
            STORE.isolates.retain(|(i, _)| *i != id);
        }
    }
}