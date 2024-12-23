use gosub_shared::async_executor::WasmNotSendSync;
use gosub_shared::geo::SizeU32;
use crate::config::{HasDrawComponents, HasRenderBackend, HasRenderTree};
use crate::render_backend::ImageBuffer;

pub trait EventLoopHandle<C: HasDrawComponents>: WasmNotSendSync + Clone + 'static {
    fn redraw(&self);

    fn add_img_cache(&self, url: String, buf: ImageBuffer<C::RenderBackend>, size: Option<SizeU32>);

    fn reload_from(&self, rt: C::RenderTree);
}

