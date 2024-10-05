use yew::{classes, function_component, html, Callback, Classes, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct IconWrapperProps {
    pub name: String,
    pub children: Html,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn IconWrapper(props: &IconWrapperProps) -> Html {
    html! {
        <i class={classes!(format!("icon icon-{}", props.name), &props.class_name)}>
            { props.children.clone() }
        </i>
    }
}
