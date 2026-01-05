use crate::web::types::ForceSettings;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DebugPanelProps {
    pub force_settings: ForceSettings,
    pub on_settings_change: Callback<ForceSettings>,
}

#[function_component(DebugPanel)]
pub fn debug_panel(props: &DebugPanelProps) -> Html {
    // デバッグビルド時のみ表示
    if !cfg!(debug_assertions) {
        return html! {};
    }

    let create_slider_callback = |field: &'static str| {
        let on_settings_change = props.on_settings_change.clone();
        let current_settings = props.force_settings;
        
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let value = target.value().parse::<f32>().unwrap_or(0.0);
            
            let mut new_settings = current_settings;
            match field {
                "repulsion_strength" => new_settings.repulsion_strength = value,
                "repulsion_min_distance" => new_settings.repulsion_min_distance = value,
                "author_repulsion_min_distance" => new_settings.author_repulsion_min_distance = value,
                "link_strength" => new_settings.link_strength = value,
                "center_strength" => new_settings.center_strength = value,
                "center_damping" => new_settings.center_damping = value,
                _ => {}
            }
            on_settings_change.emit(new_settings);
        })
    };

    html! {
        <div style="position: absolute; top: 20px; right: 20px; background: rgba(0,0,0,0.8); color: white; padding: 20px; border-radius: 10px; z-index: 100;">
            <h3 style="margin: 0 0 15px 0;">{"力の設定"}</h3>
            
            <div style="margin-bottom: 15px;">
                <label>{"反発力の強さ: "}{props.force_settings.repulsion_strength as i32}</label><br/>
                <input
                    type="range"
                    min="0"
                    max="200000"
                    step="1000"
                    value={props.force_settings.repulsion_strength.to_string()}
                    onchange={create_slider_callback("repulsion_strength")}
                    style="width: 200px;"
                />
            </div>
            
            <div style="margin-bottom: 15px;">
                <label>{"反発力の最小距離: "}{props.force_settings.repulsion_min_distance as i32}</label><br/>
                <input
                    type="range"
                    min="0"
                    max="1000"
                    step="5"
                    value={props.force_settings.repulsion_min_distance.to_string()}
                    onchange={create_slider_callback("repulsion_min_distance")}
                    style="width: 200px;"
                />
            </div>
            
            <div style="margin-bottom: 15px;">
                <label>{"作者ノード反発距離: "}{props.force_settings.author_repulsion_min_distance as i32}</label><br/>
                <input
                    type="range"
                    min="50"
                    max="500"
                    step="10"
                    value={props.force_settings.author_repulsion_min_distance.to_string()}
                    onchange={create_slider_callback("author_repulsion_min_distance")}
                    style="width: 200px;"
                />
            </div>
            
            <div style="margin-bottom: 15px;">
                <label>{"中心力の強さ: "}{props.force_settings.center_strength as i32}</label><br/>
                <input
                    type="range"
                    min="0"
                    max="10000"
                    step="1"
                    value={props.force_settings.center_strength.to_string()}
                    onchange={create_slider_callback("center_strength")}
                    style="width: 200px;"
                />
            </div>
            
            <div style="margin-bottom: 15px;">
                <label>{"中心減衰: "}{props.force_settings.center_damping as i32}</label><br/>
                <input
                    type="range"
                    min="0"
                    max="50"
                    step="1"
                    value={props.force_settings.center_damping.to_string()}
                    onchange={create_slider_callback("center_damping")}
                    style="width: 200px;"
                />
            </div>
            
            <div style="margin-bottom: 15px;">
                <label>{"リンク力の強さ: "}{props.force_settings.link_strength as i32}</label><br/>
                <input
                    type="range"
                    min="0"
                    max="50000"
                    step="100"
                    value={props.force_settings.link_strength.to_string()}
                    onchange={create_slider_callback("link_strength")}
                    style="width: 200px;"
                />
            </div>
        </div>
    }
}