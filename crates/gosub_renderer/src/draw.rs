use std::sync::mpsc::Sender;

use anyhow::anyhow;
use log::warn;
use url::Url;

use gosub_css3::colors::RgbColor;
use gosub_css3::stylesheet::CssValue;
use gosub_html5::node::NodeId;
use gosub_net::http::fetcher::Fetcher;
use gosub_render_backend::geo::{Size, SizeU32, FP};
use gosub_render_backend::layout::{Layout, LayoutTree, Layouter, TextLayout};
use gosub_render_backend::svg::SvgRenderer;
use gosub_render_backend::{
    Border, BorderSide, BorderStyle, Brush, Color, ImageBuffer, NodeDesc, Rect, RenderBackend,
    RenderBorder, RenderRect, RenderText, Scene as TScene, Text, Transform,
};
use gosub_rendering::position::PositionTree;
use gosub_shared::types::Result;
use gosub_styling::render_tree::{RenderNodeData, RenderTree, RenderTreeNode};

use crate::debug::scale::px_scale;
use crate::draw::img::request_img;
use crate::render_tree::{load_html_rendertree, TreeDrawer};

mod img;

pub trait SceneDrawer<B: RenderBackend, L: Layouter, LT: LayoutTree<L>> {
    fn draw(&mut self, backend: &mut B, data: &mut B::WindowData<'_>, size: SizeU32) -> bool;
    fn mouse_move(&mut self, backend: &mut B, x: FP, y: FP) -> bool;

    fn scroll(&mut self, point: Point);
    fn from_url(url: Url, layouter: L, debug: bool) -> Result<Self>
    where
        Self: Sized;

    fn clear_buffers(&mut self);
    fn toggle_debug(&mut self);

    fn select_element(&mut self, id: LT::NodeId);
    fn unselect_element(&mut self);

    fn send_nodes(&mut self, sender: Sender<NodeDesc>);

    fn set_needs_redraw(&mut self);
}

const DEBUG_CONTENT_COLOR: (u8, u8, u8) = (0, 192, 255); //rgb(0, 192, 255)
const DEBUG_PADDING_COLOR: (u8, u8, u8) = (0, 255, 192); //rgb(0, 255, 192)
const DEBUG_BORDER_COLOR: (u8, u8, u8) = (255, 72, 72); //rgb(255, 72, 72)
                                                        // const DEBUG_MARGIN_COLOR: (u8, u8, u8) = (255, 192, 0);

type Point = gosub_shared::types::Point<FP>;

impl<B: RenderBackend, L: Layouter> SceneDrawer<B, L, RenderTree<L>> for TreeDrawer<B, L>
where
    <<B as RenderBackend>::Text as Text>::Font:
        From<<<L as Layouter>::TextLayout as TextLayout>::Font>,
{
    fn draw(&mut self, backend: &mut B, data: &mut B::WindowData<'_>, size: SizeU32) -> bool {
        if !self.dirty && self.size == Some(size) {
            return false;
        }

        if self.tree_scene.is_none() || self.size != Some(size) {
            self.size = Some(size);

            let mut scene = B::Scene::new();

            // Apply new maximums to the scene transform
            if let Some(scene_transform) = self.scene_transform.as_mut() {
                let root_size = self.tree.get_root().layout.content();
                let max_x = root_size.width - size.width as f32;
                let max_y = root_size.height - size.height as f32;

                let x = scene_transform.tx().min(0.0).max(-max_x);
                let y = scene_transform.ty().min(0.0).max(-max_y);

                scene_transform.set_xy(x, y);
            }

            let mut drawer = Drawer {
                scene: &mut scene,
                drawer: self,
                svg: B::SVGRenderer::new(),
            };

            drawer.render(size);

            self.tree_scene = Some(scene);

            self.size = Some(size);
        }

        backend.reset(data);

        let bg = Rect::new(0.0, 0.0, size.width as FP, size.height as FP);

        let rect = RenderRect {
            rect: bg,
            transform: None,
            radius: None,
            brush: Brush::color(Color::BLACK),
            brush_transform: None,
            border: None,
        };
        //
        backend.draw_rect(data, &rect);

        if let Some(scene) = &self.tree_scene {
            backend.apply_scene(data, scene, self.scene_transform.clone());
        }

        if self.dirty {
            if let Some(id) = self.selected_element {
                self.debug_annotate(id);
            }
        }

        if let Some(scene) = &self.debugger_scene {
            self.dirty = false;
            backend.apply_scene(data, scene, self.scene_transform.clone());
        }

        if self.debug {
            let pos = self
                .scene_transform
                .as_ref()
                .map(|x| Point::new(x.tx(), x.ty()))
                .unwrap_or(Point::ZERO);

            let scale = px_scale::<B>(
                size,
                pos,
                self.size.as_ref().map(|x| x.width as f32).unwrap_or(0.0),
            );

            backend.apply_scene(data, &scale, None);
        }

        if self.dirty {
            self.dirty = false;

            return true;
        }

        false
    }

    fn mouse_move(&mut self, _backend: &mut B, x: FP, y: FP) -> bool {
        let x = x - self
            .scene_transform
            .clone()
            .unwrap_or(B::Transform::IDENTITY)
            .tx();
        let y = y - self
            .scene_transform
            .clone()
            .unwrap_or(B::Transform::IDENTITY)
            .ty();

        if let Some(e) = self.position.find(x, y) {
            if self.last_hover != Some(e) {
                self.last_hover = Some(e);
                if self.debug {
                    return self.debug_annotate(e);
                }
            }
            return false;
        };
        false
    }

    fn scroll(&mut self, point: Point) {
        let mut transform = self
            .scene_transform
            .take()
            .unwrap_or(B::Transform::IDENTITY);

        let x = transform.tx() + point.x;
        let y = transform.ty() + point.y;

        let root_size = self.tree.get_root().layout.content();
        let size = self.size.unwrap_or(SizeU32::ZERO);

        let max_x = root_size.width - size.width as f32;
        let max_y = root_size.height - size.height as f32;

        let x = x.min(0.0).max(-max_x);
        let y = y.min(0.0).max(-max_y);

        transform.set_xy(x, y);

        self.scene_transform = Some(transform);

        self.dirty = true;
    }

    fn from_url(url: Url, layouter: L, debug: bool) -> Result<Self> {
        let rt = load_html_rendertree(url.clone())?;

        Ok(Self::new(rt, layouter, url, debug))
    }

    fn clear_buffers(&mut self) {
        self.tree_scene = None;
        self.debugger_scene = None;
        self.last_hover = None;
        self.dirty = true;
    }

    fn toggle_debug(&mut self) {
        self.debug = !self.debug;
        self.dirty = true;
        self.last_hover = None;
        self.debugger_scene = None;
    }

    fn select_element(&mut self, id: NodeId) {
        self.selected_element = Some(id);
        self.dirty = true;
    }

    fn unselect_element(&mut self) {
        self.selected_element = None;
        self.debugger_scene = None;
        self.dirty = true;
    }

    fn send_nodes(&mut self, sender: Sender<NodeDesc>) {
        let _ = sender.send(self.tree.desc());
    }

    fn set_needs_redraw(&mut self) {
        self.dirty = true;
    }
}

struct Drawer<'s, 't, B: RenderBackend, L: Layouter> {
    scene: &'s mut B::Scene,
    drawer: &'t mut TreeDrawer<B, L>,
    svg: B::SVGRenderer,
}

impl<B: RenderBackend, L: Layouter> Drawer<'_, '_, B, L>
where
    <<B as RenderBackend>::Text as Text>::Font:
        From<<<L as Layouter>::TextLayout as TextLayout>::Font>,
{
    pub(crate) fn render(&mut self, size: SizeU32) {
        let root = self.drawer.tree.root;
        if let Err(e) = self
            .drawer
            .layouter
            .layout(&mut self.drawer.tree, root, size)
        {
            eprintln!("Failed to compute layout: {:?}", e);
            return;
        }

        // print_tree(&self.taffy, self.root, &self.style);

        self.drawer.position = PositionTree::from_tree::<B, L>(&self.drawer.tree);

        self.render_node_with_children(self.drawer.tree.root, Point::ZERO);
    }

    fn render_node_with_children(&mut self, id: NodeId, mut pos: Point) {
        let err = self.render_node(id, &mut pos);
        if let Err(e) = err {
            eprintln!("Error rendering node: {}", e);
        }

        let Some(children) = self.drawer.tree.children(id) else {
            eprintln!("Error rendering node children");
            return;
        };

        for child in children {
            self.render_node_with_children(child, pos);
        }
    }

    fn render_node(&mut self, id: NodeId, pos: &mut Point) -> anyhow::Result<()> {
        let mut needs_redraw = false;

        let node = self
            .drawer
            .tree
            .get_node_mut(id)
            .ok_or(anyhow!("Node {id} not found"))?;

        let p = node.layout.rel_pos();
        pos.x += p.x as FP;
        pos.y += p.y as FP;

        let (border_radius, redraw) =
            render_bg::<B, L>(node, self.scene, pos, &mut self.svg, &self.drawer.fetcher);

        needs_redraw |= redraw;

        if let RenderNodeData::Element(element) = &node.data {
            if element.name() == "img" {
                let src = element
                    .attributes
                    .get("src")
                    .ok_or(anyhow!("Image element has no src attribute"))?;

                let url = src.as_str();

                let size = node.layout.size_or().map(|x| x.u32());

                let img = request_img(&self.drawer.fetcher, &mut self.svg, url, size)?;

                if size.is_none() {
                    node.layout.set_size_and_content(img.size());
                    needs_redraw |= true;
                }

                let fit = element
                    .attributes
                    .get("object-fit")
                    .map(|prop| prop.as_str())
                    .unwrap_or("contain");

                let size = size.unwrap_or(img.size()).f32();

                render_image::<B>(img, self.scene, *pos, size, border_radius, fit)?;
            }
        }

        render_text::<B, L>(node, self.scene, pos);

        if needs_redraw {
            self.drawer.set_needs_redraw()
        }

        Ok(())
    }
}

fn render_text<B: RenderBackend, L: Layouter>(
    node: &mut RenderTreeNode<L>,
    scene: &mut B::Scene,
    pos: &Point,
) where
    <<B as RenderBackend>::Text as Text>::Font:
        From<<<L as Layouter>::TextLayout as TextLayout>::Font>,
{
    // if u64::from(node.id) < 204 && u64::from(node.id) > 202 {
    //     return;
    // }

    // if u64::from(node.id) == 203 {
    //     return;
    // }

    let color = node
        .properties
        .get("color")
        .and_then(|prop| {
            prop.compute_value();

            match &prop.actual {
                CssValue::Color(color) => Some(*color),
                CssValue::String(color) => Some(RgbColor::from(color.as_str())),
                _ => None,
            }
        })
        .map(|color| Color::rgba(color.r as u8, color.g as u8, color.b as u8, color.a as u8))
        .unwrap_or(Color::BLACK);

    if let RenderNodeData::Text(ref text) = node.data {
        let Some(layout) = text.layout.as_ref() else {
            warn!("No layout for text node");
            return;
        };

        let text: B::Text = Text::new::<L::TextLayout>(layout);

        let size = node.layout.size();

        let rect = Rect::new(
            pos.x as FP,
            pos.y as FP,
            size.width as FP,
            size.height as FP,
        );

        let render_text = RenderText {
            text,
            rect,
            transform: None,
            brush: Brush::color(color),
            brush_transform: None,
        };

        scene.draw_text(&render_text);
    }
}

fn render_bg<B: RenderBackend, L: Layouter>(
    node: &mut RenderTreeNode<L>,
    scene: &mut B::Scene,
    pos: &Point,
    svg: &mut B::SVGRenderer,
    fetcher: &Fetcher,
) -> ((FP, FP, FP, FP), bool) {
    let bg_color = node
        .properties
        .get("background-color")
        .and_then(|prop| {
            prop.compute_value();

            match &prop.actual {
                CssValue::Color(color) => Some(*color),
                CssValue::String(color) => Some(RgbColor::from(color.as_str())),
                _ => None,
            }
        })
        .map(|color| Color::rgba(color.r as u8, color.g as u8, color.b as u8, color.a as u8));

    let border_radius_left = node
        .properties
        .get("border-radius-left")
        .map(|prop| {
            prop.compute_value();
            prop.actual.unit_to_px() as f64
        })
        .unwrap_or(0.0);

    let border_radius_right = node
        .properties
        .get("border-radius-right")
        .map(|prop| {
            prop.compute_value();
            prop.actual.unit_to_px() as f64
        })
        .unwrap_or(0.0);

    let border_radius_top = node
        .properties
        .get("border-radius-top")
        .map(|prop| {
            prop.compute_value();
            prop.actual.unit_to_px() as f64
        })
        .unwrap_or(0.0);

    let border_radius_bottom = node
        .properties
        .get("border-radius-bottom")
        .map(|prop| {
            prop.compute_value();
            prop.actual.unit_to_px() as f64
        })
        .unwrap_or(0.0);

    let border_radius = (
        border_radius_top as FP,
        border_radius_right as FP,
        border_radius_bottom as FP,
        border_radius_left as FP,
    );

    let border = get_border::<B, L>(node).map(|border| RenderBorder::new(border));

    if let Some(bg_color) = bg_color {
        let size = node.layout.size();

        let rect = Rect::new(
            pos.x as FP,
            pos.y as FP,
            size.width as FP,
            size.height as FP,
        );

        let rect = RenderRect {
            rect,
            transform: None,
            radius: Some(B::BorderRadius::from(border_radius)),
            brush: Brush::color(bg_color),
            brush_transform: None,
            border,
        };

        scene.draw_rect(&rect);
    } else if let Some(border) = border {
        let size = node.layout.size();

        let rect = Rect::new(
            pos.x as FP,
            pos.y as FP,
            size.width as FP,
            size.height as FP,
        );

        let rect = RenderRect {
            rect,
            transform: None,
            radius: Some(B::BorderRadius::from(border_radius)),
            brush: Brush::color(Color::TRANSPARENT),
            brush_transform: None,
            border: Some(border),
        };

        scene.draw_rect(&rect);
    }

    let background_image = node.properties.get("background-image").and_then(|prop| {
        prop.compute_value();

        match &prop.actual {
            CssValue::String(url) => Some(url.as_str()),
            _ => None,
        }
    });

    let mut redraw = false;

    if let Some(url) = background_image {
        let size = node.layout.size_or().map(|x| x.u32());

        let img = match request_img(fetcher, svg, url, size) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error loading image: {:?}", e);
                return (border_radius, false);
            }
        };

        if size.is_none() {
            node.layout.set_size_and_content(img.size());

            redraw = true;
        }

        let _ = render_image::<B>(img, scene, *pos, node.layout.size(), border_radius, "fill")
            .map_err(|e| {
                eprintln!("Error rendering image: {:?}", e);
            });
    }

    (border_radius, redraw)
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn to_str(&self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Left => "left",
        }
    }
}

fn render_image<B: RenderBackend>(
    img: ImageBuffer<B>,
    scene: &mut B::Scene,
    pos: Point,
    size: Size,
    radii: (FP, FP, FP, FP),
    fit: &str,
) -> anyhow::Result<()> {
    let width = size.width as FP;
    let height = size.height as FP;

    let rect = Rect::new(pos.x, pos.y, pos.x + width, pos.y + height);

    let img_size = img.size_tuple();

    let transform = match fit {
        "fill" => {
            let scale_x = width / img_size.0;
            let scale_y = height / img_size.1;

            B::Transform::scale_xy(scale_x, scale_y)
        }
        "contain" => {
            let scale_x = width / img_size.0;
            let scale_y = height / img_size.1;

            let scale = scale_x.min(scale_y);

            Transform::scale(scale)
        }
        "cover" => {
            let scale_x = width / img_size.0;
            let scale_y = height / img_size.1;

            let scale = scale_x.max(scale_y);

            Transform::scale(scale)
        }
        "scale-down" => {
            let scale_x = width / img_size.0;
            let scale_y = height / img_size.1;

            let scale = scale_x.min(scale_y);
            let scale = scale.min(1.0);

            Transform::scale(scale)
        }
        _ => Transform::IDENTITY,
    };

    let transform = transform.with_translation(pos);

    match img {
        ImageBuffer::Image(img) => {
            let rect = RenderRect {
                rect,
                transform: None,
                radius: Some(B::BorderRadius::from(radii)),
                brush: Brush::image(img),
                brush_transform: Some(transform),
                border: None,
            };

            scene.draw_rect(&rect);
        }
        ImageBuffer::Scene(s, _size) => {
            scene.apply_scene(&s, Some(transform)); //TODO we probably want to use a clip layer here
        }
    }

    Ok(())
}

/*
//just for debugging
pub fn print_tree<B: RenderBackend, L: Layouter>(
    tree: &TaffyTree<GosubId>,
    root: NodeId,
    gosub_tree: &RenderTree<B>,
) {
    println!("TREE");
    print_node(tree, root, false, String::new(), gosub_tree);

    /// Recursive function that prints each node in the tree
    fn print_node<B: RenderBackend, L: Layouter>(
        tree: &TaffyTree<GosubId>,
        node_id: NodeId,
        has_sibling: bool,
        lines_string: String,
        gosub_tree: &RenderTree<B>,
    ) {
        let layout = &tree.get_final_layout(node_id);
        let display = tree.get_debug_label(node_id);
        let num_children = tree.child_count(node_id);
        let gosub_id = tree.get_node_context(node_id).unwrap();
        let width_style = tree.style(node_id).unwrap().size;

        let fork_string = if has_sibling {
            "├── "
        } else {
            "└── "
        };
        let node = gosub_tree.get_node(*gosub_id).unwrap();
        let mut node_render = String::new();

        match &node.data {
            RenderNodeData::Element(element) => {
                node_render.push('<');
                node_render.push_str(&element.name);
                for (key, value) in element.attributes.iter() {
                    node_render.push_str(&format!(" {}=\"{}\"", key, value));
                }
                node_render.push('>');
            }
            RenderNodeData::Text(text) => {
                let text = text.prerender.value().replace('\n', " ");
                node_render.push_str(text.trim());
            }

            _ => {}
        }

        println!(
            "{lines}{fork} {display} [x: {x:<4} y: {y:<4} width: {width:<4} height: {height:<4}] ({key:?}) |{node_render}|{width_style:?}|",
            lines = lines_string,
            fork = fork_string,
            display = display,
            x = layout.location.x,
            y = layout.location.y,
            width = layout.size.width,
            height = layout.size.height,
            key = node_id,
        );
        let bar = if has_sibling { "│   " } else { "    " };
        let new_string = lines_string + bar;

        // Recurse into children
        for (index, child) in tree.child_ids(node_id).enumerate() {
            let has_sibling = index < num_children - 1;
            print_node(tree, child, has_sibling, new_string.clone(), gosub_tree);
        }
    }
}
*/

fn get_border<B: RenderBackend, L: Layouter>(node: &mut RenderTreeNode<L>) -> Option<B::Border> {
    let left = get_border_side::<B, L>(node, Side::Left);
    let right = get_border_side::<B, L>(node, Side::Right);
    let top = get_border_side::<B, L>(node, Side::Top);
    let bottom = get_border_side::<B, L>(node, Side::Bottom);

    if left.is_none() && right.is_none() && top.is_none() && bottom.is_none() {
        return None;
    }

    let mut border = B::Border::empty();

    if let Some(left) = left {
        border.left(left)
    }

    if let Some(right) = right {
        border.right(right)
    }

    if let Some(top) = top {
        border.top(top)
    }

    if let Some(bottom) = bottom {
        border.bottom(bottom)
    }

    Some(border)
}

fn get_border_side<B: RenderBackend, L: Layouter>(
    node: &mut RenderTreeNode<L>,
    side: Side,
) -> Option<B::BorderSide> {
    let width = node
        .properties
        .get(&format!("border-{}-width", side.to_str()))
        .map(|prop| {
            prop.compute_value();
            prop.actual.unit_to_px()
        })?;

    let color = node
        .properties
        .get(&format!("border-{}-color", side.to_str()))
        .and_then(|prop| {
            prop.compute_value();

            match &prop.actual {
                CssValue::Color(color) => Some(*color),
                CssValue::String(color) => Some(RgbColor::from(color.as_str())),
                _ => None,
            }
        })?;

    let style = node
        .properties
        .get(&format!("border-{}-style", side.to_str()))
        .map(|prop| {
            prop.compute_value();
            prop.actual.to_string()
        })
        .unwrap_or("none".to_string());

    let style = BorderStyle::from_str(&style);

    let brush = Brush::color(Color::rgba(
        color.r as u8,
        color.g as u8,
        color.b as u8,
        color.a as u8,
    ));

    Some(BorderSide::new(width as FP, style, brush))
}

impl<B: RenderBackend, L: Layouter> TreeDrawer<B, L> {
    fn debug_annotate(&mut self, e: NodeId) -> bool {
        let Some(node) = self.tree.get_node(e) else {
            return false;
        };

        let mut scene = B::Scene::new();

        let Some(layout) = self.tree.get_layout(e) else {
            return false;
        };
        let size = layout.size();

        let padding = layout.padding();
        let border_size = layout.border();

        let Some((x, y)) = self.position.position(e) else {
            return false;
        };

        println!("Annotating: {:?}", node);
        println!("At: {:?} size: {size:?}", (x, y));

        let content_rect = Rect::new(x, y, size.width as FP, size.height as FP);

        let padding_brush = B::Brush::color(B::Color::tuple3(DEBUG_PADDING_COLOR).alpha(127));
        let content_brush = B::Brush::color(B::Color::tuple3(DEBUG_CONTENT_COLOR).alpha(127));
        // let margin_brush = B::Brush::color(B::Color::tuple3(DEBUG_MARGIN_COLOR).alpha(127));
        let border_brush = B::Brush::color(B::Color::tuple3(DEBUG_BORDER_COLOR).alpha(127));

        let mut border = B::Border::empty();

        border.left(BorderSide::new(
            padding.x2 as FP,
            BorderStyle::Solid,
            padding_brush.clone(),
        ));

        border.right(BorderSide::new(
            padding.x1 as FP,
            BorderStyle::Solid,
            padding_brush.clone(),
        ));

        border.top(BorderSide::new(
            padding.y1 as FP,
            BorderStyle::Solid,
            padding_brush.clone(),
        ));

        border.bottom(BorderSide::new(
            padding.y2 as FP,
            BorderStyle::Solid,
            padding_brush,
        ));

        let padding_border = RenderBorder::new(border);

        let render_rect = RenderRect {
            rect: content_rect,
            transform: None,
            radius: None,
            brush: content_brush,
            brush_transform: None,
            border: Some(padding_border),
        };

        scene.draw_rect(&render_rect);

        let mut border_border = B::Border::empty();

        border_border.left(BorderSide::new(
            border_size.x2 as FP,
            BorderStyle::Solid,
            border_brush.clone(),
        ));

        border_border.right(BorderSide::new(
            border_size.x1 as FP,
            BorderStyle::Solid,
            border_brush.clone(),
        ));

        border_border.top(BorderSide::new(
            border_size.y1 as FP,
            BorderStyle::Solid,
            border_brush.clone(),
        ));

        border_border.bottom(BorderSide::new(
            border_size.y2 as FP,
            BorderStyle::Solid,
            border_brush,
        ));

        let border_border = RenderBorder::new(border_border);

        let border_rect = Rect::new(
            x as FP - border_size.x2 as FP - padding.x2 as FP,
            y as FP - border_size.y1 as FP - padding.y1 as FP,
            (size.width + padding.x2 + padding.x1) as FP,
            (size.height + padding.y1 + padding.y2) as FP,
        );

        let render_rect = RenderRect {
            rect: border_rect,
            transform: None,
            radius: None,
            brush: Brush::color(Color::TRANSPARENT),
            brush_transform: None,
            border: Some(border_border),
        };

        scene.draw_rect(&render_rect);

        self.debugger_scene = Some(scene);
        self.dirty = true;

        true
    }
}
