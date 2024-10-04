use crate::components::atoms::button::Button;
use crate::components::atoms::input::Input;
use stylist::yew::use_style;
use yew::{function_component, html, Html};

#[function_component]
pub fn Login() -> Html {
    let style = use_style!(
        r#"
        text-align: center;
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding-top: 40px;

        .form {
            width: 300px
        }

        .form-item {
            margin-bottom: 20px;
        }

        button {
            width:100%;
        }
    "#
    );

    html! {
      <div class={style}>
          <img src="/assets/logo.png"  />
          <div class="login">
            <form class="form">
                <div class="form-item"><Input name="name" placeholder="name"/></div>
                <div class="form-item"><Input name="password" input_type="password" placeholder="password"/></div>
                <div><Button button_type="submit">{"Login"}</Button></div>
            </form>
          </div>
      </div>
    }
}
