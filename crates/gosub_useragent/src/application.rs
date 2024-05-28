use std::collections::HashMap;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use gosub_render_backend::RenderBackend;
use gosub_renderer::draw::SceneDrawer;

use crate::window::Window;

struct Application<D: SceneDrawer<B>, B: RenderBackend> {
    windows: HashMap<WindowId, Window<D, B>>,
    backend: B,
}

impl<D: SceneDrawer<B>, B: RenderBackend> ApplicationHandler for Application<D, B> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        for window in self.windows.values_mut() {
            window.resumed(&mut self.backend);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.event(event_loop, &mut self.backend, event);
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        for window in self.windows.values_mut() {
            window.suspended(event_loop, &mut self.backend);
        }
    }
}
