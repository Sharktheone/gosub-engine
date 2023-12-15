use std::marker::PhantomData;

use v8::{ContextScope, HandleScope, OwnedIsolate};

pub(super) struct Store<'a> {
    isolates: Vec<Inner<OwnedIsolate>>,
    handle_scopes: Vec<Inner<HandleScope<'a, ()>>>,
    context_scopes: Vec<Inner<ContextScope<'a, HandleScope<'a>>>>,
    _marker: PhantomData<*mut ()>,
}

struct Inner<T> {
    id: usize,
    ref_count: usize,
    drop_next: bool,
    data: T,
}

impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        if self.ref_count != 0 {
            panic!("Cannot drop v8 context value because it is still in use"); //TODO: Give a way to recover from this
        }
    }
}

impl<T> Inner<T> {
    fn new(id: usize, data: T) -> Self {
        Self {
            id,
            ref_count: 0,
            drop_next: false,
            data,
        }
    }
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
        self.isolates.push(Inner {
            id,
            ref_count: 0,
            drop_next: false,
            data: isolate,
        });
    }

    #[inline(always)]
    fn _insert_handle_scope(&mut self, id: usize, handle_scope: HandleScope<'a, ()>) {
        self.handle_scopes.push(Inner::new(id, handle_scope));
    }

    fn _insert_context_scope(&mut self, id: usize, context_scope: ContextScope<'a, HandleScope<'a>>) {
        self.context_scopes.push(Inner::new(id, context_scope));
    }

    fn _get_isolate(&'a mut self, id: usize) -> Option<&'a mut OwnedIsolate> {
        self.isolates.iter_mut().find_map(|inner| {
            if inner.id == id {
                Some(&mut inner.data)
            } else {
                None
            }
        })
    }

    fn _get_handle_scope(&'a mut self, id: usize) -> Option<&'static mut HandleScope<'a, ()>> {
        self.handle_scopes.iter_mut().find_map(|inner| {
            if inner.id == id {
                Some(&mut inner.data)
            } else {
                None
            }
        })
    }

    fn _get_context_scope(&'a mut self, id: usize) -> Option<&'static mut ContextScope<'a, HandleScope<'a>>> {
        self.context_scopes.iter_mut().find_map(|inner| {
            if inner.id == id {
                Some(&mut inner.data)
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
        Self::get_isolate(id).expect("something very weird just happened...") //We can unwrap here because we just inserted it
    }

    pub fn handle_scope(id: usize, handle_scope: HandleScope<'static, ()>) -> &'a mut HandleScope<'static, ()> {
        Self::insert_handle_scope(id, handle_scope);
        Self::get_handle_scope(id).expect("something very weird just happened...") //We can unwrap here because we just inserted it
    }

    pub fn context_scope(id: usize, context_scope: ContextScope<'a, HandleScope<'a>>) -> &'static mut ContextScope<'static, HandleScope<'a>> {
        Self::insert_context_scope(id, context_scope);
        Self::get_context_scope(id).expect("something very weird just happened...") //We can unwrap here because we just inserted it
    }

    pub fn drop(id: usize) {
        //use correct drop order here. First context scope, then handle scope and at last the isolate, because they depend on each other
        let mut dont_drop = false;

        unsafe {
            let context_scope = STORE.context_scopes.iter_mut().enumerate().find_map(|(i, inner)| {
                if inner.id == id {
                    Some((i, inner))
                } else {
                    None
                }
            });

            let handle_scope = STORE.handle_scopes.iter_mut().enumerate().find_map(|(i, inner)| {
                if inner.id == id {
                    Some((i, inner))
                } else {
                    None
                }
            });

            let isolate = STORE.isolates.iter_mut().enumerate().find_map(|(i, inner)| {
                if inner.id == id {
                    Some((i, inner))
                } else {
                    None
                }
            });

            if let Some((_, inner)) = &context_scope {
                if inner.ref_count != 0 {
                    dont_drop = true;
                }
            }

            if let Some((_, inner)) = &handle_scope {
                if inner.ref_count != 0 {
                    dont_drop = true;
                }
            }

            if let Some((_, inner)) = &isolate {
                if inner.ref_count != 0 {
                    dont_drop = true;
                }
            }

            if !dont_drop {
                if let Some((i, _)) = context_scope {
                    STORE.context_scopes.remove(i);
                }

                if let Some((i, _)) = handle_scope {
                    STORE.handle_scopes.remove(i);
                }

                if let Some((i, _)) = isolate {
                    STORE.isolates.remove(i);
                }
            } else {
                if let Some((_, inner)) = context_scope {
                    inner.drop_next = true;
                }

                if let Some((_, inner)) = handle_scope {
                    inner.drop_next = true;
                }

                if let Some((_, inner)) = isolate {
                    inner.drop_next = true;
                }
            }
        }
    }

    fn change_context_scope_count(id: usize, increase: bool) {
        unsafe {
            for inner in STORE.context_scopes.iter_mut() {
                if inner.id == id {
                    if increase {
                        inner.ref_count += 1;
                    } else {
                        inner.ref_count -= 1;
                    }
                    break;
                }
            }
        }
    }

    fn change_handle_scope_count(id: usize) {
        unsafe {
            for inner in STORE.handle_scopes.iter_mut() {
                if inner.id == id {
                    inner.ref_count -= 1;
                    break;
                }
            }
        }
    }

    fn change_isolate_count(id: usize) {
        unsafe {
            for inner in STORE.isolates.iter_mut() {
                if inner.id == id {
                    inner.ref_count -= 1;
                    break;
                }
            }
        }
    }

    pub(super) fn raise_context_scope_count(id: usize) {
        Self::change_context_scope_count(id, true);
    }

    pub(super) fn lower_context_scope_count(id: usize) {
        Self::change_context_scope_count(id, false);
    }

    pub(super) fn raise_handle_scope_count(id: usize) {
        Self::change_handle_scope_count(id);
    }

    pub(super) fn lower_handle_scope_count(id: usize) {
        Self::change_handle_scope_count(id);
    }

    pub(super) fn raise_isolate_count(id: usize) {
        Self::change_isolate_count(id);
    }

    pub(super) fn lower_isolate_count(id: usize) {
        Self::change_isolate_count(id);
    }
}