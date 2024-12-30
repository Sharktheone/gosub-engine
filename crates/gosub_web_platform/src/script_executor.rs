use anyhow::Result;
use gosub_interface::scripting::{Script, ScriptSource};
use gosub_net::http::fetcher::Fetcher;
use gosub_webexecutor::js::{JSContext, JSRuntime};
use std::sync::Arc;
use gosub_v8::{V8Context, V8Engine};
use crate::api;

pub struct ScriptExecutor {
    fetcher: Arc<Fetcher>,
}

impl ScriptExecutor {
    pub fn new(fetcher: Arc<Fetcher>) -> Self {
        Self { fetcher }
    }

    pub async fn run_script_on_ctx(&mut self, script: Script, ctx: &mut V8Context<'_>) -> Result<()> {
        let script_source = match script.source {
            ScriptSource::Inline(script) => script,
            ScriptSource::Src(url) => {
                let script = self.fetcher.get(&url).await?;

                String::from_utf8(script.body)?
            }
        };
        
        log::info!("Running script: {}", script_source);

        ctx.run(&script_source)?;

        Ok(())
    }
    
    pub async fn run_script(&mut self, script: Script) -> Result<()> {
        let mut engine = V8Engine::new();
        let mut ctx = engine.new_context()?;
        
        self.run_script_on_ctx(script, &mut ctx).await?;
        
        Ok(())
    }
    
    pub async fn run_scripts(&mut self, scripts: Vec<Script>) -> Result<()> {
        let mut engine = V8Engine::new();
        let mut ctx = engine.new_context()?;
        
        api::implement::<V8Engine>(ctx.clone())?;
        
        
        for script in scripts {
            self.run_script_on_ctx(script, &mut ctx).await?;
        }

        Ok(())
    }
}
