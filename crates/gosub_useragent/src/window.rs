use gosub_render_backend::RenderBackend;
use gosub_renderer::draw::SceneDrawer;
use std::sync::Arc;
use winit::window::Window as WinitWindow;

pub enum WindowState<B: RenderBackend> {
    Active {
        surface: B::ActiveWindowData,
        window: Arc<WinitWindow>,
    },
    Suspended(Arc<WinitWindow>),
}

pub struct Window<D: SceneDrawer<B>, B: RenderBackend> {
    state: WindowState<B>,
    scene_drawer: D,
    renderer_data: B::WindowData,
}
