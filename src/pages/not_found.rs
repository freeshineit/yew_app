use crate::Route;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let style = use_style!(
        r#"
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 40px 20px;
        text-align: center;

        .error-code {
            font-size: 120px;
            font-weight: 700;
            color: #1890ff;
            line-height: 1;
            margin-bottom: 24px;
        }

        h1 {
            font-size: 32px;
            font-weight: 600;
            color: #000000d9;
            margin-bottom: 16px;
        }

        p {
            font-size: 16px;
            color: #00000073;
            margin-bottom: 32px;
        }

        a {
            padding: 12px 32px;
            background: #1890ff;
            color: white;
            border-radius: 8px;
            font-weight: 500;
            transition: all 0.3s;
            display: inline-block;
        }

        a:hover {
            background: #40a9ff;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(24, 144, 255, 0.4);
        }

        @media (max-width: 768px) {
            .error-code {
                font-size: 80px;
            }

            h1 {
                font-size: 24px;
            }
        }
        "#
    );

    html! {
        <div class={style}>
            <div class="error-code">{"404"}</div>
            <h1>{"Page Not Found"}</h1>
            <p>{"Sorry, the page you are looking for does not exist."}</p>
            <Link<Route> to={Route::Home}>{"Back to Home"}</Link<Route>>
        </div>
    }
}
