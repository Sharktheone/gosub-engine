use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use gosub_render_backend::{RenderBackend, SizeU32};
use gosub_renderer::draw::SceneDrawer;

use crate::tabs::Tab;
use crate::window::{Window, WindowState};

impl<D: SceneDrawer<B>, B: RenderBackend> Window<D, B> {
    pub fn event(&mut self, el: &ActiveEventLoop, backend: &mut B, event: WindowEvent) {
        let WindowState::Active {
            surface: active_window_data,
            window,
        } = &self.state
        else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                el.exit();
            }
            WindowEvent::Resized(size) => {
                backend.resize_window(
                    &mut self.renderer_data,
                    active_window_data,
                    SizeU32::new(size.width, size.height),
                );
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();

                let Some(tab) = self.get_current_tab() else {
                    return;
                };

                tab.data.draw(&mut self.renderer_data, size);

                backend.render(&mut self.renderer_data, active_window_data);
            }

            WindowEvent::CursorMoved { position, .. } => {
                let Some(tab) = self.get_current_tab() else {
                    return;
                };

                tab.data
                    .mouse_move(&mut self.renderer_data, position.x, position.y);
            }

            _ => {}
        }
    }

    fn get_current_tab(&mut self) -> Option<&mut Tab<D, B>> {
        let Some(tab) = self.tabs.tabs.get_mut(self.tabs.active.0) else {
            if let Some(first) = self.tabs.tabs.keys().next() {
                self.tabs.active = *first;
            }

            return None;
        };

        Some(tab)
    }
}
