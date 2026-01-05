use crate::web::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// ローディング/エラー状態の表示を担当
pub struct ArticleStateRenderer;

impl ArticleStateRenderer {
    /// 共通のベーススタイル
    fn base_styles() -> &'static str {
        r#"
        html, body { 
            background: #081D35; 
            color: #e0e0e0; 
            margin: 0; 
            padding: 0; 
        }
        @keyframes spin { 
            0% { transform: rotate(0deg); } 
            100% { transform: rotate(360deg); } 
        }
        .loading-spinner {
            border: 4px solid #444; 
            border-top: 4px solid #66b3ff; 
            border-radius: 50%; 
            width: 40px; 
            height: 40px; 
            animation: spin 2s linear infinite;
        }
        .back-button {
            padding: 8px 16px; 
            background: #007bff; 
            color: white; 
            border: none; 
            border-radius: 4px; 
            cursor: pointer;
        }
        .container {
            padding: 16px; 
            background: #081D35; 
            color: #e0e0e0; 
            min-height: 100vh;
        }
        .error-text {
            color: #ff6b6b;
        }
        "#
    }

    /// 記事メタデータ読み込み中の表示
    pub fn render_article_loading() -> Html {
        html! {
            <>
                <style>{Self::base_styles()}</style>
                <div class="container">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button class="back-button">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Loading article..."}</h1>
                    <div style="margin-top: 20px;">
                        <div class="loading-spinner"></div>
                    </div>
                </div>
            </>
        }
    }

    /// 記事が見つからない場合の表示
    pub fn render_article_not_found(error: &str) -> Html {
        html! {
            <>
                <style>{Self::base_styles()}</style>
                <div class="container">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button class="back-button">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Article Not Found"}</h1>
                    <p class="error-text">{format!("Error: {}", error)}</p>
                    <p>{"The article you're looking for doesn't exist or couldn't be loaded."}</p>
                </div>
            </>
        }
    }

    /// 記事コンテンツ読み込み中の表示
    pub fn render_content_loading(title: &str) -> Html {
        html! {
            <>
                <style>{Self::base_styles()}</style>
                <div class="container">
                    <h1>{title}</h1>
                    <div style="margin-top: 20px;">
                        <div class="loading-spinner"></div>
                    </div>
                    <p>{"Loading content..."}</p>
                </div>
            </>
        }
    }

    /// コンテンツ読み込みエラーの表示
    pub fn render_content_error(title: &str, error: &str) -> Html {
        html! {
            <>
                <style>{Self::base_styles()}</style>
                <div class="container">
                    <h1>{title}</h1>
                    <p class="error-text">{format!("Failed to load content: {}", error)}</p>
                </div>
            </>
        }
    }

    /// コンテンツが利用できない場合の表示
    pub fn render_content_unavailable() -> Html {
        html! {
            <>
                <style>{Self::base_styles()}</style>
                <div class="container">
                    <div style="margin-bottom: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button class="back-button">
                                {"← Back to Home"}
                            </button>
                        </Link<Route>>
                    </div>
                    <h1>{"Content Not Available"}</h1>
                    <p>{"The article content could not be loaded."}</p>
                </div>
            </>
        }
    }

    /// 記事インデックスのローディング表示
    pub fn render_index_loading() -> Html {
        html! {
            <div style="background:#081D35; padding: 16px; height:100%">
                <h1>{"Articles"}</h1>
                <p>{"Loading articles..."}</p>
            </div>
        }
    }

    /// 記事インデックスのエラー表示
    pub fn render_index_error(error: &str) -> Html {
        html! {
            <div style="padding: 16px;">
                <h1>{"Articles"}</h1>
                <p style="color: red;">{format!("Error loading articles: {}", error)}</p>
            </div>
        }
    }
}