use stylist::yew::use_style;
use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub danger: Option<String>,
    #[prop_or_default]
    pub button_type: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let style = use_style!(
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
    );

    html! {
        <button
            class={classes!(style, props.class.to_owned())}
            onclick={props.onclick.clone()}
            type={props.button_type.to_owned()}>
            {props.children.clone()}
        </button>
    }
}
