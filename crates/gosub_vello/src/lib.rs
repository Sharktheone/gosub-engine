use std::fmt::Debug;

use gosub_shared::types::Result;
use vello::kurbo::{Point as VelloPoint, RoundedRect, Shape};
use vello::peniko::Fill;
use vello::Scene;

use crate::render::window::{ActiveWindowData, WindowData};
pub use border::*;
pub use brush::*;
pub use color::*;
use gosub_render_backend::{Point, RenderBackend, RenderRect, RenderText, SizeU32, WindowHandle};
pub use gradient::*;
pub use image::*;
pub use rect::*;
pub use text::*;
pub use transform::*;

mod border;
mod brush;
mod color;
mod gradient;
mod image;
mod rect;
mod render;
mod text;
mod transform;

pub struct VelloBackend {
    scene: Scene,
}

impl Debug for VelloBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloRenderer").finish()
    }
}

impl RenderBackend for VelloBackend {
    type Rect = Rect;
    type Border = Border;
    type BorderSide = BorderSide;
    type BorderRadius = BorderRadius;
    type Transform = Transform;
    type PreRenderText = PreRenderText;
    type Text = Text;
    type Gradient = Gradient;
    type Color = Color;
    type Image = Image;
    type Brush = Brush;
    type ActiveWindowData<'a> = ActiveWindowData<'a>;
    type WindowData<'a> = WindowData;

    fn draw_rect(&mut self, rect: &RenderRect<Self>) {
        let affine = rect.transform.as_ref().map(|t| t.0).unwrap_or_default();

        let brush = &rect.brush.0;
        let brush_transform = rect.brush_transform.as_ref().map(|t| t.0);

        if let Some(radius) = &rect.radius {
            let shape = RoundedRect::from_rect(rect.rect.0, radius.clone());
            self.scene
                .fill(Fill::NonZero, affine, brush, brush_transform, &shape)
        } else {
            self.scene
                .fill(Fill::NonZero, affine, brush, brush_transform, &rect.rect.0)
        }

        if let Some(border) = &rect.border {
            let opts = BorderRenderOptions {
                border,
                rect: &rect.rect,
                transform: rect.transform.as_ref(),
                radius: rect.radius.as_ref(),
            };

            Border::draw(self, opts);
        }
    }

    fn draw_text(&mut self, text: &RenderText<Self>) {
        Text::show(self, text)
    }

    fn reset(&mut self) {
        self.scene.reset();
    }

    fn activate_window(
        &mut self,
        handle: impl WindowHandle,
        data: &mut Self::WindowData<'_>,
        size: SizeU32,
    ) -> Result<Self::ActiveWindowData<'_>> {
        let surface = data.adapter.create_surface(
            handle,
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
    }

    fn suspend_window(
        &mut self,
        handle: impl WindowHandle,
        data: Self::ActiveWindowData<'_>,
        window_data: &mut Self::WindowData<'_>,
    ) -> Result<()> {
        todo!()
    }

    fn resize_window(
        &mut self,
        window_data: &mut Self::WindowData<'_>,
        active_window_data: Self::ActiveWindowData<'_>,
        size: SizeU32,
    ) -> Result<()> {
        todo!()
    }

    fn render(
        &mut self,
        window_data: &Self::WindowData<'_>,
        active_data: &Self::ActiveWindowData<'_>,
    ) -> Result<()> {
        todo!()
    }
}

impl VelloBackend {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }
}

trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<VelloPoint> for Point {
    fn convert(self) -> VelloPoint {
        VelloPoint::new(self.x as f64, self.y as f64)
    }
}
