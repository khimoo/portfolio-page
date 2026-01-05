/// 物理シミュレーション関連の型定義
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForceSettings {
    pub repulsion_strength: f32,
    pub repulsion_min_distance: f32,
    pub author_repulsion_min_distance: f32,
    pub link_strength: f32,
    pub center_strength: f32,
    pub center_damping: f32,
    pub direct_link_strength: f32,
    pub direct_link_damping: f32,
    pub debug_mode: bool,
    pub show_connection_lines: bool,
    pub category_attraction_strength: f32,
    pub category_attraction_range: f32,
    pub enable_category_clustering: bool,
}

impl Default for ForceSettings {
    fn default() -> Self {
        Self {
            repulsion_strength: 68000.0,
            repulsion_min_distance: 150.0,
            author_repulsion_min_distance: 150.0,
            link_strength: 5000.0,
            center_strength: 6000.0,
            center_damping: 5.0,
            direct_link_strength: 8000.0,
            direct_link_damping: 300.0,
            debug_mode: false,
            show_connection_lines: true,
            category_attraction_strength: 1500.0,
            category_attraction_range: 300.0,
            enable_category_clustering: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ContainerBound {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}