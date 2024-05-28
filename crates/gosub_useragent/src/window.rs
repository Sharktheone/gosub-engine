use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;
use winit::window::Window as WinitWindow;

use gosub_render_backend::{RenderBackend, SizeU32};
use gosub_renderer::draw::SceneDrawer;

use crate::tabs::Tabs;

pub enum WindowState<B: RenderBackend> {
    Active {
        surface: B::ActiveWindowData<'_>,
        window: Arc<WinitWindow>,
    },
    Suspended(Arc<WinitWindow>),
}

pub struct Window<D: SceneDrawer<B>, B: RenderBackend> {
    pub(crate) state: WindowState<B>,
    pub(crate) renderer_data: B::WindowData<'_>,
    pub(crate) tabs: Tabs<D, B>,
}

impl<D: SceneDrawer<B>, B: RenderBackend> Window<D, B> {
    pub fn resumed(&mut self, backend: &mut B) {
        let WindowState::Suspended(window) = &self.state else {
            return;
        };

        let size = window.inner_size();
        let size = SizeU32::new(size.width, size.height);

        let data = backend.activate_window(window, &mut self.renderer_data, size);

        self.state = WindowState::Active {
            surface: data,
            window: window.clone(),
        };
    }

    pub fn suspended(&mut self, el: &ActiveEventLoop, backend: &mut B) {
        let WindowState::Active { surface, window } = &self.state else {
            return;
        };

        backend.suspend_window(window, surface, &mut self.renderer_data);

        self.state = WindowState::Suspended(window.clone());
    }
}
