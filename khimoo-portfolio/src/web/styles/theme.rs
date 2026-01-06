use std::collections::HashMap;

/// Core color palette for the application
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // Primary colors
    pub primary_bg: &'static str,
    pub secondary_bg: &'static str,
    pub surface: &'static str,

    // Text colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,

    // Accent colors
    pub accent_blue: &'static str,
    pub accent_green: &'static str,
    pub accent_orange: &'static str,
    pub accent_red: &'static str,
    pub accent_purple: &'static str,

    // Interactive colors
    pub link_color: &'static str,
    pub link_hover: &'static str,

    // Status colors
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub info: &'static str,

    // Debug colors
    pub debug_bg: &'static str,
    pub debug_text: &'static str,
    pub debug_accent: &'static str,
}

/// Dark theme color palette (current default)
pub const DARK_THEME: ColorPalette = ColorPalette {
    // Primary colors
    primary_bg: "#081D35",
    secondary_bg: "#0F2A47",
    surface: "rgba(0, 0, 0, 0.8)",

    // Text colors
    text_primary: "#FFFFFF",
    text_secondary: "#E0E0E0",
    text_muted: "#999999",

    // Accent colors
    accent_blue: "#4A90E2",
    accent_green: "#7ED321",
    accent_orange: "#F5A623",
    accent_red: "#CE422B",
    accent_purple: "#BD10E0",

    // Interactive colors
    link_color: "lightblue",
    link_hover: "#87CEEB",

    // Status colors
    success: "#4CAF50",
    warning: "#FFC107",
    error: "#e74c3c",
    info: "#3498db",

    // Debug colors
    debug_bg: "rgba(0, 0, 0, 0.8)",
    debug_text: "#FFFFFF",
    debug_accent: "#4CAF50",
};

/// Category color configuration
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryColor {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

/// Get default category colors
pub fn get_default_category_colors() -> HashMap<String, CategoryColor> {
    let mut colors = HashMap::new();

    colors.insert(
        "programming".to_string(),
        CategoryColor {
            primary: DARK_THEME.accent_blue.to_string(),
            secondary: "#357ABD".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors.insert(
        "web".to_string(),
        CategoryColor {
            primary: DARK_THEME.accent_green.to_string(),
            secondary: "#5BA517".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors.insert(
        "rust".to_string(),
        CategoryColor {
            primary: DARK_THEME.accent_red.to_string(),
            secondary: "#A0341F".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors.insert(
        "design".to_string(),
        CategoryColor {
            primary: DARK_THEME.accent_purple.to_string(),
            secondary: "#9013B0".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors.insert(
        "tutorial".to_string(),
        CategoryColor {
            primary: DARK_THEME.accent_orange.to_string(),
            secondary: "#D1891C".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors.insert(
        "default".to_string(),
        CategoryColor {
            primary: "#9B9B9B".to_string(),
            secondary: "#7B7B7B".to_string(),
            text: DARK_THEME.text_primary.to_string(),
        },
    );

    colors
}

/// Typography scale
#[derive(Debug, Clone)]
pub struct Typography {
    pub heading_xl: &'static str,
    pub heading_lg: &'static str,
    pub heading_md: &'static str,
    pub heading_sm: &'static str,
    pub body_lg: &'static str,
    pub body_md: &'static str,
    pub body_sm: &'static str,
    pub caption: &'static str,
}

pub const TYPOGRAPHY: Typography = Typography {
    heading_xl: "30px",
    heading_lg: "24px",
    heading_md: "20px",
    heading_sm: "16px",
    body_lg: "16px",
    body_md: "14px",
    body_sm: "12px",
    caption: "11px",
};

/// Spacing scale
#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,
}

pub const SPACING: Spacing = Spacing {
    xs: "4px",
    sm: "8px",
    md: "12px",
    lg: "16px",
    xl: "24px",
    xxl: "32px",
};

/// Border radius values
#[derive(Debug, Clone)]
pub struct BorderRadius {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub full: &'static str,
}

pub const BORDER_RADIUS: BorderRadius = BorderRadius {
    sm: "4px",
    md: "8px",
    lg: "10px",
    full: "50%",
};

/// Z-index layers
#[derive(Debug, Clone)]
pub struct ZIndex {
    pub base: i32,
    pub dropdown: i32,
    pub sticky: i32,
    pub fixed: i32,
    pub modal_backdrop: i32,
    pub modal: i32,
    pub popover: i32,
    pub tooltip: i32,
}

pub const Z_INDEX: ZIndex = ZIndex {
    base: 0,
    dropdown: 100,
    sticky: 200,
    fixed: 300,
    modal_backdrop: 400,
    modal: 500,
    popover: 600,
    tooltip: 700,
};
