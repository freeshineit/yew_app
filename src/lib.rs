mod components;
mod pages;
mod state;
mod utils;

use crate::components::atoms::layout::{Layout, LayoutFooter, LayoutHeader};
use crate::state::AppStateProvider;
use pages::home::Home;
use pages::login::Login;
use pages::not_found::NotFound;
use pages::todo_list::TodoList;
use pages::videos::Videos;
use yew::prelude::*;
use yew_router::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// root route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/todo_list")]
    TodoList,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    html! {
     <Layout>
       <LayoutHeader />
       <div style="flex: 1 1 auto;">
          {
            match routes {
              Route::Home => html! {
                <Home />
              },
              Route::Login => html! {
                <Login />
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

#[function_component]
pub fn App() -> Html {
    html! {
        <AppStateProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </AppStateProvider>
    }
}
