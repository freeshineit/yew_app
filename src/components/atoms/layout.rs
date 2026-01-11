use crate::utils::storage::LocalStorage;
use crate::utils::theme::{CustomTheme, ThemeType};
use crate::Route;
use stylist::style;
use yew::{
    classes, function_component, html, use_effect_with, use_state, Callback, Html, Properties,
};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    #[prop_or_default]
    pub class: Option<String>,
    pub children: Html,
}

#[function_component]
pub fn Layout(props: &LayoutProps) -> Html {
    let style = style!(
        r#"
          display: flex;
          flex-direction: column;
          width: 100%;
          min-height: 100vh;
          background-color: var(--color-bg-secondary);
          padding-top: 64px;
          flex: 1 1 auto;
        "#
    )
    .expect("Failed to create style");

    html! {<div class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>{ props.children.clone() }</div>}
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutHeaderProps {
    #[prop_or_default]
    pub class: Option<String>,
}

///
///
///
///
///
///
#[function_component]
pub fn LayoutHeader(props: &LayoutHeaderProps) -> Html {
    let style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 64px;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        background-color: var(--color-bg-primary);
        box-shadow: var(--shadow-sm);
        padding: 0 var(--spacing-xl);
        z-index: 1000;
        
        ul {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
        }

        li {
            display: inline-flex;
        }

        li > a {
            height: 40px;
            line-height: 40px;
            padding: 0 var(--spacing-md);
            border-radius: var(--radius-md);
            transition: all 0.2s;
            font-weight: 500;
        }

        li > a:hover {
            background-color: var(--color-bg-hover);
        }

        .right-section {
            display: flex;
            align-items: center;
            gap: var(--spacing-md);
        }

        .theme-switcher {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
        }

        .theme-btn {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            border: 2px solid var(--color-border);
            cursor: pointer;
            transition: all 0.2s;
            position: relative;
        }

        .theme-btn:hover {
            transform: scale(1.1);
            border-color: var(--color-primary);
        }

        .theme-btn.active {
            border-color: var(--color-primary);
            box-shadow: 0 0 0 2px rgba(24, 144, 255, 0.2);
        }

        .theme-btn.active::after {
            content: "✓";
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: #fff;
            font-size: 14px;
            font-weight: bold;
        }

        .theme-default { background: linear-gradient(135deg, #1890ff 0%, #40a9ff 100%); }
        .theme-dark { background: linear-gradient(135deg, #141414 0%, #434343 100%); }
        .theme-purple { background: linear-gradient(135deg, #722ed1 0%, #9254de 100%); }
        .theme-green { background: linear-gradient(135deg, #52c41a 0%, #73d13d 100%); }
        .theme-orange { background: linear-gradient(135deg, #fa8c16 0%, #ffa940 100%); }

        .avatar-link {
            display: flex;
            align-items: center;
            gap: var(--spacing-sm);
        }

        img {
            width: 36px;
            height: 36px;
            border-radius: 50%;
            transition: transform 0.2s;
        }

        img:hover {
            transform: scale(1.1);
        }

        @media (max-width: 768px) {
            padding: 0 var(--spacing-md);
            
            ul {
                gap: var(--spacing-xs);
            }

            li > a {
                padding: 0 var(--spacing-sm);
                font-size: 13px;
            }

            .theme-switcher {
                gap: var(--spacing-xs);
            }

            .theme-btn {
                width: 28px;
                height: 28px;
            }
        }
      "#
    )
    .expect("Failed to create style");

    // 从 localStorage 加载主题，默认为 Default
    let current_theme = use_state(|| {
        if let Some(stored) = LocalStorage::get("theme") {
            if let Ok(theme) = serde_json::from_str::<ThemeType>(&stored) {
                return theme;
            }
        }
        ThemeType::Default
    });

    // 应用当前主题
    {
        let theme = *current_theme;
        use_effect_with(theme, move |theme| {
            let custom_theme = theme.to_theme();
            custom_theme.apply_to_css();
            || ()
        });
    }

    let switch_theme = {
        let current_theme = current_theme.clone();
        move |theme_type: ThemeType| {
            let current_theme = current_theme.clone();
            Callback::from(move |_| {
                current_theme.set(theme_type);
                // 保存到 localStorage
                if let Ok(json) = serde_json::to_string(&theme_type) {
                    let _ = LocalStorage::set("theme", &json);
                }
            })
        }
    };

    html! {
        <div class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>></li>
                <li><Link<Route> to={Route::Videos}>{"Videos"}</Link<Route>></li>
                <li><Link<Route> to={Route::Login}>{"Login"}</Link<Route>></li>
            </ul>
            <div class="right-section">
                <div class="theme-switcher">
                    {
                        [ThemeType::Default, ThemeType::Dark, ThemeType::Purple, ThemeType::Green, ThemeType::Orange]
                            .iter()
                            .map(|theme| {
                                let is_active = *current_theme == *theme;
                                let class_name = format!(
                                    "theme-btn theme-{} {}",
                                    theme.name().to_lowercase(),
                                    if is_active { "active" } else { "" }
                                );
                                html! {
                                    <div
                                        key={theme.name()}
                                        class={class_name}
                                        onclick={switch_theme(*theme)}
                                        title={theme.name()}
                                    />
                                }
                            }).collect::<Html>()
                    }
                </div>
                <div class="avatar-link">
                    <a href="https://github.com/freeshineit" target="_blank">
                        <img src="https://avatars.githubusercontent.com/u/16034259?v=4" alt="ShineShao" />
                    </a>
                </div>
            </div>
        </div>
    }
}

///
///
///
///
#[function_component]
pub fn LayoutFooter(props: &LayoutHeaderProps) -> Html {
    let style = style!(
        r#"
        display: flex;
        height: 64px;
        background-color: var(--color-bg-primary);
        box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.08);
        align-items: center;
        justify-content: center;
        gap: var(--spacing-lg);
        padding: 0 var(--spacing-xl);

        span, a {
            line-height: 22px;
            height: 22px;
            font-size: 14px;
            color: var(--color-text-secondary);
        }

        a {
            transition: color 0.2s;
        }

        a:hover {
            color: var(--color-primary);
        }

        @media (max-width: 768px) {
            padding: 0 var(--spacing-md);
            gap: var(--spacing-sm);
            font-size: 13px;
        }
      "#
    )
    .expect("Failed to create style");

    html! {
        <footer class={classes!(style.get_class_name().to_string(), props.class.to_owned())}>
            <span>{"Built with ❤️ by"}</span>
            <a href="https://github.com/freeshineit" target="_blank">{"ShineShao"}</a>
            <span>{"|"}</span>
            <a href="https://github.com/freeshineit/yew_app/issues" target="_blank">{"Report Issues"}</a>
        </footer>
    }
}
