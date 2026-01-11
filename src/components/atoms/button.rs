use stylist::style;
use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or(false)]
    pub danger: bool,
    #[prop_or_default]
    pub button_type: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let (bg_var, hover_var) = if props.danger {
        ("var(--color-error)", "var(--color-error-hover)")
    } else {
        ("var(--color-primary)", "var(--color-primary-hover)")
    };

    let style = style!(
        r#"
            color: #fff;
            border-color: ${bg_color};
            background: ${bg_color};
            text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);
            display: inline-block;
            font-weight: 500;
            user-select: none;
            touch-action: manipulation;
            height: 40px;
            padding: 8px 20px;
            font-size: 14px;
            border-radius: var(--radius-md);
            cursor: pointer;
            border: 1px solid transparent;
            text-align: center;
            transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);

            &:hover:not(:disabled) {
                background: ${hover_color};
                border-color: ${hover_color};
                transform: translateY(-1px);
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
            }

            &:active:not(:disabled) {
                transform: translateY(0);
            }

            &:disabled {
                opacity: 0.6;
                cursor: not-allowed;
            }
        "#,
        bg_color = bg_var,
        hover_color = hover_var
    )
    .expect("Failed to create style");

    html! {
        <button
            class={classes!(style.get_class_name().to_string(), props.class.to_owned())}
            onclick={props.onclick.clone()}
            type={props.button_type.to_owned()}
            disabled={props.disabled}>
            {props.children.clone()}
        </button>
    }
}
