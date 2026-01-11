use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

/// 全局应用状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: ThemeMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user: None,
            theme: ThemeMode::Light,
        }
    }
}

/// 全局状态上下文
pub type AppStateContext = UseStateHandle<Rc<AppState>>;

/// 提供全局状态的 Provider 组件
#[derive(Properties, PartialEq)]
pub struct AppStateProviderProps {
    pub children: Html,
}

#[function_component]
pub fn AppStateProvider(props: &AppStateProviderProps) -> Html {
    let state = use_state(|| Rc::new(AppState::default()));

    html! {
        <ContextProvider<AppStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<AppStateContext>>
    }
}

/// Hook 用于访问全局状态
#[hook]
pub fn use_app_state() -> AppStateContext {
    let context = use_context::<AppStateContext>();
    context.expect("AppStateContext not found. Make sure to wrap your app with AppStateProvider")
}
