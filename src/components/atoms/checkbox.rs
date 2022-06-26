use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(CheckBox)]
pub fn checkbox() -> Html {
    html! {<checkbox class={css!("color: red;")}>{"Hello World!"}</checkbox>}
}
