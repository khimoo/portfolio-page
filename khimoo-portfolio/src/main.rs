use yew::prelude::*;
use yew_router::prelude::*;

use khimoo_portfolio::home::app::Home;
use khimoo_portfolio::home::article::{ArticleIndex, ArticleView};
use khimoo_portfolio::home::header::Header;
use khimoo_portfolio::home::routes::Route;



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Admin => html! { <h1> {"Admin"} </h1> },
        Route::ArticleIndex => html! { <ArticleIndex /> },
        Route::ArticleShow { slug } => html! { <ArticleView slug={slug} /> },
    }
}

#[function_component(Root)]
fn root() -> Html {
    let basename = if cfg!(debug_assertions) {
        "/".to_string()
    } else {
        "/khimoo.io/".to_string() // github pagesのURL
    };

    html! {
        <BrowserRouter basename={basename}>
            <>
                <div style="height:100vh;margin:0;padding:0;display: flex; flex-direction: column"> // wrapperクラスにしてcss外部化していきたい
                    <Header />
                    <Switch<Route> render={switch} />
                </div>
            </>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
