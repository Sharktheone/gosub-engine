use crate::traits::css3::CssSystem;

pub trait HasCssSystem: Sized {
    type CssSystem: CssSystem;
}