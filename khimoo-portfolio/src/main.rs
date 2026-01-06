#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew_router::prelude::*;

#[cfg(target_arch = "wasm32")]
use khimoo_portfolio::web::app::App;

#[cfg(target_arch = "wasm32")]
use khimoo_portfolio::config::get_config;

#[cfg(target_arch = "wasm32")]
#[function_component(Root)]
fn root() -> Html {
    let config = get_config();
    let basename = if config.base_path.is_empty() {
        "/".to_string()
    } else {
        format!("{}/", config.base_path)
    };

    html! {
        <BrowserRouter basename={basename}>
            <App />
        </BrowserRouter>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<Root>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "cli-tools")]
    {
        use clap::Parser;
        use khimoo_portfolio::cli::Cli;

        let cli = Cli::parse();
        cli.execute()
    }

    #[cfg(not(feature = "cli-tools"))]
    {
        eprintln!("CLI tools not available. Build with --features cli-tools");
        std::process::exit(1);
    }
}
