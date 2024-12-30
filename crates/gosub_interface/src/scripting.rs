use bitflags::bitflags;
use url::Url;

pub struct Script {
    pub source: ScriptSource,
    pub flags: ScriptFlags,
    // pub lang: String,
}

impl Script {
    pub fn new_inline(script: String, flags: ScriptFlags) -> Self {
        Self {
            source: ScriptSource::Inline(script),
            flags,
        }
    }
    
    pub fn new_url(url: String, flags: ScriptFlags) -> Self {
        Self {
            source: ScriptSource::Src(url),
            flags,
        }
    }
}

pub enum ScriptSource {
    Src(String),
    Inline(String),
}

bitflags! {
    pub struct ScriptFlags: u8 {
        const MODULE = 1<<0;
        const ASYNC = 1<<1;
        const DEFER = 1<<2;
    }
}
