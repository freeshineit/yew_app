use stylist::yew::use_style;
use yew::{
    classes, function_component, html, Callback, Html, InputEvent, KeyboardEvent, NodeRef,
    Properties,
};

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or_default]
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub input_type: Option<String>,
    #[prop_or_default]
    pub input_ref: NodeRef,
}

#[function_component]

pub fn Input(props: &InputProps) -> Html {
    let style = use_style!(
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
    );

    html! {
        <input
            class={classes!(style, props.class.to_owned())}
            ref={&props.input_ref}
            placeholder={props.placeholder.to_owned()}
            name={props.name.to_owned()}
            type={props.input_type.to_owned()}
            oninput={props.oninput.clone()}
            onkeydown={props.onkeydown.clone()}
        />
    }
}
