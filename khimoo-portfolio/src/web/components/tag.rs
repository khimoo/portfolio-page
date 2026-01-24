use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagPillProps {
    pub label: String,
    #[prop_or(false)]
    pub selected: bool,
    #[prop_or_default]
    pub on_click: Option<Callback<String>>,
}

#[function_component(TagPill)]
pub fn tag_pill(props: &TagPillProps) -> Html {
    let class_name = if props.selected {
        "tag-option selected"
    } else {
        "tag-option"
    };

    if let Some(on_click) = props.on_click.as_ref() {
        let tag_label = props.label.clone();
        let on_click = on_click.clone();
        let onclick = Callback::from(move |_| on_click.emit(tag_label.clone()));

        html! {
            <button type="button" class={class_name} {onclick}>
                {&props.label}
            </button>
        }
    } else {
        html! {
            <span class={class_name}>
                {&props.label}
            </span>
        }
    }
}

#[function_component(TagStyles)]
pub fn tag_styles() -> Html {
    html! {
        <style>{TAG_STYLES}</style>
    }
}

const TAG_STYLES: &str = r#"
.tag-option {
    padding: 6px 10px;
    border-radius: 999px;
    border: 1px solid var(--border-color, #e0e0e0);
    background: transparent;
    color: var(--text-color, #333333);
    font-size: 12px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 4px;
}

.tag-option.selected {
    background: var(--link-color, #007bff);
    color: #fff;
    border-color: var(--link-color, #007bff);
}

.tag-list {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
}

.tag-list-label {
    color: inherit;
}
"#;
