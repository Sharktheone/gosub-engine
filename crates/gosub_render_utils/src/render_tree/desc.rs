use crate::render_tree::{RenderNodeData, RenderTree};
use gosub_shared::node::NodeId;
use gosub_shared::render_backend::layout::{Layout, LayoutTree};
use gosub_shared::render_backend::{NodeDesc, Point, Size};
use gosub_shared::traits::config::HasLayouter;
use gosub_shared::traits::css3::CssPropertyMap;

impl<C: HasLayouter<LayoutTree = Self>> RenderTree<C> {
    pub fn desc(&self) -> NodeDesc {
        self.desc_node(self.root)
    }

    pub fn desc_node(&self, node: NodeId) -> NodeDesc {
        let Some(node) = self.get_node(node) else {
            return NodeDesc {
                id: 0,
                name: "<unknown>".into(),
                children: vec![],
                attributes: vec![],
                properties: vec![],
                text: None,
                pos: Point::ZERO,
                size: Size::ZERO,
            };
        };

        let attributes = if let RenderNodeData::Element { attributes } = &node.data {
            attributes.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        } else {
            Vec::new()
        };

        let (name, text) = if let RenderNodeData::Text(t) = &node.data {
            ("#text".into(), Some(t.text.clone()))
        } else {
            (node.name.clone(), None)
        };

        NodeDesc {
            id: node.id.into(),
            name,
            children: node.children.iter().map(|child| self.desc_node(*child)).collect(),
            attributes,
            properties: node
                .properties
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_string()))
                .collect(),
            text,
            pos: node.layout.rel_pos(),
            size: node.layout.size(),
        }
    }
}
