/// スタイル設定を一元管理
pub struct StyleConfig;

impl StyleConfig {
    /// レスポンシブデザインのブレークポイント
    pub const MOBILE_BREAKPOINT: i32 = 768;
    pub const TABLET_BREAKPOINT: i32 = 1024;
    pub const DESKTOP_BREAKPOINT: i32 = 1200;

    /// 共通のアニメーション設定
    pub const TRANSITION_DURATION: &'static str = "0.2s";
    pub const TRANSITION_EASING: &'static str = "ease-in-out";

    /// 共通のz-index値
    pub const Z_INDEX_BACKGROUND: i32 = 1;
    pub const Z_INDEX_NODES: i32 = 10;
    pub const Z_INDEX_UI: i32 = 100;
    pub const Z_INDEX_DEBUG: i32 = 1000;

    /// 共通のボックスシャドウ
    pub fn node_shadow() -> String {
        "0 4px 8px rgba(0,0,0,0.2)".to_string()
    }

    pub fn hover_shadow() -> String {
        "0 6px 12px rgba(0,0,0,0.3)".to_string()
    }

    /// 共通のボーダー半径
    pub const BORDER_RADIUS_SMALL: &'static str = "4px";
    pub const BORDER_RADIUS_MEDIUM: &'static str = "8px";
    pub const BORDER_RADIUS_LARGE: &'static str = "12px";

    /// 共通のスペーシング
    pub const SPACING_XS: &'static str = "4px";
    pub const SPACING_SM: &'static str = "8px";
    pub const SPACING_MD: &'static str = "16px";
    pub const SPACING_LG: &'static str = "24px";
    pub const SPACING_XL: &'static str = "32px";
}