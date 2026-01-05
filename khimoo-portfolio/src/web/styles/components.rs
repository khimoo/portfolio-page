use super::theme::*;

/// Button styles
pub struct ButtonStyles;

impl ButtonStyles {
    pub fn primary() -> String {
        format!(
            "background: {}; color: {}; border: none; padding: {} {}; border-radius: {}; font-weight: 600; cursor: pointer; text-decoration: none; display: inline-flex; align-items: center; gap: {};",
            DARK_THEME.accent_blue,
            DARK_THEME.text_primary,
            SPACING.sm,
            SPACING.md,
            BORDER_RADIUS.sm,
            SPACING.xs
        )
    }
    
    pub fn secondary() -> String {
        format!(
            "background: transparent; color: {}; border: 1px solid {}; padding: {} {}; border-radius: {}; font-weight: 600; cursor: pointer; text-decoration: none; display: inline-flex; align-items: center; gap: {};",
            DARK_THEME.text_primary,
            DARK_THEME.text_secondary,
            SPACING.sm,
            SPACING.md,
            BORDER_RADIUS.sm,
            SPACING.xs
        )
    }
    
    pub fn link() -> String {
        format!(
            "background: none; border: none; color: {}; text-decoration: none; cursor: pointer; font-weight: 600;",
            DARK_THEME.link_color
        )
    }
}

/// Card styles
pub struct CardStyles;

impl CardStyles {
    pub fn base() -> String {
        format!(
            "background: {}; border-radius: {}; padding: {}; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);",
            DARK_THEME.surface,
            BORDER_RADIUS.md,
            SPACING.lg
        )
    }
    
    pub fn elevated() -> String {
        format!(
            "background: {}; border-radius: {}; padding: {}; box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);",
            DARK_THEME.surface,
            BORDER_RADIUS.md,
            SPACING.lg
        )
    }
}

/// Input styles
pub struct InputStyles;

impl InputStyles {
    pub fn base() -> String {
        format!(
            "background: {}; border: 1px solid {}; color: {}; padding: {} {}; border-radius: {}; font-size: {}; width: 100%;",
            DARK_THEME.secondary_bg,
            DARK_THEME.text_muted,
            DARK_THEME.text_primary,
            SPACING.sm,
            SPACING.md,
            BORDER_RADIUS.sm,
            TYPOGRAPHY.body_md
        )
    }
    
    pub fn range() -> String {
        format!(
            "width: 100%; height: 4px; border-radius: {}; background: {}; outline: none; cursor: pointer;",
            BORDER_RADIUS.sm,
            DARK_THEME.text_muted
        )
    }
}

/// Loading spinner styles
pub struct LoadingStyles;

impl LoadingStyles {
    pub fn spinner() -> String {
        format!(
            "border: 4px solid #f3f3f3; border-top: 4px solid {}; border-radius: {}; width: 40px; height: 40px; animation: spin 2s linear infinite; margin: 0 auto;",
            DARK_THEME.info,
            BORDER_RADIUS.full
        )
    }
    
    pub fn container() -> String {
        format!(
            "display: flex; justify-content: center; align-items: center; height: 100vh; background: {};",
            "#f0f0f0"
        )
    }
    
    pub fn text() -> String {
        format!(
            "text-align: center; color: {}; margin-top: {};",
            DARK_THEME.text_primary,
            SPACING.lg
        )
    }
}

/// Node styles for the physics simulation
pub struct NodeStyles;

impl NodeStyles {
    pub fn text_node() -> String {
        format!(
            "color: {}; font-size: {}; text-align: center; pointer-events: none;",
            DARK_THEME.text_primary,
            TYPOGRAPHY.body_sm
        )
    }
    
    pub fn link_node() -> String {
        format!(
            "color: {}; text-decoration: none; font-size: {};",
            DARK_THEME.link_color,
            TYPOGRAPHY.body_sm
        )
    }
    
    pub fn author_image() -> String {
        format!(
            "width: 100%; height: 100%; border-radius: {}; object-fit: cover;",
            BORDER_RADIUS.full
        )
    }
    
    pub fn node_circle(size: f64) -> String {
        format!(
            "width: {}px; height: {}px; background-color: slateblue; border-radius: {}; transform: translate(-50%, -50%); position: absolute;",
            size,
            size,
            BORDER_RADIUS.full
        )
    }
    
    pub fn connection_line() -> String {
        format!(
            "stroke: {}; stroke-width: 1.5; opacity: 0.6;",
            DARK_THEME.text_muted
        )
    }
}

/// Debug panel styles
pub struct DebugStyles;

impl DebugStyles {
    pub fn panel() -> String {
        format!(
            "position: fixed; top: 10px; right: 10px; background: {}; color: {}; padding: {}; border-radius: {}; z-index: {}; max-width: 300px; max-height: 80vh; overflow-y: auto;",
            DARK_THEME.debug_bg,
            DARK_THEME.debug_text,
            SPACING.lg,
            BORDER_RADIUS.md,
            Z_INDEX.modal
        )
    }
    
    pub fn title() -> String {
        format!(
            "margin: 0 0 {} 0; color: {}; font-size: {};",
            SPACING.md,
            DARK_THEME.debug_accent,
            TYPOGRAPHY.heading_sm
        )
    }
    
    pub fn section() -> String {
        format!(
            "margin-bottom: {}; border-top: 1px solid {}; padding-top: {};",
            SPACING.lg,
            DARK_THEME.text_muted,
            SPACING.md
        )
    }
    
    pub fn label() -> String {
        format!(
            "display: block; margin-bottom: {}; color: {}; font-size: {};",
            SPACING.xs,
            DARK_THEME.text_secondary,
            TYPOGRAPHY.body_sm
        )
    }
    
    pub fn value() -> String {
        format!(
            "color: {}; font-weight: bold;",
            DARK_THEME.warning
        )
    }
    
    pub fn checkbox_label() -> String {
        format!(
            "display: flex; align-items: center; gap: {}; color: {}; font-size: {};",
            SPACING.sm,
            DARK_THEME.text_secondary,
            TYPOGRAPHY.body_sm
        )
    }
    
    pub fn stat_text() -> String {
        format!(
            "margin: {} 0; color: {}; font-size: {};",
            SPACING.xs,
            DARK_THEME.success,
            TYPOGRAPHY.caption
        )
    }
}

/// Article styles
pub struct ArticleStyles;

impl ArticleStyles {
    pub fn container() -> String {
        format!(
            "max-width: 800px; margin: 0 auto; padding: {}; color: {}; line-height: 1.6;",
            SPACING.xl,
            DARK_THEME.text_primary
        )
    }
    
    pub fn title() -> String {
        format!(
            "font-size: {}; font-weight: bold; margin-bottom: {}; color: {};",
            TYPOGRAPHY.heading_lg,
            SPACING.lg,
            DARK_THEME.text_primary
        )
    }
    
    pub fn meta() -> String {
        format!(
            "color: {}; font-size: {}; margin-bottom: {};",
            DARK_THEME.text_muted,
            TYPOGRAPHY.body_sm,
            SPACING.lg
        )
    }
    
    pub fn content() -> String {
        format!(
            "font-size: {}; line-height: 1.8;",
            TYPOGRAPHY.body_md
        )
    }
    
    pub fn wiki_link() -> String {
        format!(
            "color: {}; text-decoration: underline; font-weight: 500;",
            DARK_THEME.link_color
        )
    }
}

/// Error styles
pub struct ErrorStyles;

impl ErrorStyles {
    pub fn container() -> String {
        format!(
            "display: flex; justify-content: center; align-items: center; height: 100vh; background: {};",
            "#f0f0f0"
        )
    }
    
    pub fn content() -> String {
        format!(
            "text-align: center; color: {}; padding: {};",
            DARK_THEME.error,
            SPACING.xl
        )
    }
    
    pub fn title() -> String {
        format!(
            "font-size: {}; margin-bottom: {};",
            TYPOGRAPHY.heading_md,
            SPACING.md
        )
    }
    
    pub fn message() -> String {
        format!(
            "font-size: {}; color: {};",
            TYPOGRAPHY.body_md,
            DARK_THEME.text_muted
        )
    }
}