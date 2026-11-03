use crate::config::get_config;
use crate::web::components::debug_panel::DebugPanel;
use crate::web::config::StyleConfig;
use crate::web::components::node_renderer::NodeRenderer;
use crate::web::physics_sim::{PhysicsWorld, Viewport};
use crate::web::styles::{AnimationStyles, LayoutStyles};
use crate::web::types::*;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::{use_effect_update_with_deps, use_interval};

#[derive(Properties)]
pub struct PhysicsRendererProps {
    pub node_registry: Rc<RefCell<NodeRegistry>>,
    pub physics_world: Rc<RefCell<PhysicsWorld>>,
    pub container_bound: ContainerBound,
    pub container_ref: NodeRef,
    pub on_node_click: Callback<NodeId>,
}

impl PartialEq for PhysicsRendererProps {
    fn eq(&self, other: &Self) -> bool {
        // Rc<RefCell<T>>の比較は参照の比較のみ行う
        Rc::ptr_eq(&self.node_registry, &other.node_registry)
            && Rc::ptr_eq(&self.physics_world, &other.physics_world)
            && self.container_bound == other.container_bound
            && self.container_ref == other.container_ref
    }
}

/// ドラッグ中のポインタ状態
#[derive(Default)]
struct DragState {
    node_id: Option<NodeId>,
    pointer_id: Option<i32>,
    start_pos: Option<(i32, i32)>,
    is_dragging: bool,
}

/// タッチ操作では指のブレが大きいので閾値を少し大きくする
fn drag_threshold(e: &PointerEvent) -> f32 {
    if e.pointer_type() == "mouse" {
        5.0
    } else {
        10.0
    }
}

/// ポインタのクライアント座標をコンテナ内のローカル座標に変換
fn to_container_position(container_ref: &NodeRef, e: &PointerEvent) -> Position {
    let (left, top) = container_ref
        .cast::<web_sys::Element>()
        .map(|el| {
            let rect = el.get_bounding_client_rect();
            (rect.left() as f32, rect.top() as f32)
        })
        .unwrap_or((0.0, 0.0));
    Position {
        x: e.client_x() as f32 - left,
        y: e.client_y() as f32 - top,
    }
}

#[function_component(PhysicsRenderer)]
pub fn physics_renderer(props: &PhysicsRendererProps) -> Html {
    let drag_state = use_mut_ref(DragState::default);
    let viewport = use_state(Viewport::default);
    let force_settings = use_state(ForceSettings::default);

    // 力の設定が変更されたらPhysicsWorldを更新
    {
        let physics_world = props.physics_world.clone();
        let force_settings_clone = force_settings.clone();
        use_effect_update_with_deps(
            move |_| {
                physics_world
                    .borrow_mut()
                    .update_force_settings(*force_settings_clone);
                || {}
            },
            force_settings.clone(),
        );
    }

    // コンテナ境界が変更されたらPhysicsWorldを更新
    {
        let physics_world = props.physics_world.clone();
        use_effect_update_with_deps(
            move |container_bound| {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(
                    &format!("Container bound changed in effect: {container_bound:?}").into(),
                );
                physics_world
                    .borrow_mut()
                    .update_container_bound(container_bound.clone());
                || {}
            },
            props.container_bound.clone(),
        );
    }

    // ポインタ移動処理（マウス・タッチ・ペン共通）
    let on_pointer_move = {
        let drag_state = drag_state.clone();
        let physics_world = props.physics_world.clone();
        let viewport = viewport.clone();
        let container_ref = props.container_ref.clone();

        Callback::from(move |e: PointerEvent| {
            let mut state = drag_state.borrow_mut();
            if state.pointer_id != Some(e.pointer_id()) {
                return;
            }
            let (Some(id), Some((start_x, start_y))) = (state.node_id, state.start_pos) else {
                return;
            };

            let dx = e.client_x() - start_x;
            let dy = e.client_y() - start_y;
            let distance = ((dx * dx + dy * dy) as f32).sqrt();

            // 閾値以上移動したらドラッグ開始
            if !state.is_dragging && distance > drag_threshold(&e) {
                state.is_dragging = true;
                physics_world.borrow_mut().set_node_kinematic(id);
            }

            // ドラッグ中の場合のみノード位置を更新
            if state.is_dragging {
                e.prevent_default();
                let pos = to_container_position(&container_ref, &e);
                physics_world
                    .borrow_mut()
                    .set_node_position(id, &pos, &viewport);
            }
        })
    };

    // ポインタダウン処理
    let on_pointer_down = {
        let drag_state = drag_state.clone();

        Callback::from(move |(id, e): (NodeId, PointerEvent)| {
            let mut state = drag_state.borrow_mut();
            // マルチタッチ時は最初の指だけを扱う
            if state.pointer_id.is_some() {
                return;
            }
            if e.pointer_type() == "mouse" && e.button() != 0 {
                return;
            }
            // テキスト選択や互換マウスイベントの発生を防ぐ
            e.prevent_default();
            // ノード外にポインタが出てもイベントを受け取り続ける
            if let Some(target) = e.target_dyn_into::<web_sys::Element>() {
                let _ = target.set_pointer_capture(e.pointer_id());
            }
            *state = DragState {
                node_id: Some(id),
                pointer_id: Some(e.pointer_id()),
                start_pos: Some((e.client_x(), e.client_y())),
                is_dragging: false,
            };
        })
    };

    // ポインタアップ処理（ドラッグしていなければクリックとして扱う）
    let on_pointer_up = {
        let drag_state = drag_state.clone();
        let physics_world = props.physics_world.clone();
        let on_node_click = props.on_node_click.clone();

        Callback::from(move |e: PointerEvent| {
            let finished = {
                let mut state = drag_state.borrow_mut();
                if state.pointer_id != Some(e.pointer_id()) {
                    return;
                }
                std::mem::take(&mut *state)
            };

            if let Some(id) = finished.node_id {
                if finished.is_dragging {
                    physics_world.borrow_mut().set_node_dynamic(id);
                } else {
                    on_node_click.emit(id);
                }
            }
        })
    };

    // ポインタキャンセル処理（スクロール開始・通知割り込みなど）
    let on_pointer_cancel = {
        let drag_state = drag_state.clone();
        let physics_world = props.physics_world.clone();

        Callback::from(move |e: PointerEvent| {
            let finished = {
                let mut state = drag_state.borrow_mut();
                if state.pointer_id != Some(e.pointer_id()) {
                    return;
                }
                std::mem::take(&mut *state)
            };
            if let (Some(id), true) = (finished.node_id, finished.is_dragging) {
                physics_world.borrow_mut().set_node_dynamic(id);
            }
        })
    };

    // 物理シミュレーションのステップ実行
    let rerender = use_state(|| ());
    {
        let physics_world = props.physics_world.clone();
        let viewport = viewport.clone();
        let rerender = rerender.clone();

        use_interval(
            move || {
                let mut world = physics_world.borrow_mut();
                world.step(&viewport);
                rerender.set(());
            },
            8, // ~120fps
        );
    }

    // 力の設定変更コールバック
    let on_settings_change = {
        let force_settings = force_settings.clone();
        Callback::from(move |new_settings: ForceSettings| {
            force_settings.set(new_settings);
        })
    };

    // ウェルカム文の位置: デスクトップは従来通り、モバイルでは縮小した作者ノードの直下に置く
    let welcome_top = {
        let bound = &props.container_bound;
        if StyleConfig::is_mobile_width(bound.width) {
            let author_radius = get_config().node_config.author_node_radius as f32
                * StyleConfig::node_scale_for_width(bound.width);
            (bound.height / 2.0 + author_radius + 16.0) as i32
        } else {
            (bound.height / 2.0 + 200.0) as i32
        }
    };

    html! {
        <>
            <style>
                {AnimationStyles::spinner_keyframes()}
            </style>
            <div
                class="physics-container"
                style={LayoutStyles::physics_container()}
                onpointermove={on_pointer_move}
                onpointerup={on_pointer_up}
                onpointercancel={on_pointer_cancel}
                ref={props.container_ref.clone()}
            >
                // ウェルカムオーバーレイていうんや
                <div class="welcome-overlay" style={format!(
                    "{} top: {}px;",
                    LayoutStyles::welcome_overlay(),
                    welcome_top
                )}>
                    <span style="display:flex; flex-direction: column; margin-bottom:12px">
                        <span class="welcome-name" style="font-size: 32px; font-weight: bold;">{"日比野 文"}</span>
                        <span class="welcome-name-en" style="font-size: 16px; font-weight: bold;">{"Bun Hibino"}</span>
                    </span>
                    <div class="welcome-body" style="white-space: nowrap; margin: 10px 0; line-height: 1.5;">
                        {"筑波大学 理工情報生命学術院 数理物質科学研究群 数学学位プログラム"}<br/>
                        {"専門：幾何学/連続体理論"}<br/>
                        {"Rust, neovim, NixOS, HoTTにも興味があります！"}
                    </div>
                </div>

                // デバッグ情報（デバッグビルド時のみ）
                {
                    if cfg!(debug_assertions) {
                        html! { <p>{ format!("{:?}", props.container_bound) }</p> }
                    } else {
                        html! {}
                    }
                }

                // デバッグパネル
                <DebugPanel
                    force_settings={*force_settings}
                    on_settings_change={on_settings_change}
                />

                // ノード描画
                <NodeRenderer
                    node_registry={props.node_registry.clone()}
                    on_pointer_down={on_pointer_down}
                />
            </div>
        </>
    }
}
