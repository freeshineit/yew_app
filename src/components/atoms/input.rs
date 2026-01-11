use stylist::style;
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
    let style = style!(
        r#"
            box-sizing: border-box;
            margin: 0;
            list-style: none;
            position: relative;
            display: inline-block;
            width: 100%;
            min-width: 0;
            padding: var(--spacing-sm) var(--spacing-md);
            color: var(--color-text-primary);
            font-size: 14px;
            line-height: 1.5715;
            background-color: var(--color-bg-primary);
            background-image: none;
            border: 1px solid var(--color-border);
            border-radius: var(--radius-md);
            transition: all 0.3s;

            &:hover:not(:disabled) {
                border-color: var(--color-primary-hover);
            }

            &:focus {
                border-color: var(--color-primary);
                box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
                outline: 0;
            }

            &:disabled {
                background-color: var(--color-bg-secondary);
                cursor: not-allowed;
                opacity: 0.6;
            }
        "#
    )
    .expect("Failed to create style");

    html! {
        <input
            class={classes!(style.get_class_name().to_string(), props.class.to_owned())}
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
