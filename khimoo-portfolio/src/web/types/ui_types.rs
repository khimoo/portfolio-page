/// UI関連の型定義
use yew_hooks::UseMeasureState;
use yew::NodeRef;

/// コンポーネントのプロパティ型
#[derive(yew::Properties, PartialEq)]
pub struct ContainerProps {
    pub container_ref: NodeRef,
    pub container_measure: UseMeasureState,
    pub container_bound: super::physics_types::ContainerBound,
}

/// ローディング状態
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

/// UI状態管理
#[derive(Debug, Clone, PartialEq)]
pub struct UIState {
    pub loading: LoadingState,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
}