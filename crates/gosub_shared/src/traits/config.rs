mod css_system;
mod document;
mod layouter;
mod render;

pub use css_system::*;
pub use document::*;
pub use layouter::*;
pub use render::*;

pub trait ModuleConfiguration:
Sized
+ HasCssSystem
+ HasDocument
+ HasHtmlParser
+ HasLayouter
+ HasRenderTree
+ HasTreeDrawer
+ HasRenderBackend
{}