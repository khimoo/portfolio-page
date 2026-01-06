use super::layout::ResponsiveStyles;
use super::theme::*;

/// Animation utilities
pub struct AnimationStyles;

impl AnimationStyles {
    /// CSS keyframes for spinner animation
    pub fn spinner_keyframes() -> &'static str {
        r#"
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        "#
    }

    /// Fade in animation
    pub fn fade_in(duration: &str) -> String {
        format!(
            "animation: fadeIn {} ease-in-out; @keyframes fadeIn {{ 0% {{ opacity: 0; }} 100% {{ opacity: 1; }} }}",
            duration
        )
    }

    /// Slide in from top
    pub fn slide_in_top(duration: &str) -> String {
        format!(
            "animation: slideInTop {} ease-out; @keyframes slideInTop {{ 0% {{ transform: translateY(-20px); opacity: 0; }} 100% {{ transform: translateY(0); opacity: 1; }} }}",
            duration
        )
    }

    /// Smooth transition
    pub fn smooth_transition(properties: &str, duration: &str) -> String {
        format!("transition: {} {};", properties, duration)
    }
}

/// Shadow utilities
pub struct ShadowStyles;

impl ShadowStyles {
    pub fn small() -> &'static str {
        "box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);"
    }

    pub fn medium() -> &'static str {
        "box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16), 0 3px 6px rgba(0, 0, 0, 0.23);"
    }

    pub fn large() -> &'static str {
        "box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);"
    }

    pub fn inset() -> &'static str {
        "box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);"
    }
}

/// Text utilities
pub struct TextStyles;

impl TextStyles {
    pub fn truncate() -> &'static str {
        "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;"
    }

    pub fn line_clamp(lines: u32) -> String {
        format!(
            "display: -webkit-box; -webkit-line-clamp: {}; -webkit-box-orient: vertical; overflow: hidden;",
            lines
        )
    }

    pub fn no_select() -> &'static str {
        "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;"
    }

    pub fn break_word() -> &'static str {
        "word-wrap: break-word; word-break: break-word; hyphens: auto;"
    }
}

/// Positioning utilities
pub struct PositionStyles;

impl PositionStyles {
    pub fn absolute_center() -> &'static str {
        "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);"
    }

    pub fn absolute_top_left() -> &'static str {
        "position: absolute; top: 0; left: 0;"
    }

    pub fn absolute_top_right() -> &'static str {
        "position: absolute; top: 0; right: 0;"
    }

    pub fn absolute_bottom_left() -> &'static str {
        "position: absolute; bottom: 0; left: 0;"
    }

    pub fn absolute_bottom_right() -> &'static str {
        "position: absolute; bottom: 0; right: 0;"
    }

    pub fn fixed_full_screen() -> &'static str {
        "position: fixed; top: 0; left: 0; width: 100%; height: 100%;"
    }

    pub fn sticky_top() -> String {
        format!("position: sticky; top: 0; z-index: {};", Z_INDEX.sticky)
    }
}

/// Visibility utilities
pub struct VisibilityStyles;

impl VisibilityStyles {
    pub fn hidden() -> &'static str {
        "display: none;"
    }

    pub fn invisible() -> &'static str {
        "visibility: hidden;"
    }

    pub fn opacity_0() -> &'static str {
        "opacity: 0;"
    }

    pub fn opacity_50() -> &'static str {
        "opacity: 0.5;"
    }

    pub fn opacity_75() -> &'static str {
        "opacity: 0.75;"
    }

    pub fn screen_reader_only() -> &'static str {
        "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;"
    }
}

/// Interaction utilities
pub struct InteractionStyles;

impl InteractionStyles {
    pub fn clickable() -> &'static str {
        "cursor: pointer;"
    }

    pub fn not_allowed() -> &'static str {
        "cursor: not-allowed;"
    }

    pub fn grab() -> &'static str {
        "cursor: grab;"
    }

    pub fn grabbing() -> &'static str {
        "cursor: grabbing;"
    }

    pub fn pointer_events_none() -> &'static str {
        "pointer-events: none;"
    }

    pub fn pointer_events_auto() -> &'static str {
        "pointer-events: auto;"
    }
}

/// Helper functions for common style combinations
pub struct StyleHelpers;

impl StyleHelpers {
    /// Combine multiple style strings
    pub fn combine(styles: &[&str]) -> String {
        styles.join(" ")
    }

    /// Create a style string from key-value pairs
    pub fn from_pairs(pairs: &[(&str, &str)]) -> String {
        pairs
            .iter()
            .map(|(key, value)| format!("{}: {};", key, value))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Add conditional styles
    pub fn conditional(base: &str, condition: bool, conditional_style: &str) -> String {
        if condition {
            format!("{} {}", base, conditional_style)
        } else {
            base.to_string()
        }
    }

    /// Create responsive style with breakpoints
    pub fn responsive(mobile: &str, tablet: Option<&str>, desktop: Option<&str>) -> String {
        let mut style = mobile.to_string();

        if let Some(tablet_style) = tablet {
            style.push_str(&format!(
                " {} {{ {} }}",
                ResponsiveStyles::tablet_only(),
                tablet_style
            ));
        }

        if let Some(desktop_style) = desktop {
            style.push_str(&format!(
                " {} {{ {} }}",
                ResponsiveStyles::desktop_only(),
                desktop_style
            ));
        }

        style
    }
}
