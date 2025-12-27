use yew::prelude::*;
use pulldown_cmark::{html, Parser};
use yew::virtual_dom::AttrValue;
use super::data_loader::{use_article_content, use_lightweight_articles};
use yew_router::prelude::*;
use super::routes::Route;

#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    let (articles, loading, error) = use_lightweight_articles();

    if *loading {
        return html! {
            <div style="padding: 16px;">
                <h1>{"Articles"}</h1>
                <p>{"Loading articles..."}</p>
            </div>
        };
    }

    if let Some(err) = error.as_ref() {
        return html! {
            <div style="padding: 16px;">
                <h1>{"Articles"}</h1>
                <p style="color: red;">{format!("Error loading articles: {}", err)}</p>
            </div>
        };
    }

    html! {
        <>
            <style>
                {"body {
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
                }"}
            </style>
            <div class="article-index-container">
                <h1>{"記事一覧"}</h1>
                <div style="margin-bottom: 20px;">
                    <Link<Route> to={Route::Home}>
                        <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                            {"← Back to Home"}
                        </button>
                    </Link<Route>>
                </div>
                {
                    if let Some(articles_list) = articles.as_ref() {
                        html! {
                            <ul style="list-style: none; padding: 0;">
                                {
                                    articles_list.iter().map(|article| {
                                        html! {
                                            <li key={article.slug.clone()} style="margin-bottom: 20px; padding: 16px; border-radius: 8px;">
                                                <h3 style="margin: 0 0 8px 0;">
                                                    <Link<Route> to={Route::ArticleShow { slug: article.slug.clone() }}>
                                                        {&article.title}
                                                    </Link<Route>>
                                                </h3>
                                                {
                                                    if let Some(summary) = &article.summary {
                                                        html! { <p style="margin: 8px 0;">{summary}</p> }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                                <div class="article-meta">
                                                    {
                                                        if let Some(category) = &article.metadata.category {
                                                            html! { <span style="margin-right: 16px;">{"Category: "}{category}</span> }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                    <span>{"Links: "}{article.inbound_count}</span>
                                                </div>
                                            </li>
                                        }
                                    }).collect::<Html>()
                                }
                            </ul>
                        }
                    } else {
                        html! { <p>{"No articles found."}</p> }
                    }
                }
            </div>
        </>
    }
}



#[derive(Properties, PartialEq)]
pub struct ArticleViewProps {
    pub slug: String,
}

#[function_component(ArticleView)]
pub fn article_view(props: &ArticleViewProps) -> Html {
    let (article, loading, error) = use_article_content(Some(props.slug.clone()));

    if *loading {
        return html! {
            <>
                <style>
                    {"html, body { background: #081D35; color: #e0e0e0; margin: 0; padding: 0; }
                     @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }"}
                </style>
                <div style="padding: 16px; background: #081D35; color: #e0e0e0; min-height: 100vh;">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Loading article..."}</h1>
                    <div style="margin-top: 20px;">
                        <div style="border: 4px solid #444; border-top: 4px solid #66b3ff; border-radius: 50%; width: 40px; height: 40px; animation: spin 2s linear infinite;"></div>
                    </div>
                </div>
            </>
        };
    }

    if let Some(err) = error.as_ref() {
        return html! {
            <>
                <style>
                    {"html, body { background: #081D35; color: #e0e0e0; margin: 0; padding: 0; }"}
                </style>
                <div style="padding: 16px; background: #081D35; color: #e0e0e0; min-height: 100vh;">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Article Not Found"}</h1>
                    <p style="color: #ff6b6b;">{format!("Error: {}", err)}</p>
                    <p>{"The article you're looking for doesn't exist or couldn't be loaded."}</p>
                </div>
            </>
        };
    }

    if let Some(article_data) = article.as_ref() {
        // Convert markdown to HTML
        let parser = Parser::new(&article_data.content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        let rendered = Html::from_html_unchecked(AttrValue::from(html_output));

        html! {
            <>
                <style>
                    {"html, body { background: #081D35; color: #e0e0e0; margin: 0; padding: 0; }
                     @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
                     .markdown-body { line-height: 1.6; color: #e0e0e0; }
                     .markdown-body h1, .markdown-body h2, .markdown-body h3 { margin-top: 24px; margin-bottom: 16px; color: #e0e0e0; }
                     .markdown-body p { margin-bottom: 16px; color: #e0e0e0; }
                     .markdown-body ul, .markdown-body ol { margin-bottom: 16px; padding-left: 30px; color: #e0e0e0; }
                     .markdown-body code { background: #2d3748; color: #e0e0e0; padding: 2px 4px; border-radius: 3px; font-size: 85%; }
                     .markdown-body pre { background: #2d3748; color: #e0e0e0; padding: 16px; border-radius: 6px; overflow: auto; }
                     .markdown-body blockquote { border-left: 4px solid #66b3ff; padding-left: 16px; color: #aaa; margin: 0 0 16px 0; }
                     .markdown-body a { color: #66b3ff; text-decoration: none; }
                     .markdown-body a:hover { color: #99ccff; text-decoration: underline; }
                     a { color: #66b3ff; text-decoration: none; }
                     a:hover { color: #99ccff; text-decoration: underline; }"}
                </style>
                <div style="padding: 16px; max-width: 800px; margin: 0 auto; background: #081D35; min-height: 100vh;">
                    <div style="margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center;">
                        <Link<Route> to={Route::Home}>
                            <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                        <Link<Route> to={Route::ArticleIndex}>
                            <button style="padding: 8px 16px; background: #4a5568; color: white; border: none; border-radius: 4px; cursor: pointer;">
                                {"All Articles"}
                            </button>
                        </Link<Route>>
                    </div>

                    <article>
                        <header style="margin-bottom: 32px; padding-bottom: 16px; border-bottom: 1px solid #444;">
                            <h1 style="margin: 0 0 16px 0; font-size: 2.5em; color: #e0e0e0;">{&article_data.title}</h1>
                            <div style="font-size: 14px; color: #aaa; display: flex; gap: 16px; flex-wrap: wrap;">
                                {
                                    if let Some(category) = &article_data.metadata.category {
                                        html! { <span>{"Category: "}<strong>{category}</strong></span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if let Some(importance) = article_data.metadata.importance {
                                        html! { <span>{"Importance: "}<strong>{importance}{"/5"}</strong></span> }
                                    } else {
                                        html! {}
                                    }
                                }
                                <span>{"Inbound links: "}<strong>{article_data.inbound_count}</strong></span>
                                {
                                    if !article_data.metadata.tags.is_empty() {
                                        html! {
                                            <span>
                                                {"Tags: "}
                                                {
                                                    article_data.metadata.tags.iter().enumerate().map(|(i, tag)| {
                                                        html! {
                                                            <>
                                                                {if i > 0 { ", " } else { "" }}
                                                                <span style="background: #4a5568; color: #e0e0e0; padding: 2px 6px; border-radius: 3px; font-size: 12px;">{tag}</span>
                                                            </>
                                                        }
                                                    }).collect::<Html>()
                                                }
                                            </span>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </header>

                        <div class="markdown-body">
                            { rendered }
                        </div>

                        {
                            if !article_data.outbound_links.is_empty() {
                                html! {
                                    <footer style="margin-top: 48px; padding-top: 24px; border-top: 1px solid #444;">
                                        <h3 style="color: #e0e0e0;">{"Related Articles"}</h3>
                                        <ul style="list-style: none; padding: 0;">
                                            {
                                                article_data.outbound_links.iter().map(|link| {
                                                    html! {
                                                        <li key={link.target_slug.clone()} style="margin-bottom: 8px;">
                                                            <Link<Route> to={Route::ArticleShow { slug: link.target_slug.clone() }}>
                                                                {&link.target_slug}
                                                            </Link<Route>>
                                                            {
                                                                if !link.context.is_empty() {
                                                                    html! { <span style="color: #aaa; font-size: 12px; margin-left: 8px;">{format!("\"{}\"", &link.context)}</span> }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                    </footer>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </article>
                </div>
            </>
        }
    } else {
        html! {
            <>
                <style>
                    {"html, body { background: #081D35; color: #e0e0e0; margin: 0; padding: 0; }"}
                </style>
                <div style="padding: 16px; background: #081D35; color: #e0e0e0; min-height: 100vh;">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Article Not Found"}</h1>
                    <p>{"The article you're looking for doesn't exist."}</p>
                </div>
            </>
        }
    }
}
