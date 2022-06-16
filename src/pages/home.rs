use crate::components::atoms::layout::{Layout, LayoutHeader};
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::style;

#[function_component(Home)]
pub fn home() -> Html {
    // let style = style!(
    //     r#"
    //     background-color: red;

    //     .nested {
    //         background-color: blue;
    //         width: 100px
    //     }
    // "#
    // )
    // .expect("Failed to mount style!");

    html! {
      <Layout class={String::from("adfadsfa")}>
        <LayoutHeader />
          <div>
          <Link<Route> to={Route::TodoList}>{"todo list"}</Link<Route>>
          // <Button />
          </div>
          <div>
            <h1>{"Home"}</h1>
            <img src="/logo.png" />
          </div>
      </Layout>
    }
}
