use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagQuery {
    #[serde(default)]
    pub tags: Option<String>,
}

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
