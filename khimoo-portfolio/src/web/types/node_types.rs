use crate::web::routes::Route;
use crate::web::styles::NodeStyles;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct NodeId(pub u32);

// Special node ID for the author node (always 0)
pub const AUTHOR_NODE_ID: NodeId = NodeId(0);

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Author,
    Article,
}

/// Navigation action when a node is clicked
#[derive(Debug, Clone, PartialEq)]
pub enum NodeNavigation {
    /// Navigate to article detail page (/article/:slug)
    ShowArticle(String),
    /// Do nothing or stay on home
    StayOnHome,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeContent {
    Text(String),
    Image(String),
    Link {
        text: String,
        url: String,
    },
    Author {
        name: String,
        image_url: String,
        bio: Option<String>,
    },
    Article {
        title: String,
        slug: String,
    },
}

impl Default for NodeContent {
    fn default() -> Self {
        NodeContent::Text("".to_string())
    }
}

impl NodeContent {
    pub fn render_content(&self) -> Html {
        match self {
            NodeContent::Text(text) => html! {
                <span style={NodeStyles::text_node()}>
                    {text}
                </span>
            },
            NodeContent::Image(url) => html! {
                <img
                    src={url.clone()}
                    style="max-width: 100%; max-height: 100%; object-fit: contain;"
                />
            },
            NodeContent::Article { title, slug } => {
                html! {
                    <Link<Route> to={Route::ArticleShow { slug: slug.clone() }}>
                        <span style={NodeStyles::link_node()}>
                            {title}
                        </span>
                    </Link<Route>>
                }
            }
            NodeContent::Link { text, url } => {
                if url.starts_with("/") && !url.starts_with("//") {
                    if url == "/" {
                        html! {
                            <Link<Route> to={Route::Home}>
                                <span style={NodeStyles::link_node()}>
                                    {text}
                                </span>
                            </Link<Route>>
                        }
                    } else if url == "/article" {
                        html! {
                            <Link<Route> to={Route::ArticleIndex}>
                                <span style={NodeStyles::link_node()}>
                                    {text}
                                </span>
                            </Link<Route>>
                        }
                    } else if url.starts_with("/article/") {
                        let slug = url.strip_prefix("/article/").unwrap_or("").to_string();
                        html! {
                            <Link<Route> to={Route::ArticleShow { slug }}>
                                <span style={NodeStyles::link_node()}>
                                    {text}
                                </span>
                            </Link<Route>>
                        }
                    } else {
                        html! {
                            <a
                                href={url.clone()}
                                style={NodeStyles::link_node()}
                            >
                                {text}
                            </a>
                        }
                    }
                } else {
                    html! {
                        <a
                            href={url.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                            style={NodeStyles::link_node()}
                        >
                            {text}
                        </a>
                    }
                }
            }
            NodeContent::Author {
                name: _,
                image_url,
                bio: _,
            } => html! {
                <img
                    src={image_url.clone()}
                    style={NodeStyles::author_image()}
                    loading="lazy"
                    decoding="async"
                />
            },
        }
    }

    pub fn get_node_type(&self) -> NodeType {
        match self {
            NodeContent::Author { .. } => NodeType::Author,
            _ => NodeType::Article,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionLine {
    pub from: NodeId,
    pub to: NodeId,
    pub connection_type: ConnectionLineType,
    pub strength: f32,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionLineType {
    DirectLink,
    Bidirectional,
    AuthorToArticle,
    Strong,
    Medium,
    Weak,
}
