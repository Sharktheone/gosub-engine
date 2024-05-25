use std::fmt::Debug;
use std::mem::offset_of;

use vello::kurbo::Point as VelloPoint;
use vello::Scene;

use gosub_render_backend::{Point, RenderBackend, RenderRect, RenderText};

mod border;
mod brush;
mod color;
mod gradient;
mod image;
mod rect;
mod text;
mod transform;

pub use border::*;
pub use brush::*;
pub use color::*;
pub use gradient::*;
pub use image::*;
pub use rect::*;
pub use text::*;
pub use transform::*;

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

    fn draw_rect(&mut self, rect: &RenderRect<Self>) {
        todo!()
    }

    fn draw_text(&mut self, text: &RenderText<Self>) {
        todo!()
    }

    fn reset(&mut self) {
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
