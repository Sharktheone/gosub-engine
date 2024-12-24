use gosub_shared::geo::SizeU32;
use crate::config::HasRenderBackend;
use crate::instance::InstanceId;
use crate::render_backend::RenderBackend;




pub trait ChromeHandle<C: HasRenderBackend>: Send + Clone {
    type WindowId;
    
    fn draw_scene(&self, scene: <C::RenderBackend as RenderBackend>::Scene, size: SizeU32, instance: InstanceId);
    
    fn set_window(&mut self, window_id: Self::WindowId);
}



