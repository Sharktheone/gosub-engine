use crate::traits::config::HasCssSystem;
use crate::render_backend::layout::{LayoutTree, Layouter};

pub trait HasLayouter: HasCssSystem {
    type Layouter: Layouter;
    type LayoutTree: LayoutTree<Self>;
}