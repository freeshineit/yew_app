mod components;
mod pages;
mod utils;

use pages::home::Home;
use pages::not_found::NotFound;
use pages::todo_list::TodoList;
use pages::videos::Videos;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atoms::layout::{Layout, LayoutFooter, LayoutHeader};

// root route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/todo_list")]
    TodoList,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    html! {
     <Layout>
       <LayoutHeader />
       <div style="flex: 1 1 auto;">
          {
            match routes {
              Route::Home => html! {
                <Home />
              },
              Route::TodoList => html! {
                <TodoList />
              },
              Route::Videos => html! {
                <Videos />
              },
              Route::NotFound => html! {<NotFound /> },
            }
          }
       </div>
       <LayoutFooter />
     </Layout>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
