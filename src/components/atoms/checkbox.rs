use stylist::style;
use yew::{function_component, html, Html};

#[function_component]

pub fn CheckBox() -> Html {
    let style = style!("color: red;").expect("Failed to create style");

    html! {<checkbox class={style.get_class_name().to_string()}>{"Hello World!"}</checkbox>}
}
