use crate::utils::theme::Theme;
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
    #[prop_or(false)]
    pub disabled: bool,
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
            padding: 8px 12px;
            color: ${text_color};
            font-size: 14px;
            line-height: 1.5715;
            background-color: #fff;
            background-image: none;
            border: 1px solid ${border_color};
            border-radius: 4px;
            transition: all 0.3s;

            &:hover:not(:disabled) {
                border-color: ${primary_hover};
            }

            &:focus {
                border-color: ${primary};
                box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
                outline: 0;
            }

            &:disabled {
                background-color: ${bg_disabled};
                cursor: not-allowed;
                opacity: 0.6;
            }
        "#,
        text_color = Theme::TEXT_PRIMARY,
        border_color = Theme::BORDER,
        primary = Theme::PRIMARY,
        primary_hover = Theme::PRIMARY_HOVER,
        bg_disabled = Theme::BG_SECONDARY
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
            disabled={props.disabled}
        />
    }
}
