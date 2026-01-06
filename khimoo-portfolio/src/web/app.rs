use crate::config::get_config;
use crate::web::header::Header;
use crate::web::pages::{ArticleIndexPage, ArticleViewPage, HomePage};
use crate::web::routes::Route;
use crate::web::styles::LayoutStyles;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let config = get_config();
    let basename = if config.base_path.is_empty() {
        None
    } else {
        Some(config.base_path.clone())
    };

    html! {
        <BrowserRouter basename={basename}>
            <div style={LayoutStyles::app_wrapper()}>
                <Header />
                <main style={LayoutStyles::main_content()}>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Admin => html! { <div>{"Admin page - Not implemented yet"}</div> },
        Route::ArticleIndex => html! { <ArticleIndexPage /> },
        Route::ArticleShow { slug } => html! { <ArticleViewPage slug={slug} /> },
    }
}
