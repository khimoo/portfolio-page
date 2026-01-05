use crate::web::data_loader::{use_lightweight_articles, LightweightArticle};
use crate::web::routes::Route;
use crate::web::components::ArticleStateRenderer;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ArticleIndexPage)]
pub fn article_index_page() -> Html {
    let (articles, loading, error) = use_lightweight_articles();

    if *loading {
        return ArticleStateRenderer::render_index_loading();
    }

    if let Some(err) = error.as_ref() {
        return ArticleStateRenderer::render_index_error(&format!("{}", err));
    }

    html! {
        <>
            <style>{index_styles()}</style>
            <div class="article-index-container">
                <h1>{"記事一覧"}</h1>
                <div style="margin-bottom: 20px;">
                    <Link<Route> to={Route::Home}>
                        <button class="back-button">
                            {"← Back to Home"}
                        </button>
                    </Link<Route>>
                </div>
                {render_articles_list(&*articles)}
            </div>
        </>
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
        <li key={article.slug.clone()} style="margin-bottom: 20px; padding: 16px; border-radius: 8px;">
            <h3 style="margin: 0 0 8px 0;">
                <Link<Route> to={Route::ArticleShow { slug: article.slug.clone() }}>
                    {&article.title}
                </Link<Route>>
            </h3>
            {render_article_meta(article)}
        </li>
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
    }

    @media (prefers-color-scheme: dark) {
        :root {
            --bg-color: #081D35;
            --text-color: #e0e0e0;
            --link-color: #66b3ff;
            --meta-color: #aaa;
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
    }

    .article-index-container a {
        color: var(--link-color);
        text-decoration: none;
    }

    .article-meta {
        font-size: 12px;
        color: var(--meta-color);
    }

    .back-button {
        padding: 8px 16px; 
        background: #007bff; 
        color: white; 
        border: none; 
        border-radius: 4px; 
        cursor: pointer;
    }
    "#
}