use crate::Route;
use stylist::yew::use_style;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Html,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    let style = use_style!(
        r#"
          display: flex;
          flex-direction: column;
          width: 100%;
          min-height: 100vh;
          background-color: #FFF;
          padding-top: 60px;
          flex: 1 1 auto;
        "#
    );

    html! {<div class={classes!(style, props.class.to_owned())}>{ props.children.clone() }</div>}
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutHeaderProps {
    #[prop_or_default]
    pub class: Option<String>,
}

///
///
///
///
///
///
#[function_component]
pub fn LayoutHeader(props: &LayoutHeaderProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        justify-content:space-between;
        height: 60px;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background-color: #FFF;
        box-shadow: 0 2px 8px #f0f1f2;
        padding:10px 20px;
        z-index: 99;
        
        ul {
            display: flex;
            align-items: center;
        }

        li {
            height: 40px;
            margin-right: 15px;
            display: inline-flex;
        }

        li > a {
            height: 40px;
            line-height: 24px;
            padding: 8px 10px;
        }

        img {
            width: 34px;
            height: 34px;
            border-radius: 50%;
        }
      "#
    );

    html! {
        <div class={classes!(style, props.class.to_owned())}>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>></li>
                <li><Link<Route> to={Route::Videos}>{"Videos"}</Link<Route>></li>
            </ul>
            <div>
                <a href="https://github.com/freeshineit" target="_block">
                    <img src="https://avatars.githubusercontent.com/u/16034259?v=4" alt="ShineShao" />
                </a>
            </div>
        </div>
    }
}

///
///
///
///
#[function_component]
pub fn LayoutFooter(props: &LayoutHeaderProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        height: 60px;
        background-color: #FFF;
        box-shadow: 0 2px 8px #f0f1f2;
        align-items: center;
        justify-content: center;

        span, a {
            line-height: 22px;
            height: 22px;
        }
      "#
    );

    html! {
        <footer class={classes!(style, props.class.to_owned())}>
            <span style="margin-right:20px">{"I'm here"} </span>
            <a href="https://github.com/freeshineit" target="_block">{"ShineShao"}</a>
            <a href="https://github.com/freeshineit/yew_app/issues" target="_blank" style="margin-left:20px">{"issues"}</a>
        </footer>
    }
}
