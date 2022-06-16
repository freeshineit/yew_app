use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Button)]
pub fn button() -> Html {
    html! {<button class={css!("color: red;")}>{"Hello World!"}</button>}
}
