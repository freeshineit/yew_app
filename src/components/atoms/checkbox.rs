use crate::components::icons::check::CheckIcon;
use stylist::style;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct CheckBoxProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn CheckBox(props: &CheckBoxProps) -> Html {
    let style = style!(
        r#"
            display: inline-flex;
            width: 32px;
            height: 32px;
            cursor: pointer;
            padding: var(--spacing-xs);
            justify-content: center;
            align-items: center;
            font-size: 20px;
            border-radius: var(--radius-md);
            transition: all 0.2s;
            color: var(--color-text-secondary);

            &:hover {
                background-color: var(--color-bg-hover);
            }

            &.checked {
                color: var(--color-primary);
            }
        "#
    )
    .expect("Failed to create style");

    let class_name = if props.checked {
        format!("{} checked", style.get_class_name())
    } else {
        style.get_class_name().to_string()
    };

    let combined_class = if let Some(extra_class) = &props.class {
        format!("{} {}", class_name, extra_class)
    } else {
        class_name
    };

    html! {
        <span class={combined_class} onclick={props.onclick.clone()}>
            <CheckIcon checked={props.checked}/>
        </span>
    }
}
