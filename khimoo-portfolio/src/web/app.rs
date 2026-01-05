use yew::prelude::*;
use yew_router::prelude::*;
use crate::web::routes::Route;
use crate::web::pages::{HomePage, ArticleIndexPage, ArticleViewPage};
use crate::web::header::Header;
use crate::web::styles::LayoutStyles;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
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