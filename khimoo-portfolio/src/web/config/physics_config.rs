use crate::web::types::ForceSettings;

/// 物理シミュレーションの設定を一元管理
pub struct PhysicsConfig;

impl PhysicsConfig {
    /// デフォルトの物理設定
    pub fn default_force_settings() -> ForceSettings {
        ForceSettings::default()
    }

    /// 物理シミュレーションのフレームレート設定
    pub const PHYSICS_FPS: u32 = 60;
    pub const PHYSICS_INTERVAL_MS: u32 = 300 / Self::PHYSICS_FPS;
}