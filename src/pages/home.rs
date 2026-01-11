use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::style;

#[function_component(Home)]
pub fn home() -> Html {
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

        .content {
          background: white;
          padding: 60px 40px;
          border-radius: 16px;
          box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
          max-width: 600px;
          width: 100%;
        }

        img {
          max-width: 200px;
          height: auto;
          margin-bottom: 32px;
          animation: float 3s ease-in-out infinite;
        }

        @keyframes float {
          0%, 100% { transform: translateY(0px); }
          50% { transform: translateY(-20px); }
        }

        h1 {
          font-size: 32px;
          font-weight: 600;
          color: #000000d9;
          margin-bottom: 16px;
        }

        .subtitle {
          font-size: 16px;
          color: #00000073;
          margin-bottom: 40px;
        }

        .nav-links {
          display: flex;
          gap: 16px;
          justify-content: center;
          flex-wrap: wrap;
        }

        .nav-links a {
          padding: 12px 32px;
          background: #1890ff;
          color: white;
          border-radius: 8px;
          font-weight: 500;
          transition: all 0.3s;
          box-shadow: 0 2px 8px rgba(24, 144, 255, 0.3);
        }

        .nav-links a:hover {
          background: #40a9ff;
          transform: translateY(-2px);
          box-shadow: 0 4px 12px rgba(24, 144, 255, 0.4);
        }

        @media (max-width: 768px) {
          .content {
            padding: 40px 24px;
          }

          h1 {
            font-size: 24px;
          }

          .nav-links {
            flex-direction: column;
          }

          .nav-links a {
            width: 100%;
          }
        }
    "#
    )
    .expect("Failed to mount style!");

    html! {
      <div class={style.get_class_name().to_string()}>
        <div class="content">
          <img src="./assets/logo.png" alt="Yew Logo" />
          <h1>{"Welcome to Yew App"}</h1>
          <p class="subtitle">{"A modern web application built with Rust and Yew"}</p>
          <div class="nav-links">
            <Link<Route> to={Route::TodoList}>{"Todo List"}</Link<Route>>
            <Link<Route> to={Route::Videos}>{"Videos"}</Link<Route>>
          </div>
        </div>
      </div>
    }
}
