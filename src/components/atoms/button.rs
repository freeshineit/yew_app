use stylist::yew::styled_component;
use yew::prelude::*;

use stylist::style;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub class: Option<String>,
    pub children: Children,
    pub danger: Option<String>,
    pub button_type: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = style!(
        r#"
            color: #fff;
            border-color: #1890ff;
            background: #1890ff;
            text-shadow: 0 -1px 0 rgb(0 0 0 / 12%);
            box-shadow: 0 2px #0000000b;
            display: inline-block;
            font-weight: 400;
            user-select: none;
            touch-action: manipulation;
            height: 32px;
            padding: 4px 15px;
            font-size: 14px;
            border-radius: 2px;
            cursor: pointer;
            border: 1px solid transparent;
            text-align: center;
        "#
    )
    .expect("Failed to mount style!");

    html! {
        <button
            class={classes!(style, props.class.to_owned())}
            onclick={props.onclick.clone()}
            type={props.button_type.to_owned()}>
            {props.children.clone()}
        </button>
    }
}
