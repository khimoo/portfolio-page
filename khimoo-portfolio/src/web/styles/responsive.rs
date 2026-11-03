//! Global responsive stylesheet
//!
//! Most styles in this app are inline, and inline styles cannot contain
//! media queries. Elements that need to adapt to the screen size get a
//! class name, and the rules below override their inline styles
//! (hence `!important`) on small screens.

use crate::web::config::StyleConfig;
use yew::prelude::*;

/// Render the global responsive `<style>` element. Mount once at the app root.
#[function_component(GlobalResponsiveStyles)]
pub fn responsive_styles() -> Html {
    html! { <style>{global_responsive_css()}</style> }
}

pub fn global_responsive_css() -> String {
    let mobile = StyleConfig::MOBILE_BREAKPOINT;
    let small = StyleConfig::SMALL_MOBILE_BREAKPOINT;
    format!(
        r#"
*, *::before, *::after {{ box-sizing: border-box; }}
html {{ -webkit-text-size-adjust: 100%; text-size-adjust: 100%; }}

.markdown-body {{ overflow-wrap: break-word; }}
.markdown-body img, .markdown-body video, .markdown-body iframe {{ max-width: 100%; height: auto; }}
.markdown-body pre {{ overflow-x: auto; -webkit-overflow-scrolling: touch; }}
.markdown-body table {{ display: block; max-width: 100%; overflow-x: auto; }}

/* ホームのノードグラフ: タッチ操作でスクロール/ズームさせずドラッグに使う */
.physics-container {{ overflow: hidden; touch-action: none; overscroll-behavior: none; -webkit-user-select: none; user-select: none; }}
.graph-node {{ touch-action: none; -webkit-tap-highlight-color: transparent; }}

@media (max-width: {mobile}px) {{
    .site-header-container {{ padding: 8px 12px !important; align-items: center !important; flex-wrap: wrap; }}
    .site-logo {{ font-size: 22px !important; }}
    .site-nav {{ gap: 14px !important; align-items: center !important; }}
    .site-nav a {{ padding: 6px 0; }}

    .welcome-overlay {{
        width: calc(100% - 32px);
        max-width: 480px;
        transform: translate(-50%, 0) !important;
        padding: 12px !important;
        font-size: 13px !important;
    }}
    .welcome-name {{ font-size: 24px !important; }}
    .welcome-name-en {{ font-size: 14px !important; }}
    .welcome-body {{ white-space: normal !important; }}

    .debug-panel {{
        top: 8px !important; right: 8px !important; padding: 10px !important;
        max-height: 45vh; overflow-y: auto; font-size: 12px;
    }}
    .debug-panel input[type=range] {{ width: 140px !important; }}

    .article-index-container {{ padding: 12px !important; }}
    .article-item {{ padding: 14px !important; margin-bottom: 16px !important; }}
    .tag-option {{ padding: 8px 12px !important; }}

    .article-container {{ padding: 12px !important; }}
    .article-header {{ flex-direction: column-reverse !important; gap: 12px !important; margin-bottom: 20px !important; }}
    .article-title-heading {{ font-size: 1.7em !important; overflow-wrap: break-word; }}
    .article-author-image img {{ height: 80px !important; }}
    .markdown-body {{ font-size: 17px !important; }}
    .markdown-body ul, .markdown-body ol {{ padding-left: 22px !important; }}
    .markdown-body h1 {{ font-size: 1.6em; }}
    .markdown-body h2 {{ font-size: 1.35em; }}
}}

@media (max-width: {small}px) {{
    .site-logo {{ font-size: 18px !important; }}
    .site-nav {{ gap: 10px !important; font-size: 14px; }}
    .article-title-heading {{ font-size: 1.45em !important; }}
    .markdown-body {{ font-size: 16px !important; }}
}}
"#
    )
}
