use crate::components::atoms::button::Button;
use crate::components::atoms::input::Input;
use crate::Route;
use stylist::style;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Login() -> Html {
    let style = style!(
        r#"
        text-align: center;
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 40px 20px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

        .login-container {
            background: white;
            padding: 48px 40px;
            border-radius: 12px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
            width: 100%;
            max-width: 400px;
        }

        img {
            max-width: 120px;
            height: auto;
            margin-bottom: 32px;
        }

        h1 {
            font-size: 24px;
            font-weight: 600;
            color: #000000d9;
            margin-bottom: 32px;
        }

        .form-item {
            margin-bottom: 20px;
            text-align: left;
        }

        .form-item label {
            display: block;
            margin-bottom: 8px;
            color: #000000d9;
            font-weight: 500;
        }

        button {
            width: 100%;
            height: 44px;
        }

        .footer {
            margin-top: 24px;
            text-align: center;
            color: #00000073;
            font-size: 14px;
        }

        .footer a {
            color: #1890ff;
            margin-left: 8px;
        }

        @media (max-width: 768px) {
            .login-container {
                padding: 32px 24px;
            }
        }
    "#
    ).expect("Failed to create style");

    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let handle_submit = {
        let username_ref = username_ref.clone();
        let password_ref = password_ref.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if let (Some(username_input), Some(password_input)) = (
                username_ref.cast::<HtmlInputElement>(),
                password_ref.cast::<HtmlInputElement>(),
            ) {
                let username = username_input.value();
                let password = password_input.value();
                
                // 简单的演示逻辑
                if !username.is_empty() && !password.is_empty() {
                    // 这里可以添加实际的登录逻辑
                    navigator.push(&Route::Home);
                }
            }
        })
    };

    html! {
        <div class={style.get_class_name().to_string()}>
            <div class="login-container">
                <img src="./assets/logo.png" alt="Logo" />
                <h1>{"Welcome Back"}</h1>
                <form onsubmit={handle_submit}>
                    <div class="form-item">
                        <label>{"Username"}</label>
                        <Input 
                            name="username" 
                            placeholder="Enter your username"
                            input_ref={username_ref}
                        />
                    </div>
                    <div class="form-item">
                        <label>{"Password"}</label>
                        <Input 
                            name="password" 
                            input_type="password" 
                            placeholder="Enter your password"
                            input_ref={password_ref}
                        />
                    </div>
                    <div>
                        <Button button_type="submit">{"Sign In"}</Button>
                    </div>
                </form>
                <div class="footer">
                    {"Don't have an account?"}
                    <a href="#">{"Sign up"}</a>
                </div>
            </div>
        </div>
    }
}
