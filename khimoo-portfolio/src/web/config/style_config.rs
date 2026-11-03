/// スタイル設定を一元管理
pub struct StyleConfig;

impl StyleConfig {
    /// レスポンシブデザインのブレークポイント
    pub const MOBILE_BREAKPOINT: i32 = 768;
    pub const TABLET_BREAKPOINT: i32 = 1024;
    pub const DESKTOP_BREAKPOINT: i32 = 1200;
    pub const SMALL_MOBILE_BREAKPOINT: i32 = 420;

    /// コンテナ幅がモバイルブレークポイント以下か
    pub fn is_mobile_width(width: f32) -> bool {
        width > 0.0 && width <= Self::MOBILE_BREAKPOINT as f32
    }

    /// 画面幅に応じたノードサイズの倍率（狭い画面ではノードを縮小する）
    pub fn node_scale_for_width(width: f32) -> f32 {
        if !Self::is_mobile_width(width) {
            return 1.0;
        }
        (width / Self::MOBILE_BREAKPOINT as f32).clamp(0.55, 1.0)
    }

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
