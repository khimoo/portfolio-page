use crate::web::types::ForceSettings;

/// 物理シミュレーションの設定を一元管理
pub struct PhysicsConfig;

impl PhysicsConfig {
    /// デフォルトの物理設定
    pub fn default_force_settings() -> ForceSettings {
        ForceSettings::default()
    }

    /// 高性能設定（軽量）
    pub fn performance_force_settings() -> ForceSettings {
        ForceSettings {
            repulsion_strength: 50000.0,
            repulsion_min_distance: 100.0,
            author_repulsion_min_distance: 120.0,
            link_strength: 3000.0,
            center_strength: 4000.0,
            center_damping: 3.0,
            direct_link_strength: 6000.0,
            direct_link_damping: 200.0,
            debug_mode: false,
            show_connection_lines: false,
            category_attraction_strength: 1000.0,
            category_attraction_range: 250.0,
            enable_category_clustering: true,
        }
    }

    /// 高品質設定（重い）
    pub fn quality_force_settings() -> ForceSettings {
        ForceSettings {
            repulsion_strength: 80000.0,
            repulsion_min_distance: 180.0,
            author_repulsion_min_distance: 200.0,
            link_strength: 7000.0,
            center_strength: 8000.0,
            center_damping: 8.0,
            direct_link_strength: 10000.0,
            direct_link_damping: 400.0,
            debug_mode: false,
            show_connection_lines: true,
            category_attraction_strength: 2000.0,
            category_attraction_range: 350.0,
            enable_category_clustering: true,
        }
    }

    /// 物理シミュレーションのフレームレート設定
    pub const PHYSICS_FPS: u32 = 120;
    pub const PHYSICS_INTERVAL_MS: u32 = 1000 / Self::PHYSICS_FPS;

    /// ノードサイズの設定
    pub const MIN_NODE_RADIUS: i32 = 20;
    pub const MAX_NODE_RADIUS: i32 = 80;
    pub const DEFAULT_NODE_RADIUS: i32 = 30;
    pub const AUTHOR_NODE_RADIUS: i32 = 60;
}