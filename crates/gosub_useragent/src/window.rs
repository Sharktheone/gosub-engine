use std::sync::Arc;

use anyhow::anyhow;
use log::warn;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window as WinitWindow, WindowId};

use gosub_render_backend::{RenderBackend, SizeU32};
use gosub_renderer::draw::SceneDrawer;
use gosub_shared::types::Result;
use url::Url;

use crate::tabs::Tabs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState<'a, B: RenderBackend> {
    Active { surface: B::ActiveWindowData<'a> },
    Suspended,
}

pub struct Window<'a, D: SceneDrawer<B>, B: RenderBackend> {
    pub(crate) state: WindowState<'a, B>,
    pub(crate) window: Arc<WinitWindow>,
    pub(crate) renderer_data: B::WindowData<'a>,
    pub(crate) tabs: Tabs<D, B>,
}

impl<'a, D: SceneDrawer<B>, B: RenderBackend> Window<'a, D, B> {
    pub fn new(event_loop: &ActiveEventLoop, backend: &mut B, default_url: Url) -> Result<Self> {
        let window = create_window(event_loop)?;

        let renderer_data = backend.create_window_data(window.clone())?;

        Ok(Self {
            state: WindowState::Suspended,
            window,
            renderer_data,
            tabs: Tabs::from_url(default_url)?,
        })
    }

    pub fn resumed(&mut self, backend: &mut B) -> Result<()> {
        println!("Resuming window...");
        if !matches!(self.state, WindowState::Suspended) {
            return Ok(());
        };
        println!("Resuming window");

        let size = self.window.inner_size();
        let size = SizeU32::new(size.width, size.height);

        let data = backend.activate_window(self.window.clone(), &mut self.renderer_data, size)?;

        self.state = WindowState::Active { surface: data };

        Ok(())
    }

    pub fn suspended(&mut self, _el: &ActiveEventLoop, backend: &mut B) {
        let WindowState::Active { surface: data } = &mut self.state else {
            return;
        };

        if let Err(e) = backend.suspend_window(self.window.clone(), data, &mut self.renderer_data) {
            warn!("Failed to suspend window: {}", e);
        }

        self.state = WindowState::Suspended;
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn state(&self) -> &'static str {
        match self.state {
            WindowState::Active { .. } => "Active",
            WindowState::Suspended => "Suspended",
        }
    }
}

fn create_window(event_loop: &ActiveEventLoop) -> Result<Arc<WinitWindow>> {
    let attributes = WinitWindow::default_attributes()
        .with_title("Gosub Browser")
        .with_inner_size(LogicalSize::new(1920, 1080));

    event_loop
        .create_window(attributes)
        .map_err(|e| anyhow!(e.to_string()))
        .map(Arc::new)
}
