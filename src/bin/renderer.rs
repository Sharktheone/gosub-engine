use url::Url;

use gosub_renderer::render_tree::TreeDrawer;
use gosub_shared::types::Result;
use gosub_useragent::application::Application;
use gosub_vello::VelloBackend;

fn main() -> Result<()> {
    let matches = clap::Command::new("Gosub Renderer")
        .arg(
            clap::Arg::new("url")
                .help("The url or file to parse")
                .required(true)
                .index(1),
        )
        .get_matches();

    let url: String = matches.get_one::<String>("url").expect("url").to_string();

    // let mut rt = load_html_rendertree(&url)?;

    let mut application: Application<TreeDrawer<VelloBackend>, VelloBackend> =
        Application::new(VelloBackend::new());

    application.initial_tab(Url::parse(&url)?);

    application.start()?;

    Ok(())
}
