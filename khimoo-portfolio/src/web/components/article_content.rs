use crate::web::data_loader::ProcessedArticle;
use crate::web::routes::Route;
use pulldown_cmark::{html, Parser};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleContentProps {
    pub article: ProcessedArticle,
    pub content: String,
}

#[function_component(ArticleContent)]
pub fn article_content(props: &ArticleContentProps) -> Html {
    // Markdownを処理してHTMLに変換（WikiLink処理なし）
    let processed_html = process_markdown_content(&props.content);
    let rendered = Html::from_html_unchecked(AttrValue::from(processed_html));

    html! {
        <>
            <style>{content_styles()}</style>
            <div class="markdown-body">
                {rendered}
            </div>
            {render_related_articles(&props.article)}
        </>
    }
}

/// Markdownコンテンツを処理してHTMLに変換（シンプル版）
fn process_markdown_content(content: &str) -> String {
    // Markdown を HTML に変換
    let parser = Parser::new(content);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn render_related_articles(article: &ProcessedArticle) -> Html {
    if !article.outbound_links.is_empty() {
        html! {
            <footer style="margin-top: 48px; padding-top: 24px; border-top: 1px solid #444;">
                <h3 style="color: #e0e0e0;">{"関連リンク"}</h3>
                <ul style="list-style: none; padding: 0;">
                    {
                        article.outbound_links.iter().map(|link| {
                            html! {
                                <li key={link.target_slug.clone()} style="margin-bottom: 8px;">
                                    <Link<Route> to={Route::ArticleShow { slug: link.target_slug.clone() }}>
                                        {&link.target_slug}
                                    </Link<Route>>
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

/// コンテンツ用のCSS（WikiLinkスタイル削除）
fn content_styles() -> String {
    r#"
    .markdown-body {
        line-height: 1.6;
        color: #e0e0e0;
    }
    .markdown-body h1, .markdown-body h2, .markdown-body h3 {
        margin-top: 24px;
        margin-bottom: 6px;
        color: #e0e0e0;
    }
    .markdown-body p {
        margin-bottom: 16px;
        color: #e0e0e0;
    }
    .markdown-body ul, .markdown-body ol {
        margin-bottom: 16px;
        padding-left: 30px;
        color: #e0e0e0;
    }
    .markdown-body code {
        background: #2d3748;
        color: #e0e0e0;
        padding: 2px 4px;
        border-radius: 3px;
        font-size: 85%;
    }
    .markdown-body pre {
        background: #2d3748;
        color: #e0e0e0;
        padding: 16px;
        border-radius: 6px;
        overflow: auto;
    }
    .markdown-body blockquote {
        border-left: 4px solid #66b3ff;
        padding-left: 16px;
        color: #aaa;
        margin: 0 0 16px 0;
    }
    .markdown-body a {
        color: #66b3ff;
        text-decoration: none;
    }
    .markdown-body a:hover {
        color: #99ccff;
        text-decoration: underline;
    }
    "#
    .to_string()
}
