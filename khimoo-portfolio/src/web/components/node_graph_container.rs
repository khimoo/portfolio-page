use crate::config::get_config;
use crate::web::components::node_data_manager::NodeDataManager;
use crate::web::components::physics_renderer::PhysicsRenderer;
use crate::web::data_loader::use_articles_data;
use crate::web::physics_sim::{PhysicsWorld, Viewport};
use crate::web::routes::Route;
use crate::web::styles::{ErrorStyles, LoadingStyles};
use crate::web::types::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::UseMeasureState;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NodeGraphContainerProps {
    pub container_ref: NodeRef,
    pub container_measure: UseMeasureState,
    pub container_bound: ContainerBound,
}

#[function_component(NodeGraphContainer)]
pub fn node_graph_container(props: &NodeGraphContainerProps) -> Html {
    let force_settings = use_state(ForceSettings::default);
    let viewport = use_state(Viewport::default);

    // データローダーを使用して記事データを取得
    let (articles_data, loading, error) = use_articles_data();

    // 記事データが読み込まれたらノードレジストリと物理世界を一度だけ初期化
    let node_registry = use_state(|| Rc::new(RefCell::new(NodeRegistry::new_with_config(get_config().node_config.clone()))));
    let node_slug_mapping = use_state(|| HashMap::<NodeId, String>::new());
    let physics_world = use_state(|| {
        let empty_registry = Rc::new(RefCell::new(NodeRegistry::new_with_config(get_config().node_config.clone())));
        let default_bound = ContainerBound::default();
        Rc::new(RefCell::new(PhysicsWorld::new(
            empty_registry,
            &viewport,
            *force_settings,
            default_bound,
        )))
    });

    // 記事データが初回読み込まれた時のみ初期化
    let initialized = use_state(|| false);
    if let Some(data) = articles_data.as_ref() {
        if !*initialized {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(
                &format!(
                    "Initializing with container_bound: {:?}",
                    props.container_bound
                )
                .into(),
            );

            let (new_registry, slug_mapping) =
                NodeDataManager::create_node_registry_from_articles(data, &props.container_bound);
            let registry_rc = Rc::new(RefCell::new(new_registry));
            node_registry.set(Rc::clone(&registry_rc));
            node_slug_mapping.set(slug_mapping);

            let new_physics_world = PhysicsWorld::new(
                registry_rc,
                &viewport,
                *force_settings,
                props.container_bound.clone(),
            );
            physics_world.set(Rc::new(RefCell::new(new_physics_world)));
            initialized.set(true);
        }
    }

    // ノードクリック時のナビゲーション処理
    let navigator = use_navigator().unwrap();
    let on_node_click = {
        let navigator = navigator.clone();
        let node_slug_mapping = node_slug_mapping.clone();

        Callback::from(move |node_id: NodeId| {
            if let Some(slug) = node_slug_mapping.get(&node_id) {
                // フォールバック作者ノードの場合はホームに留まる
                if slug == "author" {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(
                        &"Fallback author node clicked - staying on home page".into(),
                    );
                    return;
                }

                // 記事ページに遷移
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("Navigating to article: {}", slug).into());
                let route = Route::ArticleShow { slug: slug.clone() };
                navigator.push(&route);
            }
        })
    };

    // ローディング中やエラー時の表示
    if *loading {
        return html! {
            <div style={LoadingStyles::container()}>
                <div style={LoadingStyles::text()}>
                    <h2>{"記事データを読み込み中..."}</h2>
                    <div style="margin-top: 20px;">
                        <div style={LoadingStyles::spinner()}></div>
                    </div>
                </div>
            </div>
        };
    }

    if let Some(err) = error.as_ref() {
        return html! {
            <div style={ErrorStyles::container()}>
                <div style={ErrorStyles::content()}>
                    <h2 style={ErrorStyles::title()}>{"データの読み込みに失敗しました"}</h2>
                    <p style={ErrorStyles::message()}>{format!("エラー: {}", err)}</p>
                </div>
            </div>
        };
    }

    html! {
        <PhysicsRenderer
            node_registry={(*node_registry).clone()}
            physics_world={(*physics_world).clone()}
            container_bound={props.container_bound.clone()}
            container_ref={props.container_ref.clone()}
            on_node_click={on_node_click}
        />
    }
}
