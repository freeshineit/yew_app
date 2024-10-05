use stylist::yew::use_style;
use yew::{function_component, html, Html};

#[function_component]

pub fn CheckBox() -> Html {
    let style = use_style!("color: red;");

    html! {<checkbox class={style}>{"Hello World!"}</checkbox>}
}
