use crate::web::components::{ArticleContent, ArticleHeader, ArticleStateRenderer};
use crate::web::data_loader::{use_article_content, DataLoader};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleViewProps {
    pub slug: String,
}

#[function_component(ArticleViewPage)]
pub fn article_view_page(props: &ArticleViewProps) -> Html {
    // hooks: 必ず先頭で宣言
    let (article, loading, error) = use_article_content(Some(props.slug.clone()));
    let article_content = use_state(|| None::<String>);
    let content_loading = use_state(|| false);
    let content_error = use_state(|| None::<String>);

    // コンテンツ読み込み処理
    {
        let article = article.clone();
        let article_content = article_content.clone();
        let content_loading = content_loading.clone();
        let content_error = content_error.clone();

        use_effect_with(article.clone(), move |article| {
            if let Some(article_data) = article.as_ref() {
                let file_path = article_data.file_path.clone();
                let article_content = article_content.clone();
                let content_loading = content_loading.clone();
                let content_error = content_error.clone();

                content_loading.set(true);
                content_error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let loader = DataLoader::new();
                    match loader.load_article_content_only(&file_path).await {
                        Ok(content) => {
                            article_content.set(Some(content));
                            content_error.set(None);
                        }
                        Err(e) => {
                            content_error.set(Some(format!("{}", e)));
                        }
                    }
                    content_loading.set(false);
                });
            }

            || {}
        });
    }

    // 状態に応じたレンダリング
    if *loading {
        return ArticleStateRenderer::render_article_loading();
    }

    if let Some(err) = error.as_ref() {
        return ArticleStateRenderer::render_article_not_found(&format!("{}", err));
    }

    if let Some(article_data) = article.as_ref() {
        // コンテンツ読み込み中
        if *content_loading {
            return ArticleStateRenderer::render_content_loading(&article_data.title);
        }

        // コンテンツ読み込みエラー
        if let Some(err) = content_error.as_ref() {
            return ArticleStateRenderer::render_content_error(&article_data.title, err);
        }

        // 正常なコンテンツ表示
        if let Some(raw_content) = article_content.as_ref() {
            return html! {
                <>
                    <style>{article_styles()}</style>
                    <div class="article-container">
                        <article>
                            <ArticleHeader article={article_data.clone()} />
                            <ArticleContent
                                article={article_data.clone()}
                                content={raw_content.clone()}
                            />
                        </article>
                    </div>
                </>
            };
        } else {
            return ArticleStateRenderer::render_content_unavailable();
        }
    }

    // フォールバック: 記事が見つからない
    ArticleStateRenderer::render_article_not_found("Article not found")
}

fn article_styles() -> &'static str {
    r#"
    html, body { 
        background: #081D35; 
        color: #e0e0e0; 
        margin: 0; 
        padding: 0; 
    }
    .article-container {
        padding: 16px; 
        max-width: 800px; 
        margin: 0 auto; 
        background: #081D35; 
        min-height: 100vh;
    }
    "#
}
