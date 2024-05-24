use crate::VelloBackend;
use gosub_render_backend::{ColorStops, Gradient as TGradient, Point, FP};
use vello::peniko::Gradient as VelloGradient;

pub struct Gradient(pub(crate) VelloGradient);

impl From<VelloGradient> for Gradient {
    fn from(gradient: VelloGradient) -> Self {
        Gradient(gradient)
    }
}

impl TGradient<VelloBackend> for Gradient {
    fn new_linear(start: Point, end: Point, stops: ColorStops<VelloBackend>) -> Self {
        todo!()
    }

    fn new_radial(
        start_center: Point,
        start_radius: FP,
        end_center: Point,
        end_radius: FP,
        stops: ColorStops<VelloBackend>,
    ) -> Self {
        todo!()
    }

    fn new_sweep(
        center: Point,
        start_angle: FP,
        end_angle: FP,
        stops: ColorStops<VelloBackend>,
    ) -> Self {
        todo!()
    }
}
