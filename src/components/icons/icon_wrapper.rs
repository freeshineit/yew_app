use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IconWrapperProps {
    pub name: String,
    pub children: Children,
    pub class: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(IconWrapper)]
pub fn icon_wrapper(props: &IconWrapperProps) -> Html {
    html! {
        <i onclick={props.onclick.clone()} class={classes!(format!("icon icon-{}", props.name), props.class.to_owned())}>
            {props.children.clone()}
        </i>
    }
}
