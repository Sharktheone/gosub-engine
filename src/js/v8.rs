use std::sync::atomic::{AtomicBool, Ordering};
use v8;


static PLATFORM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct V8Engine;

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