use gosub_render_backend::{
    Border as TBorder, BorderRadius as TBorderRadius, BorderSide as TBorderSide, BorderStyle,
    Radius, RenderBackend, FP,
};

use crate::{Brush, VelloBackend};

pub struct Border {
    pub(crate) left: BorderSide,
    pub(crate) right: BorderSide,
    pub(crate) top: BorderSide,
    pub(crate) bottom: BorderSide,
}

// type BorderSide = <VelloBackend as gosub_render_backend::RenderBackend>::BorderSide;

impl TBorder<VelloBackend> for Border {
    fn new(all: BorderSide) -> Self {
        Self {
            left: all.clone(),
            right: all.clone(),
            top: all.clone(),
            bottom: all,
        }
    }

    fn all(left: BorderSide, right: BorderSide, top: BorderSide, bottom: BorderSide) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    fn left(&mut self, side: BorderSide) {
        self.left = side;
    }

    fn right(&mut self, side: BorderSide) {
        self.right = side;
    }

    fn top(&mut self, side: BorderSide) {
        self.top = side;
    }

    fn bottom(&mut self, side: BorderSide) {
        self.bottom = side;
    }
}

#[derive(Clone)]
pub struct BorderSide {
    pub(crate) width: FP,
    pub(crate) style: BorderStyle,
    pub(crate) brush: Brush,
}

impl TBorderSide<VelloBackend> for BorderSide {
    fn new(width: FP, style: BorderStyle, brush: Brush) -> Self {
        Self {
            width,
            style,
            brush,
        }
    }
}

pub struct BorderRadius {
    pub(crate) top_left: Radius,
    pub(crate) top_right: Radius,
    pub(crate) bottom_left: Radius,
    pub(crate) bottom_right: Radius,
}

impl From<[FP; 4]> for BorderRadius {
    fn from(value: [FP; 4]) -> Self {
        Self {
            top_left: value[0].into(),
            top_right: value[1].into(),
            bottom_left: value[2].into(),
            bottom_right: value[3].into(),
        }
    }
}

impl From<[FP; 8]> for BorderRadius {
    fn from(value: [FP; 8]) -> Self {
        Self {
            top_left: (value[0], value[1]).into(),
            top_right: (value[2], value[3]).into(),
            bottom_left: (value[4], value[5]).into(),
            bottom_right: (value[6], value[7]).into(),
        }
    }
}

impl From<(FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP)) -> Self {
        Self {
            top_left: value.0.into(),
            top_right: value.1.into(),
            bottom_left: value.2.into(),
            bottom_right: value.3.into(),
        }
    }
}

impl From<(FP, FP, FP, FP, FP, FP, FP, FP)> for BorderRadius {
    fn from(value: (FP, FP, FP, FP, FP, FP, FP, FP)) -> Self {
        Self {
            top_left: (value.0, value.1).into(),
            top_right: (value.2, value.3).into(),
            bottom_left: (value.4, value.5).into(),
            bottom_right: (value.6, value.7).into(),
        }
    }
}

impl From<FP> for BorderRadius {
    fn from(value: FP) -> Self {
        Self {
            top_left: value.into(),
            top_right: value.into(),
            bottom_left: value.into(),
            bottom_right: value.into(),
        }
    }
}

impl From<Radius> for BorderRadius {
    fn from(value: Radius) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_left: value,
            bottom_right: value,
        }
    }
}

impl From<[Radius; 4]> for BorderRadius {
    fn from(value: [Radius; 4]) -> Self {
        Self {
            top_left: value[0],
            top_right: value[1],
            bottom_left: value[2],
            bottom_right: value[3],
        }
    }
}

impl From<(Radius, Radius, Radius, Radius)> for BorderRadius {
    fn from(value: (Radius, Radius, Radius, Radius)) -> Self {
        Self {
            top_left: value.0,
            top_right: value.1,
            bottom_left: value.2,
            bottom_right: value.3,
        }
    }
}

impl TBorderRadius for BorderRadius {
    fn uniform_radius(radius: Radius) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    fn all_radius(tl: Radius, tr: Radius, dl: Radius, dr: Radius) -> Self {
        Self {
            top_left: tl,
            top_right: tr,
            bottom_left: dl,
            bottom_right: dr,
        }
    }

    fn top_left_radius(&mut self, radius: Radius) {
        self.top_left = radius;
    }

    fn top_right_radius(&mut self, radius: Radius) {
        self.top_right = radius;
    }

    fn bottom_left_radius(&mut self, radius: Radius) {
        self.bottom_left = radius;
    }

    fn bottom_right_radius(&mut self, radius: Radius) {
        self.bottom_right = radius;
    }
}
