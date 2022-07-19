use stylist::yew::styled_component;
use yew::prelude::*;

use stylist::style;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub class: Option<String>,
    pub placeholder: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    pub name: Option<String>,
    pub input_type: Option<String>,
}

#[styled_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let style = style!(
        r#"
            box-sizing: border-box;
            margin: 0;
            list-style: none;
            position: relative;
            display: inline-block;
            width: 100%;
            min-width: 0;
            padding: 4px 11px;
            color: #000000d9;
            font-size: 14px;
            line-height: 1.5715;
            background-color: #fff;
            background-image: none;
            border: 1px solid #d9d9d9;
            border-radius: 2px;
            transition: all .3s;

            &:hover {
                border-color: #40a9ff;
                border-right-width: 1px;
            }
            &:focus {
                border-color: #40a9ff;
                box-shadow: 0 0 0 2px rgba(24, 144, 255, .2);
                border-right-width: 1px;
                outline: 0;
            }
        "#
    )
    .expect("Failed to mount style!");

    html! {
        <input
            class={classes!(style, props.class.to_owned())}
            placeholder={props.placeholder.to_owned()}
            name={props.name.to_owned()}
            type={props.input_type.to_owned()}
            oninput={props.oninput.clone()}
            onkeydown={props.onkeydown.clone()}
        />
    }
}
