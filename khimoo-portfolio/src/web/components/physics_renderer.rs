use crate::web::components::debug_panel::DebugPanel;
use crate::web::components::node_renderer::NodeRenderer;
use crate::web::physics_sim::{PhysicsWorld, Viewport};
use crate::web::styles::{AnimationStyles, LayoutStyles};
use crate::web::types::*;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::{use_effect_update_with_deps, use_interval, use_window_scroll};

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

#[function_component(PhysicsRenderer)]
pub fn physics_renderer(props: &PhysicsRendererProps) -> Html {
    let dragged_node_id = use_state(|| None::<NodeId>);
    let viewport = use_state(Viewport::default);
    let force_settings = use_state(ForceSettings::default);
    let drag_start_pos = use_state(|| None::<(i32, i32)>);
    let is_dragging = use_state(|| false);
    let scroll = use_window_scroll();

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
                    &format!("Container bound changed in effect: {:?}", container_bound).into(),
                );
                physics_world
                    .borrow_mut()
                    .update_container_bound(container_bound.clone());
                || {}
            },
            props.container_bound.clone(),
        );
    }

    // マウス移動処理
    let on_mouse_move = {
        let dragged_node_id = dragged_node_id.clone();
        let physics_world = props.physics_world.clone();
        let viewport = viewport.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(id) = *dragged_node_id {
                if let Some((start_x, start_y)) = *drag_start_pos {
                    let dx = e.client_x() - start_x;
                    let dy = e.client_y() - start_y;
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();

                    // 5px以上移動したらドラッグ開始
                    if distance > 5.0 && !*is_dragging {
                        is_dragging.set(true);
                        physics_world.borrow_mut().set_node_kinematic(id);
                    }

                    // ドラッグ中の場合のみノード位置を更新
                    if *is_dragging {
                        let mut world = physics_world.borrow_mut();
                        let screen_pos = Position {
                            x: (e.client_x() + scroll.0 as i32) as f32,
                            y: (e.client_y() + scroll.1 as i32) as f32,
                        };
                        world.set_node_position(id, &screen_pos, &viewport);
                    }
                }
            }
        })
    };

    // マウスダウン処理
    let on_mouse_down = {
        let dragged_node_id = dragged_node_id.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();

        Callback::from(move |(id, e): (NodeId, MouseEvent)| {
            drag_start_pos.set(Some((e.client_x(), e.client_y())));
            is_dragging.set(false);
            dragged_node_id.set(Some(id));
        })
    };

    // マウスアップ処理
    let on_mouse_up = {
        let dragged_node_id = dragged_node_id.clone();
        let physics_world = props.physics_world.clone();
        let drag_start_pos = drag_start_pos.clone();
        let is_dragging = is_dragging.clone();
        let on_node_click = props.on_node_click.clone();

        Callback::from(move |_: MouseEvent| {
            if let Some(id) = *dragged_node_id {
                if *is_dragging {
                    physics_world.borrow_mut().set_node_dynamic(id);
                } else {
                    on_node_click.emit(id);
                }
            }

            dragged_node_id.set(None);
            drag_start_pos.set(None);
            is_dragging.set(false);
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

    html! {
        <>
            <style>
                {AnimationStyles::spinner_keyframes()}
            </style>
            <div
                style={LayoutStyles::physics_container()}
                onmousemove={on_mouse_move}
                onmouseup={on_mouse_up}
                ref={props.container_ref.clone()}
            >
                // ウェルカムオーバーレイていうんや
                <div style={format!(
                    "{} top: {}px;",
                    LayoutStyles::welcome_overlay(),
                    (props.container_bound.height / 2.0 + 200.0) as i32
                )}>
                    <span style="display:flex; flex-direction: column; margin-bottom:12px">
                        <span style="font-size: 32px; font-weight: bold;">{"日比野 文"}</span>
                        <span style="font-size: 16px; font-weight: bold;">{"Bun Hibino"}</span>
                    </span>
                    <div style="white-space: nowrap; margin: 10px 0; line-height: 1.5;">
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
                    on_mouse_down={on_mouse_down}
                />
            </div>
        </>
    }
}
