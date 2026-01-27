use crate::config::get_config;
use crate::web::components::{TagPill, TagStyles};
use crate::web::data_loader::ProcessedArticle;
use crate::web::routes::{Route, TagQuery};
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleHeaderProps {
    pub article: ProcessedArticle,
}

#[function_component(ArticleHeader)]
pub fn article_header(props: &ArticleHeaderProps) -> Html {
    let article = &props.article;
    let navigator = use_navigator();
    let on_tag_click = {
        let navigator = navigator.clone();
        Callback::from(move |tag: String| {
            let query = TagQuery { tags: Some(tag.clone()) };
            if let Some(navigator) = navigator.as_ref() {
                if navigator
                    .push_with_query(&Route::ArticleIndex, &query)
                    .is_ok()
                {
                    return;
                }
            }

            let fallback_url = format!("{}?tags={}", get_config().get_url("article"), tag);
            if let Some(window) = window() {
                let _ = window.location().set_href(&fallback_url);
            }
        })
    };

    html! {
        <>
            <TagStyles />
            <header style="margin-bottom: 32px; padding-bottom: 16px; border-bottom: 1px solid #444; display: flex; justify-content: space-between; align-items: flex-start; gap: 20px;">
                <div style="flex: 1;">
                    <h1 style="margin: 0 0 16px 0; font-size: 2.5em; color: #e0e0e0;">
                        {&article.title}
                    </h1>
                    <div style="font-size: 14px; color: #aaa; display: flex; gap: 16px; flex-wrap: wrap;">
                        {render_importance(Some(article.metadata.importance))}
                        {render_inbound_links_count(article.inbound_links.len())}
                    </div>
                    <div style="font-size: 14px; color: #aaa; display: flex; gap: 16px; flex-wrap: wrap;">
                        {render_tags(&article.metadata.tags, &on_tag_click)}
                    </div>
                </div>
                {render_author_image(&article.metadata.author_image)}
            </header>
        </>
    }
}

fn render_importance(importance: Option<u8>) -> Html {
    if let Some(importance) = importance {
        html! {
            <span>{"Importance: "}<strong>{importance}{"/5"}</strong></span>
        }
    } else {
        html! {}
    }
}

fn render_inbound_links_count(count: usize) -> Html {
    html! {
        <span>{"Inbound links: "}<strong>{count}</strong></span>
    }
}

fn render_tags(tags: &[String], on_tag_click: &Callback<String>) -> Html {
    if !tags.is_empty() {
        html! {
            <span class="tag-list">
                <span class="tag-list-label">{"Tags: "}</span>
                {tags.iter().map(|tag| {
                    html! {
                        <TagPill label={tag.clone()} on_click={Some(on_tag_click.clone())} />
                    }
                }).collect::<Html>()}
            </span>
        }
    } else {
        html! {}
    }
}

fn render_author_image(author_image: &Option<String>) -> Html {
    if let Some(author_image) = author_image {
        let resolved_image_path = get_config().get_url(author_image);
        html! {
            <div style="flex-shrink: 0; display: flex; align-items: stretch;">
                <img
                    src={resolved_image_path}
                    alt="Author image"
                    style="height: 120px; object-fit: cover;"
                />
            </div>
        }
    } else {
        html! {}
    }
}
