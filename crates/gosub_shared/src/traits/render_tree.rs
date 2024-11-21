use std::collections::HashMap;
use crate::render_backend::layout::Layouter;
use crate::traits::config::{HasCssSystem, HasLayouter};
use crate::traits::css3::CssSystem;

pub trait RenderTree<C: HasLayouter>: Send + 'static {
    type NodeId: Copy;

    type Node: RenderTreeNode<C>;

    fn root(&self) -> Self::NodeId;

    fn get_node(&self, id: Self::NodeId) -> Option<&Self::Node>;

    fn get_node_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::Node>;

    fn get_children(&self, id: Self::NodeId) -> Option<Vec<Self::NodeId>>;
    
    fn get_layout(&self, id: Self::NodeId) -> Option<&<C::Layouter as Layouter>::Layout>;
    
    
}

pub trait RenderTreeNode<C: HasLayouter> {
    fn props(&self) -> &<C::CssSystem as CssSystem>::PropertyMap;

    fn props_mut(&mut self) -> &mut <C::CssSystem as CssSystem>::PropertyMap;
    
    fn layout(&self) ->  &<C::Layouter as Layouter>::Layout;
    
    fn element_attributes(&self) -> Option<&HashMap<String, String>>;
    fn name(&self) -> &str;
}
