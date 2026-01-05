use super::theme::*;

/// Layout utility styles
pub struct LayoutStyles;

impl LayoutStyles {
    /// Main application wrapper
    pub fn app_wrapper() -> String {
        format!(
            "height: 100vh; margin: 0; padding: 0; display: flex; flex-direction: column; background: {};",
            DARK_THEME.primary_bg
        )
    }
    
    /// Header layout
    pub fn header() -> String {
        format!(
            "margin: 0; background: {}; padding: 0; z-index: {};",
            DARK_THEME.primary_bg,
            Z_INDEX.sticky
        )
    }
    
    pub fn header_container() -> String {
        format!(
            "max-width: 1000px; margin: auto; display: flex; align-items: flex-end; gap: {}; padding: {};",
            SPACING.md,
            SPACING.md
        )
    }
    
    pub fn header_nav() -> String {
        format!(
            "margin-left: auto; display: flex; gap: {}; align-items: flex-end;",
            SPACING.md
        )
    }
    
    /// Main content area
    pub fn main_content() -> String {
        format!(
            "display: flex; width: 100%; height: 100%; background: {};",
            DARK_THEME.primary_bg
        )
    }
    
    /// Centered content
    pub fn centered_container() -> String {
        format!(
            "display: flex; justify-content: center; align-items: center; min-height: 100vh; padding: {};",
            SPACING.xl
        )
    }
    
    /// Full viewport overlay
    pub fn overlay() -> String {
        format!(
            "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0, 0, 0, 0.5); z-index: {}; display: flex; justify-content: center; align-items: center;",
            Z_INDEX.modal_backdrop
        )
    }
    
    /// Physics simulation container
    pub fn physics_container() -> String {
        format!(
            "display: flex; width: 100%; height: 100%; background: {}; position: relative;",
            DARK_THEME.primary_bg
        )
    }
    
    /// Welcome message overlay
    pub fn welcome_overlay() -> String {
        format!(
            "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center; font-size: {}; color: {}; z-index: {}; backdrop-filter: blur(10px); pointer-events: none; padding: {}; border-radius: {};",
            TYPOGRAPHY.heading_md,
            DARK_THEME.text_primary,
            Z_INDEX.popover,
            SPACING.xl,
            BORDER_RADIUS.lg
        )
    }
}

/// Responsive breakpoints and utilities
pub struct ResponsiveStyles;

impl ResponsiveStyles {
    /// Mobile-first responsive container
    pub fn responsive_container() -> String {
        format!(
            "width: 100%; max-width: 1200px; margin: 0 auto; padding: 0 {}; box-sizing: border-box;",
            SPACING.md
        )
    }
    
    /// Mobile styles (up to 768px)
    pub fn mobile_only() -> &'static str {
        "@media (max-width: 768px)"
    }
    
    /// Tablet styles (769px to 1024px)
    pub fn tablet_only() -> &'static str {
        "@media (min-width: 769px) and (max-width: 1024px)"
    }
    
    /// Desktop styles (1025px and up)
    pub fn desktop_only() -> &'static str {
        "@media (min-width: 1025px)"
    }
    
    /// Hide on mobile
    pub fn hide_mobile() -> String {
        format!(
            "{} {{ display: none; }}",
            Self::mobile_only()
        )
    }
    
    /// Show only on mobile
    pub fn show_mobile_only() -> String {
        format!(
            "display: none; {} {{ display: block; }}",
            Self::mobile_only()
        )
    }
}

/// Grid system utilities
pub struct GridStyles;

impl GridStyles {
    /// Basic grid container
    pub fn grid_container() -> String {
        format!(
            "display: grid; gap: {}; width: 100%;",
            SPACING.lg
        )
    }
    
    /// Two column grid
    pub fn two_column() -> String {
        format!(
            "{} grid-template-columns: 1fr 1fr;",
            Self::grid_container()
        )
    }
    
    /// Three column grid
    pub fn three_column() -> String {
        format!(
            "{} grid-template-columns: repeat(3, 1fr);",
            Self::grid_container()
        )
    }
    
    /// Auto-fit grid with minimum column width
    pub fn auto_fit_grid(min_width: &str) -> String {
        format!(
            "{} grid-template-columns: repeat(auto-fit, minmax({}, 1fr));",
            Self::grid_container(),
            min_width
        )
    }
}

/// Flexbox utilities
pub struct FlexStyles;

impl FlexStyles {
    /// Basic flex container
    pub fn flex_container() -> String {
        "display: flex;".to_string()
    }
    
    /// Flex column
    pub fn flex_column() -> String {
        format!(
            "{} flex-direction: column;",
            Self::flex_container()
        )
    }
    
    /// Flex row with gap
    pub fn flex_row_gap(gap: &str) -> String {
        format!(
            "{} gap: {};",
            Self::flex_container(),
            gap
        )
    }
    
    /// Flex column with gap
    pub fn flex_column_gap(gap: &str) -> String {
        format!(
            "{} flex-direction: column; gap: {};",
            Self::flex_container(),
            gap
        )
    }
    
    /// Center content both ways
    pub fn center_content() -> String {
        format!(
            "{} justify-content: center; align-items: center;",
            Self::flex_container()
        )
    }
    
    /// Space between items
    pub fn space_between() -> String {
        format!(
            "{} justify-content: space-between; align-items: center;",
            Self::flex_container()
        )
    }
    
    /// Flex grow
    pub fn flex_grow() -> &'static str {
        "flex: 1;"
    }
}