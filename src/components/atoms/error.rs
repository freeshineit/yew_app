use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ErrorMessageProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub message: String,
}

#[function_component]
pub fn ErrorMessage(props: &ErrorMessageProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 40px;
        
        .icon {
            font-size: 48px;
            color: #ff4d4f;
            margin-bottom: 16px;
        }
        
        .message {
            color: #ff4d4f;
            font-size: 16px;
            text-align: center;
        }
        "#
    );

    html! {
        <div class={classes!(style, props.class.clone())}>
            <div class="icon">{"⚠️"}</div>
            <div class="message">{&props.message}</div>
        </div>
    }
}
