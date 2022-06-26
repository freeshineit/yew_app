use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

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
        height: 60px;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background-color: #FFF;
        box-shadow: 0 2px 8px #f0f1f2;
      "#
    )
    .expect("Failed to mount style!");

    html! {<div class={classes!(style, props.class.to_owned())}>{"Header"}</div>}
}
