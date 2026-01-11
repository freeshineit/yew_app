use crate::Route;
use stylist::style;
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
    let style = style!(
        r#"
          display: flex;
          flex-direction: column;
          width: 100%;
          min-height: 100vh;
          background-color: #fafafa;
          padding-top: 64px;
          flex: 1 1 auto;
        "#
    ).expect("Failed to create style");

    html! {<div class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>{ props.children.clone() }</div>}
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
    let style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 64px;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background-color: #fff;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
        padding: 0 32px;
        z-index: 1000;
        
        ul {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        li {
            display: inline-flex;
        }

        li > a {
            height: 40px;
            line-height: 40px;
            padding: 0 16px;
            border-radius: 4px;
            transition: all 0.2s;
            font-weight: 500;
        }

        li > a:hover {
            background-color: #f0f0f0;
        }

        .avatar-link {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        img {
            width: 36px;
            height: 36px;
            border-radius: 50%;
            transition: transform 0.2s;
        }

        img:hover {
            transform: scale(1.1);
        }

        @media (max-width: 768px) {
            padding: 0 16px;
            
            ul {
                gap: 4px;
            }

            li > a {
                padding: 0 12px;
                font-size: 13px;
            }
        }
      "#
    ).expect("Failed to create style");

    html! {
        <div class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>></li>
                <li><Link<Route> to={Route::Videos}>{"Videos"}</Link<Route>></li>
            </ul>
            <div class="avatar-link">
                <a href="https://github.com/freeshineit" target="_blank">
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
    let style = style!(
        r#"
        display: flex;
        height: 64px;
        background-color: #fff;
        box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.08);
        align-items: center;
        justify-content: center;
        gap: 24px;
        padding: 0 32px;

        span, a {
            line-height: 22px;
            height: 22px;
            font-size: 14px;
            color: #00000073;
        }

        a {
            transition: color 0.2s;
        }

        a:hover {
            color: #1890ff;
        }

        @media (max-width: 768px) {
            padding: 0 16px;
            gap: 12px;
            font-size: 13px;
        }
      "#
    ).expect("Failed to create style");

    html! {
        <footer class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>
            <span>{"Built with ❤️ by"}</span>
            <a href="https://github.com/freeshineit" target="_blank">{"ShineShao"}</a>
            <span>{"|"}</span>
            <a href="https://github.com/freeshineit/yew_app/issues" target="_blank">{"Report Issues"}</a>
        </footer>
    }
}
