use anyhow::bail;
use gosub_net::http::fetcher::Fetcher;
use gosub_rendering::render_tree::{generate_render_tree, RenderTree};
use gosub_shared::byte_stream::{ByteStream, Encoding};
use gosub_shared::document::DocumentHandle;
use gosub_shared::render_backend::layout::Layouter;
use gosub_shared::render_backend::ImgCache;
use gosub_shared::traits::css3::CssSystem;
use gosub_shared::traits::document::{Document, DocumentBuilder};
use gosub_shared::traits::html5::Html5Parser;
use std::fs;
use url::Url;
use gosub_shared::traits::config::{HasCssSystem, HasHtmlParser, HasLayouter};

pub(crate) async fn load_html_rendertree<C: HasLayouter + HasHtmlParser>(
    url: Url,
) -> gosub_shared::types::Result<(RenderTree<C>, Fetcher)> {
    let fetcher = Fetcher::new(url.clone());

    let rt = load_html_rendertree_fetcher::<C>(url, &fetcher).await?;

    Ok((rt, fetcher))
}

pub(crate) async fn load_html_rendertree_fetcher<C: HasLayouter + HasHtmlParser>(
    url: Url,
    fetcher: &Fetcher,
) -> gosub_shared::types::Result<RenderTree<C>> {
    let html = if url.scheme() == "http" || url.scheme() == "https" {
        // Fetch the html from the url
        let response = fetcher.get(url.as_ref()).await?;
        if response.status != 200 {
            bail!(format!("Could not get url. Status code {}", response.status));
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

    let mut doc_handle = C::DocumentBuilder::new_document(Some(url));
    let parse_errors = C::HtmlParser::parse(&mut stream, DocumentHandle::clone(&doc_handle), None)?;

    for error in parse_errors {
        eprintln!("Parse error: {:?}", error);
    }

    let mut doc = doc_handle.get_mut();

    doc.add_stylesheet(C::CssSystem::load_default_useragent_stylesheet());

    drop(doc);

    generate_render_tree(DocumentHandle::clone(&doc_handle))
}
