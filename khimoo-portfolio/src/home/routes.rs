use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
    #[at("/article")]
    ArticleIndex,
    #[at("/article/:slug")]
    ArticleShow { slug: String },
}