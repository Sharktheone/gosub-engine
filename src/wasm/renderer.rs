use js_sys::Promise;
use url::Url;
use wasm_bindgen::prelude::*;

use gosub_renderer::render_tree::TreeDrawer;
use gosub_shared::types::Result;
use gosub_styling::render_tree::RenderTree;
use gosub_taffy::TaffyLayouter;
use gosub_useragent::application::Application;
use gosub_vello::VelloBackend;

type Backend = VelloBackend;
type Layouter = TaffyLayouter;
type Drawer = TreeDrawer<Backend, Layouter>;
type Tree = RenderTree<Layouter>;

#[wasm_bindgen]
pub struct RendererOptions {
    id: String,
    html: String,
    url: String,
    debug: bool,
}

#[wasm_bindgen]
impl RendererOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, html: String, url: String, debug: bool) -> Self {
        Self {
            id,
            html,
            url,
            debug,
        }
    }
}

#[wasm_bindgen]
pub struct RendererOutput {
    successful: bool,
    errors: String,
    promise: Promise,
}

#[wasm_bindgen]
impl RendererOutput {
    pub fn is_successful(&self) -> bool {
        self.successful
    }

    pub fn get_errors(&self) -> String {
        self.errors.clone()
    }

    pub fn get_promise(&self) -> Promise {
        self.promise.clone()
    }
}

impl RendererOutput {
    pub fn ok(promise: Promise) -> Self {
        Self {
            successful: true,
            errors: String::new(),
            promise,
        }
    }
}

#[wasm_bindgen]
pub fn renderer(opts: RendererOptions) -> RendererOutput {
    let promise = wasm_bindgen_futures::future_to_promise(async {
        if let Err(e) = renderer_internal(opts).await {
            return Err(JsValue::from_str(&format!("{}", e)));
        };
        Ok(JsValue::NULL)
    });

    RendererOutput::ok(promise)
}

async fn renderer_internal(opts: RendererOptions) -> Result<()> {
    let mut application: Application<Drawer, Backend, Layouter, Tree> =
        Application::new(VelloBackend::new(), TaffyLayouter, opts.debug);

    application.initial_tab(Url::parse(&opts.url)?);

    application.initialize()?;

    application.run()?;

    Ok(())
}
