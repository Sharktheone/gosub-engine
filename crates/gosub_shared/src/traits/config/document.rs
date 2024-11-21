use crate::traits::config::css_system::HasCssSystem;
use crate::traits::document::Document;

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self>;
}