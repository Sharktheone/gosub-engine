mod css_system;
mod document;



pub use css_system::*;
pub use document::*;

pub trait ModuleConfiguration:
Sized
+ HasCssSystem
+ HasDocument
+ HasCssParser
+ HasHtmlParser
+ HasLayouter
+ HasRenderTree
+ HasTreeDrawer
+ HasRenderBackend
{
}