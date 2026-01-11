use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LoadingProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or("Loading...".to_string())]
    pub text: String,
}

#[function_component]
pub fn Loading(props: &LoadingProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 40px;
        
        .spinner {
            width: 40px;
            height: 40px;
            border: 4px solid #f3f3f3;
            border-top: 4px solid #1890ff;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }
        
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        
        .text {
            margin-top: 16px;
            color: #00000073;
            font-size: 14px;
        }
        "#
    );

    html! {
        <div class={classes!(style, props.class.clone())}>
            <div class="spinner"></div>
            <div class="text">{&props.text}</div>
        </div>
    }
}
