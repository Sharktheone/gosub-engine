use std::fs;
use std::sync::{Arc, Mutex};
use anyhow::bail;
use url::Url;

use gosub_html5::node::NodeId;
use gosub_html5::parser::document::{Document, DocumentBuilder};
use gosub_html5::parser::Html5Parser;
use gosub_net::http::fetcher::Fetcher;
use gosub_render_backend::geo::SizeU32;
use gosub_render_backend::layout::Layouter;
use gosub_render_backend::{ImgCache, RenderBackend};
use gosub_rendering::position::PositionTree;
use gosub_shared::byte_stream::{ByteStream, Encoding};
use gosub_styling::render_tree::{generate_render_tree, RenderNodeData, RenderTree};
use gosub_styling::styling::CssProperties;
use crate::draw::img_cache::ImageCache;

#[derive(Debug)]
pub struct TreeDrawer<B: RenderBackend, L: Layouter> {
    pub(crate) tree: RenderTree<L>,
    pub(crate) fetcher: Arc<Fetcher>,
    pub(crate) layouter: L,
    pub(crate) size: Option<SizeU32>,
    pub(crate) position: PositionTree,
    pub(crate) last_hover: Option<NodeId>,
    pub(crate) debug: bool,
    pub(crate) dirty: bool,
    pub(crate) debugger_scene: Option<B::Scene>,
    pub(crate) tree_scene: Option<B::Scene>,
    pub(crate) selected_element: Option<NodeId>,
    pub(crate) scene_transform: Option<B::Transform>,
    pub(crate) img_cache: Arc<Mutex<ImageCache<B>>>,
}

impl<B: RenderBackend, L: Layouter> TreeDrawer<B, L> {
    pub fn new(tree: RenderTree<L>, layouter: L, fetcher: Fetcher, debug: bool) -> Self {
        Self {
            tree,
            fetcher: Arc::new(fetcher),
            layouter,
            size: None,
            position: PositionTree::default(),
            last_hover: None,
            debug,
            debugger_scene: None,
            dirty: false,
            tree_scene: None,
            selected_element: None,
            scene_transform: None,
            img_cache: Arc::new(Mutex::new(ImageCache::new())),
        }
    }
}

pub struct RenderTreeNode<L: Layouter> {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub layout: i32, //TODO
    pub name: String,
    pub properties: CssProperties,
    pub namespace: Option<String>,
    pub data: RenderNodeData<L>,
}

pub(crate) async fn load_html_rendertree<L: Layouter>(
    url: Url,
) -> gosub_shared::types::Result<(RenderTree<L>, Fetcher)> {
    let fetcher = Fetcher::new(url.clone());

    let html = if url.scheme() == "http" || url.scheme() == "https" {
        // Fetch the html from the url
        let response = fetcher.get(url.as_ref()).await?;
        if response.status != 200 {
            bail!(format!(
                "Could not get url. Status code {}",
                response.status
            ));
        }

        String::from_utf8(response.body.clone())?
    } else if url.scheme() == "file" {
        fs::read_to_string(url.as_str().trim_start_matches("file://"))?
    } else {
        bail!("Unsupported url scheme: {}", url.scheme());
    };

    let mut stream = ByteStream::new(Encoding::UTF8, None);
    stream.read_from_str(&html, Some(Encoding::UTF8));
    stream.close();

    let mut doc_handle = DocumentBuilder::new_document(Some(url));
    let parse_errors =
        Html5Parser::parse_document(&mut stream, Document::clone(&doc_handle), None)?;

    for error in parse_errors {
        eprintln!("Parse error: {:?}", error);
    }

    let mut doc = doc_handle.get_mut();
    doc.stylesheets
        .push(gosub_styling::load_default_useragent_stylesheet()?);

    drop(doc);

    Ok((generate_render_tree(Document::clone(&doc_handle))?, fetcher))
}
