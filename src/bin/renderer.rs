use std::sync::mpsc;
use std::{io, thread};

use clap::ArgAction;
use gosub_css3::system::Css3System;
use gosub_html5::document::builder::DocumentBuilderImpl;
use gosub_html5::document::document_impl::DocumentImpl;
use gosub_html5::document::fragment::DocumentFragmentImpl;
use gosub_html5::parser::Html5Parser;
use gosub_renderer::draw::TreeDrawerImpl;
use gosub_rendering::render_tree::RenderTree;
use gosub_shared::traits::config::{
    HasCssSystem, HasDocument, HasHtmlParser, HasLayouter, HasRenderBackend, HasRenderTree, HasTreeDrawer,
    ModuleConfiguration,
};
use gosub_shared::types::Result;
use gosub_taffy::TaffyLayouter;
use gosub_useragent::application::{Application, CustomEventInternal, WindowOptions};
use gosub_useragent::winit::window::WindowId;
use gosub_vello::VelloBackend;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use url::Url;

#[derive(Clone, Debug, PartialEq)]
struct Config;

impl HasCssSystem for Config {
    type CssSystem = Css3System;
}
impl HasDocument for Config {
    type Document = DocumentImpl<Self>;
    type DocumentFragment = DocumentFragmentImpl<Self>;
    type DocumentBuilder = DocumentBuilderImpl;
}

impl HasHtmlParser for Config {
    type HtmlParser = Html5Parser<'static, Self>;
}

impl HasLayouter for Config {
    type Layouter = TaffyLayouter;
    type LayoutTree = RenderTree<Self>;
}

impl HasRenderTree for Config {
    type RenderTree = RenderTree<Self>;
}

impl HasTreeDrawer for Config {
    type TreeDrawer = TreeDrawerImpl<Self>;
}

impl HasRenderBackend for Config {
    type RenderBackend = VelloBackend;
}

impl ModuleConfiguration for Config {}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("wgpu_hal", LevelFilter::Warn)
        .with_module_level("wgpu_core", LevelFilter::Warn)
        .init()?;

    let matches = clap::Command::new("Gosub Renderer")
        .arg(
            clap::Arg::new("url")
                .help("The url or file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let url: String = matches.get_one::<String>("url").expect("url").to_string();
    let debug = matches.get_one::<bool>("debug").copied().unwrap_or(false);

    // let drawer: TreeDrawer<Tree, TaffyLayouter> = TreeDrawer::new(todo!(), TaffyLayouter, "".to_string().into(), debug);

    // let mut rt = load_html_rendertree(&url)?;
    //
    let mut application: Application<Config> = Application::new(VelloBackend::new(), TaffyLayouter, debug);

    application.initial_tab(Url::parse(&url)?, WindowOptions::default());

    //this will initialize the application
    let p = application.proxy()?;

    thread::spawn(move || {
        let mut window = None;

        loop {
            let mut input = String::new();
            if let Err(e) = io::stdin().read_line(&mut input) {
                eprintln!("Error reading input: {e:?}");
                continue;
            };

            let input = input.trim();

            match input {
                "list" => {
                    let (sender, receiver) = mpsc::channel();

                    if let Err(e) = p.send_event(CustomEventInternal::SendNodes(sender)) {
                        eprintln!("Error sending event: {e:?}");
                        continue;
                    }

                    let node = match receiver.recv() {
                        Ok(node) => node,
                        Err(e) => {
                            eprintln!("Error receiving node: {e:?}");
                            continue;
                        }
                    };

                    println!("{}", node);
                }

                "add" => {
                    if let Err(e) = p.send_event(CustomEventInternal::Select(u64::MAX)) {
                        eprintln!("Error sending event: {e:?}");
                    }
                }
                "unselect" => {
                    if let Err(e) = p.send_event(CustomEventInternal::Unselect) {
                        eprintln!("Error sending event: {e:?}");
                    }
                }

                _ => {}
            }

            if input.starts_with("select ") {
                let id = input.trim_start_matches("select ");
                let Ok(id) = id.parse::<u64>() else {
                    eprintln!("Invalid id: {id}");
                    continue;
                };

                if let Err(e) = p.send_event(CustomEventInternal::Select(id)) {
                    eprintln!("Error sending event: {e:?}");
                }
            }

            if input.starts_with("info ") {
                let id = input.trim_start_matches("info ");
                let Ok(id) = id.parse::<u64>() else {
                    eprintln!("Invalid id: {id}");
                    continue;
                };

                let (sender, receiver) = mpsc::channel();

                if let Err(e) = p.send_event(CustomEventInternal::Info(id, sender)) {
                    eprintln!("Error sending event: {e:?}");
                }
                let node = match receiver.recv() {
                    Ok(node) => node,
                    Err(e) => {
                        eprintln!("Error receiving node: {e:?}");
                        continue;
                    }
                };

                node.dbg_current()
            }

            if input.starts_with("window ") {
                let id = input.trim_start_matches("window ");
                let Ok(id) = id.parse::<u64>() else {
                    eprintln!("Invalid window id: {id}");
                    continue;
                };

                window = Some(WindowId::from(id));
            }

            if input.starts_with("open ") {
                let url = input.trim_start_matches("open ");
                let Ok(url) = Url::parse(url) else {
                    eprintln!("Invalid url: {url}");
                    continue;
                };

                let Some(id) = window else {
                    eprintln!("No window set!");
                    continue;
                };

                if let Err(e) = p.send_event(CustomEventInternal::OpenTab(url, id)) {
                    eprintln!("Error sending event: {e:?}");
                }
            }
        }
    });

    application.run()?;

    Ok(())
}
