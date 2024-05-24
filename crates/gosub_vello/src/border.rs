use crate::{Brush, VelloBackend};
use gosub_render_backend::{
    Border as TBorder, BorderRadius as TBorderRadius, BorderSide as TBorderSide, BorderStyle,
    RenderBackend, FP,
};

pub struct Border {}

// type BorderSide = <VelloBackend as gosub_render_backend::RenderBackend>::BorderSide;

impl TBorder<VelloBackend> for Border {
    fn new(all: BorderSide) -> Self {
        todo!()
    }

    fn all(left: BorderSide, right: BorderSide, top: BorderSide, bottom: BorderSide) -> Self {
        todo!()
    }

    fn left(&mut self, side: BorderSide) {
        todo!()
    }

    fn right(&mut self, side: BorderSide) {
        todo!()
    }

    fn top(&mut self, side: BorderSide) {
        todo!()
    }

    fn bottom(&mut self, side: BorderSide) {
        todo!()
    }
}

pub struct BorderSide {}

impl TBorderSide<VelloBackend> for BorderSide {
    fn new(width: FP, style: BorderStyle, brush: Brush) -> Self {
        todo!()
    }
}

pub struct BorderRadius {}

impl From<[FP; 4]> for BorderRadius {
    fn from(value: [FP; 4]) -> Self {
        todo!()
    }
}

impl From<[FP; 8]> for BorderRadius {
    fn from(value: [FP; 8]) -> Self {
        todo!()
    }
}

impl From<(FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP)) -> Self {
        todo!()
    }
}

impl From<(FP, FP, FP, FP, FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP, FP, FP, FP, FP)) -> Self {
        todo!()
    }
}

impl TBorderRadius for BorderRadius {
    fn empty() -> Self {
        todo!()
    }

    fn uniform(radius: FP) -> Self {
        todo!()
    }

    fn uniform_elliptical(radius_x: FP, radius_y: FP) -> Self {
        todo!()
    }

    fn top_left(&mut self, radius: FP) {
        todo!()
    }

    fn top_left_elliptical(&mut self, radius_x: FP, radius_y: FP) {
        todo!()
    }

    fn top_right(&mut self, radius: FP) {
        todo!()
    }

    fn top_right_elliptical(&mut self, radius_x: FP, radius_y: FP) {
        todo!()
    }

    fn bottom_left(&mut self, radius: FP) {
        todo!()
    }

    fn bottom_left_elliptical(&mut self, radius_x: FP, radius_y: FP) {
        todo!()
    }

    fn bottom_right(&mut self, radius: FP) {
        todo!()
    }

    fn bottom_right_elliptical(&mut self, radius_x: FP, radius_y: FP) {
        todo!()
    }

    fn build(self) -> Option<Self> {
        todo!()
    }
}
