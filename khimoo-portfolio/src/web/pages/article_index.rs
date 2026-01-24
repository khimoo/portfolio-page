use crate::web::components::ArticleStateRenderer;
use crate::web::data_loader::{use_articles_data, LightweightArticle};
use crate::web::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ArticleIndexPage)]
pub fn article_index_page() -> Html {
    let (articles_data, loading, error) = use_articles_data();
    let selected_tags = use_state(|| Vec::<String>::new());

    if *loading {
        return ArticleStateRenderer::render_index_loading();
    }

    if let Some(err) = error.as_ref() {
        return ArticleStateRenderer::render_index_error(&format!("{}", err));
    }

    let articles = articles_data.as_ref().map(|data| {
        data.articles
            .iter()
            .cloned()
            .map(LightweightArticle::from)
            .collect::<Vec<_>>()
    });

    let all_tags = articles
        .as_ref()
        .map(|articles_list| collect_all_tags(articles_list))
        .unwrap_or_default();

    let on_toggle_tag = {
        let selected_tags = selected_tags.clone();
        Callback::from(move |tag: String| {
            let mut next_tags = (*selected_tags).clone();
            if let Some(index) = next_tags.iter().position(|item| item == &tag) {
                next_tags.remove(index);
            } else {
                next_tags.push(tag);
            }
            selected_tags.set(next_tags);
        })
    };

    let filtered_articles = articles.as_ref().map(|articles_list| {
        filter_articles_by_tags(articles_list, &selected_tags)
    });

    html! {
        <>
            <style>{index_styles()}</style>
            <div class="article-index-container">
                <h1>{"記事一覧"}</h1>

                {render_tag_filters(&all_tags, &selected_tags, &on_toggle_tag)}
                {render_articles_list(&filtered_articles)}
            </div>
        </>
    }
}

fn collect_all_tags(articles_list: &[LightweightArticle]) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    for article in articles_list {
        for tag in &article.metadata.tags {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        }
    }

    tags
}

fn filter_articles_by_tags(
    articles_list: &[LightweightArticle],
    selected_tags: &UseStateHandle<Vec<String>>,
) -> Vec<LightweightArticle> {
    if selected_tags.is_empty() {
        return articles_list.to_vec();
    }

    articles_list
        .iter()
        .cloned()
        .filter(|article| {
            article
                .metadata
                .tags
                .iter()
                .any(|tag| selected_tags.contains(tag))
        })
        .collect::<Vec<_>>()
}

fn render_tag_filters(
    all_tags: &[String],
    selected_tags: &UseStateHandle<Vec<String>>,
    on_toggle_tag: &Callback<String>,
) -> Html {
    if all_tags.is_empty() {
        return html! {};
    }

    html! {
        <div class="tag-filter">
            <span class="tag-filter-label">{"タグで絞り込み:"}</span>
            <div class="tag-filter-options">
                {all_tags.iter().map(|tag| {
                    let is_selected = selected_tags.contains(tag);
                    render_tag_button(tag, is_selected, on_toggle_tag)
                }).collect::<Html>()}
            </div>
        </div>
    }
}

fn render_tag_button(tag: &str, is_selected: bool, on_toggle_tag: &Callback<String>) -> Html {
    let tag_label = tag.to_string();
    let on_toggle_tag = on_toggle_tag.clone();
    let onclick = Callback::from(move |_| on_toggle_tag.emit(tag_label.clone()));

    let class_name = if is_selected {
        "tag-option selected"
    } else {
        "tag-option"
    };

    html! {
        <button type="button" class={class_name} {onclick}>
            {tag}
        </button>
    }
}

fn render_articles_list(articles: &Option<Vec<LightweightArticle>>) -> Html {
    if let Some(articles_list) = articles.as_ref() {
        html! {
            <ul style="list-style: none; padding: 0;">
                {
                    articles_list.iter().map(|article| {
                        render_article_item(article)
                    }).collect::<Html>()
                }
            </ul>
        }
    } else {
        html! { <p>{"No articles found."}</p> }
    }
}

fn render_article_item(article: &LightweightArticle) -> Html {
    html! {
        <li key={article.slug.clone()} class="article-item">
            <h3 class="article-title">
                <Link<Route> to={Route::ArticleShow { slug: article.slug.clone() }}>
                    {&article.title}
                </Link<Route>>
            </h3>
            {render_article_summary(article)}
            {render_article_meta(article)}
        </li>
    }
}

fn render_article_summary(article: &LightweightArticle) -> Html {
    if let Some(summary) = &article.summary {
        html! {
            <p class="article-summary">{summary}</p>
        }
    } else {
        html! {}
    }
}

fn render_article_meta(article: &LightweightArticle) -> Html {
    html! {
        <div class="article-meta">
            {render_category(&article.metadata.category)}
            <span>{"Links: "}{article.inbound_links.len()}</span>
        </div>
    }
}

fn render_category(category: &Option<String>) -> Html {
    if let Some(category) = category {
        html! { <span style="margin-right: 16px;">{"Category: "}{category}</span> }
    } else {
        html! {}
    }
}

fn index_styles() -> &'static str {
    r#"
    body {
        margin: 0;
        padding: 0;
    }

    :root {
        --bg-color: #ffffff;
        --text-color: #333333;
        --link-color: #007bff;
        --meta-color: #666;
        --summary-color: #555;
        --border-color: #e0e0e0;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            --bg-color: #081D35;
            --text-color: #e0e0e0;
            --link-color: #66b3ff;
            --meta-color: #aaa;
            --summary-color: #ccc;
            --border-color: #333;
        }
    }

    html, body {
        background: var(--bg-color);
        color: var(--text-color);
    }

    .article-index-container {
        padding: 16px;
        background: var(--bg-color);
        color: var(--text-color);
        min-height: 100vh;
        width: 100%;
    }

    .article-index-container a {
        color: var(--link-color);
        text-decoration: none;
    }

    .article-item {
        margin-bottom: 24px;
        padding: 20px;
        border-radius: 8px;
        border: 1px solid var(--border-color);
        background: var(--bg-color);
    }

    .article-title {
        margin: 0 0 12px 0;
        font-size: 1.2em;
    }

    .article-summary {
        margin: 0 0 12px 0;
        color: var(--summary-color);
        line-height: 1.5;
        font-size: 0.95em;
    }

    .article-meta {
        font-size: 12px;
        color: var(--meta-color);
    }

    .tag-filter {
        margin: 12px 0 20px 0;
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .tag-filter-label {
        font-size: 12px;
        color: var(--meta-color);
    }

    .tag-filter-options {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .tag-option {
        padding: 6px 10px;
        border-radius: 999px;
        border: 1px solid var(--border-color);
        background: transparent;
        color: var(--text-color);
        font-size: 12px;
        cursor: pointer;
    }

    .tag-option.selected {
        background: var(--link-color);
        color: #fff;
        border-color: var(--link-color);
    }


    "#
}
