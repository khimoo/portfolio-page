use super::data_loader::{use_article_content, use_lightweight_articles};
use super::routes::Route;
use super::utils::resolve_image_path;
use pulldown_cmark::{html, Parser};
use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;

fn process_wiki_links(content: &str) -> String {
    // [[...]] の中身を取り出す
    let wiki_regex = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();

    wiki_regex
        .replace_all(content, |caps: &regex::Captures| {
            let inner = caps.get(1).unwrap().as_str();
            // '|' があれば左側をリンクターゲット、右側を表示テキストとして扱う
            let parts: Vec<&str> = inner.splitn(2, '|').collect();
            let (link_target, display) = if parts.len() == 2 {
                (parts[0].trim(), parts[1].trim())
            } else {
                (inner.trim(), inner.trim())
            };

            let slug = generate_slug_from_title(link_target);
            // マーカーは -- slug と display を :: で区切る形式にする
            format!("WIKILINKSTART:{}::{}::WIKILINKEND", slug, display)
        })
        .to_string()
}

fn convert_wiki_markers_to_html(html_content: &str) -> String {
    // process_wiki_links で作ったマーカーを <a> に置換
    // 非貪欲マッチ (.*?) を使い display 部分を正しく取得する
    let marker_regex = Regex::new(r"WIKILINKSTART:([^:]+)::(.*?)::WIKILINKEND").unwrap();

    marker_regex
        .replace_all(html_content, |caps: &regex::Captures| {
            let slug = &caps[1];
            let title = &caps[2];
            format!(
                r#"<a href="/article/{}" class="wiki-link">{}</a>"#,
                slug, title
            )
        })
        .to_string()
}

/// Generate a slug from article title (same logic as in article_processing.rs)
fn generate_slug_from_title(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .trim()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    // Replace multiple consecutive dashes with single dash
    let re = Regex::new(r"-+").unwrap();
    re.replace_all(&slug, "-").to_string()
}

#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    let (articles, loading, error) = use_lightweight_articles();

    if *loading {
        return html! {
            <div style="background:#081D35; padding: 16px; height:100%">
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

    // ナビゲーターを取得（これでRustコードから画面遷移できます）
    let navigator = use_navigator().unwrap();

    // クリックイベントハンドラーの定義
    // 記事本文内のクリックを監視し、.wiki-link がクリックされた時だけSPA遷移を割り込ませます
    let on_article_click = {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            // クリックされた要素を取得
            let target = e.target_unchecked_into::<web_sys::Element>();

            // クリックされた要素、またはその親要素が ".wiki-link" クラスを持っているか確認
            // (closestを使うことで、リンク内の文字などをクリックしても反応するようにします)
            if let Ok(Some(element)) = target.closest(".wiki-link") {
                // デフォルトのリンク動作（画面リロード）をキャンセル
                e.prevent_default();

                // href属性やpathnameから遷移先情報を取得するためにアンカー要素として扱う
                let anchor = element.unchecked_into::<HtmlAnchorElement>();
                let pathname = anchor.pathname(); // 例: "/article/my-slug"

                // パスからslug部分を抽出 (/article/以降)
                // ※URLの構造に依存します。ここでは単純に最後のセグメントを取得
                if let Some(slug) = pathname.split('/').last() {
                    if !slug.is_empty() {
                        // ここで <Link> と同じ動き（ルーター遷移）を実行
                        navigator.push(&Route::ArticleShow {
                            slug: slug.to_string(),
                        });
                    }
                }
            }
        })
    };
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
        // 1. 本文をそのまま取得
        let raw_content = &article_data.content;

        // 2. Wikiリンク記法をマーカーに変換
        // ここで変換されないと、Markdownパーサーが [[ ]] をリンクとして認識しません
        let processed_content = process_wiki_links(raw_content);

        // 3. Markdown を HTML に変換
        let parser = Parser::new(&processed_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // 4. 【重要】生成されたHTML文字列に対して、マーカーを <a> タグに置換
        // Markdownパーサーが特殊文字をエスケープしている可能性があるため、
        // 最終的なHTML文字列に対して実行するのが最も確実です。
        let final_html = convert_wiki_markers_to_html(&html_output);

        // 5. HTMLとしてレンダリング
        let rendered = Html::from_html_unchecked(AttrValue::from(final_html));

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

                     /* Wiki Link用のスタイル */
                     .wiki-link {
                         color: #66b3ff;
                         text-decoration: none;
                         background: rgba(102, 179, 255, 0.1);
                         padding: 2px 4px;
                         border-radius: 3px;
                         border: 1px solid rgba(102, 179, 255, 0.3);
                         transition: all 0.2s ease;
                     }
                     .wiki-link:hover {
                         color: #99ccff;
                         background: rgba(102, 179, 255, 0.3);
                         border-color: rgba(102, 179, 255, 0.6);
                         text-decoration: none;
                     }
                     "}
                </style>
                <div style="padding: 16px; max-width: 800px; margin: 0 auto; background: #081D35; min-height: 100vh;">
                    // <div style="margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center;">
                    //     <Link<Route> to={Route::Home}>
                    //         <button style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    //             {"← Back to Home"}
                    //         </button>
                    //     </Link<Route>>
                    //     <Link<Route> to={Route::ArticleIndex}>
                    //         <button style="padding: 8px 16px; background: #4a5568; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    //             {"All Articles"}
                    //         </button>
                    //     </Link<Route>>
                    // </div>

                    <article>
                        <header style="margin-bottom: 32px; padding-bottom: 16px; border-bottom: 1px solid #444; display: flex; justify-content: space-between; align-items: flex-start; gap: 20px;">
                            <div style="flex: 1;">
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
                            </div>
                            {
                                if let Some(author_image) = &article_data.metadata.author_image {
                                    let resolved_image_path = resolve_image_path(author_image);
                                    html! {
                                        <div style="flex-shrink: 0; display: flex; align-items: stretch;">
                                            <img
                                                src={resolved_image_path}
                                                alt="Author image"
                                                style="
                                                    height: 100%;
                                                    // aspect-ratio: 1;
                                                    object-fit: cover;
                                                    // min-height: 60px;
                                                    // max-height: 120px;
                                                "
                                            />
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </header>

                        // ここでリンク化されたHTMLがレンダリングされます
                        <div class="markdown-body" onclick={on_article_click}>
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
