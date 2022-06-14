use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <div>
      <h1>{"Home"}</h1>
      <Link<Route> to={Route::TodoList}>{"todo list"}</Link<Route>>
    </div>
  }
}
