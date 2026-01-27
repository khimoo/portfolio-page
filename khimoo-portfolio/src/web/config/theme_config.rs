/// テーマとカラースキームを一元管理
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub accent: String,
    pub error: String,
    pub warning: String,
    pub success: String,
}

pub struct ThemeConfig;

impl ThemeConfig {
    /// ライトテーマの色設定
    pub fn light_theme() -> ThemeColors {
        ThemeColors {
            primary: "#2563eb".to_string(),
            secondary: "#64748b".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f8fafc".to_string(),
            text_primary: "#1e293b".to_string(),
            text_secondary: "#64748b".to_string(),
            accent: "#3b82f6".to_string(),
            error: "#ef4444".to_string(),
            warning: "#f59e0b".to_string(),
            success: "#10b981".to_string(),
        }
    }

    /// ダークテーマの色設定
    pub fn dark_theme() -> ThemeColors {
        ThemeColors {
            primary: "#3b82f6".to_string(),
            secondary: "#94a3b8".to_string(),
            background: "#0f172a".to_string(),
            surface: "#1e293b".to_string(),
            text_primary: "#f1f5f9".to_string(),
            text_secondary: "#94a3b8".to_string(),
            accent: "#60a5fa".to_string(),
            error: "#f87171".to_string(),
            warning: "#fbbf24".to_string(),
            success: "#34d399".to_string(),
        }
    }

    /// 現在のテーマを取得（将来的にはユーザー設定から）
    pub fn current_theme() -> ThemeColors {
        // 現在はライトテーマ固定、将来的にはユーザー設定やシステム設定から取得
        Self::light_theme()
    }
}
