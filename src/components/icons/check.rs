use super::icon_wrapper::IconWrapper;
use yew::{function_component, html, Classes, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct CheckIconProps {
    pub checked: bool,
    #[prop_or_default]
    pub class_name: Option<Classes>,
}

#[function_component]
pub fn CheckIcon(props: &CheckIconProps) -> Html {
    html! {
        <IconWrapper class_name={props.class_name.to_owned()} name={"check"}>
            <svg viewBox="0 0 1024 1024" version="1.1"  width="1em" height="1em" fill="currentColor" focusable="false">
                {
                    if props.checked {
                        html! {
                            <path d="M433.1 657.7c12.7 17.7 39 17.7 51.7 0l210.6-292c3.8-5.3 0-12.7-6.5-12.7H642c-10.2 0-19.9 4.9-25.9 13.3L459 584.3l-71.2-98.8c-6-8.3-15.6-13.3-25.9-13.3H315c-6.5 0-10.3 7.4-6.5 12.7l124.6 172.8z" />
                        }
                    } else {
                        html!{}
                    }
                }
                <path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32z m-40 728H184V184h656v656z" />
            </svg>
        </IconWrapper>
    }
}
