use crate::web::styles::NodeStyles;
use crate::web::types::*;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NodeRendererProps {
    pub node_registry: Rc<RefCell<NodeRegistry>>,
    pub on_mouse_down: Callback<(NodeId, MouseEvent)>,
}

#[function_component(NodeRenderer)]
pub fn node_renderer(props: &NodeRendererProps) -> Html {
    let registry = props.node_registry.borrow();
    
    html! {
        <>
            // 背景のエッジ描画
            <svg style="position: absolute; left: 0; top: 0; width: 100%; height: 100%; z-index: 1; pointer-events: none;">
                {
                    registry.iter_edges().filter_map(|(a, b)| {
                        let p1 = registry.positions.get(a)?;
                        let p2 = registry.positions.get(b)?;
                        Some(html!{
                            <line
                                x1={format!("{:.2}", p1.x)}
                                y1={format!("{:.2}", p1.y)}
                                x2={format!("{:.2}", p2.x)}
                                y2={format!("{:.2}", p2.y)}
                                stroke="#8a8a8a"
                                stroke-width="1.5"
                                style={NodeStyles::connection_line()}
                            />
                        })
                    }).collect::<Html>()
                }
            </svg>
            
            // ノード描画
            {
                registry.iter().map(|(id, pos, radius, content)| {
                    let importance = registry.get_node_importance(*id);
                    let inbound_count = registry.get_node_inbound_count(*id);

                    let on_mouse_down = {
                        let on_mouse_down = props.on_mouse_down.clone();
                        let id = *id;
                        Callback::from(move |e: MouseEvent| {
                            e.stop_propagation();
                            on_mouse_down.emit((id, e));
                        })
                    };

                    html!{
                        <NodeComponent
                            key={id.0}
                            id={*id}
                            pos={*pos}
                            radius={*radius}
                            content={content.clone()}
                            {importance}
                            {inbound_count}
                            {on_mouse_down}
                        />
                    }
                }).collect::<Html>()
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct NodeProps {
    pub id: NodeId,
    pub pos: Position,
    pub radius: i32,
    pub content: NodeContent,
    pub on_mouse_down: Callback<MouseEvent>,
    pub importance: Option<u8>,
    pub inbound_count: usize,
}

#[function_component(NodeComponent)]
pub fn node_component(props: &NodeProps) -> Html {
    // Author画像の場合は画像がノード全体を覆うようにする
    let content_container_style = match &props.content {
        NodeContent::Author { .. } => "width: 80%; height: 80%; object-fit: contain; overflow: hidden; pointer-events: none;",
        _ => "max-width: 80%; max-height: 80%; overflow: hidden; pointer-events: none;",
    };

    html! {
        <div
            key={props.id.0.to_string()}
            onmousedown={props.on_mouse_down.clone()}
            style={format!(
                "{} left: {}px; top: {}px; box-shadow: 0 4px 8px rgba(0,0,0,0.2); z-index: 10; display: flex; justify-content: center; align-items: center; position: absolute; cursor: pointer; transition: transform 0.2s ease-in-out; user-select: none;",
                NodeStyles::node_circle(props.radius as f64 * 2.0),
                props.pos.x,
                props.pos.y
            )}
        >
            <div style={content_container_style}>
                {props.content.render_content()}
            </div>
        </div>
    }
}