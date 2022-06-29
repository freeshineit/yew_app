use crate::Route;
use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    pub class: Option<String>,
    pub children: Children,
}

#[styled_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let style = style!(
        r#"
          display: flex;
          flex-direction: column;
          width: 100%;
          min-height: 100vh;
          background-color: #FFF;
          padding-top: 60px;
          flex: 1 1 auto;
        "#
    )
    .expect("Failed to mount style!");

    html! {<div class={classes!(style, props.class.to_owned())}>{ props.children.clone() }</div>}
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutHeaderProps {
    pub class: Option<String>,
}

///
///
///
///
///
///
#[styled_component(LayoutHeader)]
pub fn layout_header(props: &LayoutHeaderProps) -> Html {
    let style = style!(
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
    )
    .expect("Failed to mount style!");

    html! {
        <div class={classes!(style, props.class.to_owned())}>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>></li>
            </ul>
            <div>
                <a href="https://github.com/freeshineit" target="_block">
                    <img src="https://freeshineit.github.io/static/images/ShineShao.jpg" alt="ShineShao" />
                </a>
            </div>
        </div>
    }
}

///
///
///
///
#[styled_component(LayoutFooter)]
pub fn layout_footer(props: &LayoutHeaderProps) -> Html {
    let style = style!(
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
    )
    .expect("Failed to mount style!");

    html! {
        <footer class={classes!(style, props.class.to_owned())}>
            <span style="margin-right:20px">{"I'm here"} </span>
            <a href="https://github.com/freeshineit" target="_block">{"ShineShao"}</a>
            <a href="https://github.com/freeshineit/yew_app/issues" target="_blank" style="margin-left:20px">{"issues"}</a>
        </footer>
    }
}
