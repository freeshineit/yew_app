use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::style;

#[function_component(Home)]
pub fn home() -> Html {
    let style = style!(
        r#"
        text-align: center;
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding-top: 40px;

        a {
          margin: 0 10px;
        }

        .list {
          margin-top: 50px;
        }
    "#
    )
    .expect("Failed to mount style!");

    html! {
      <div class={style}>
          <img src="/assets/logo.png"  />
          <div class="list">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>>
          </div>
      </div>
    }
}
