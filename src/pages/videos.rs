use crate::components::atoms::error::ErrorMessage;
use crate::components::atoms::loading::Loading;
use reqwasm::http::Request;
use serde::Deserialize;
use stylist::yew::use_style;
use yew::prelude::*;

#[derive(Debug, PartialEq, Deserialize, Properties)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(Videos)]
pub fn videos() -> Html {
    let style = use_style!(
        r#"
        text-align: center;
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 40px 20px;

        h1 {
            font-size: 28px;
            font-weight: 600;
            color: #000000d9;
            margin-bottom: 32px;
        }

        ul {
            width: 100%;
            max-width: 800px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
            padding: 24px;
        }

        li {
            height: auto;
            min-height: 48px;
            line-height: 1.5;
            padding: 16px;
            border-bottom: 1px solid #f0f0f0;
            transition: background-color 0.2s;
        }

        li:last-child {
            border-bottom: none;
        }

        li:hover {
            background-color: #fafafa;
        }

        a {
            display: block;
            text-align: left;
            font-size: 14px;
        }

        @media (max-width: 768px) {
            padding: 20px 16px;

            h1 {
                font-size: 24px;
            }

            ul {
                padding: 16px;
            }

            li {
                padding: 12px;
            }
        }
    "#
    );

    let videos = use_state(|| vec![]);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    {
        let videos = videos.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/assets/videos.json").send().await {
                    Ok(response) => match response.json::<Vec<Video>>().await {
                        Ok(fetched_videos) => {
                            videos.set(fetched_videos);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse videos: {:?}", e)));
                            loading.set(false);
                        }
                    },
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch videos: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
      <div class={style}>
        <h1>{"Video Collection"}</h1>
        {
            if *loading {
                html! { <Loading text="Loading videos..." /> }
            } else if let Some(err_msg) = (*error).clone() {
                html! { <ErrorMessage message={err_msg} /> }
            } else if videos.is_empty() {
                html! {
                    <div class="empty-state">
                        {"No videos available"}
                    </div>
                }
            } else {
                html! {
                    <ul>
                    {
                        videos.iter().enumerate().map(|(idx, video)| {
                          html! {
                            <li key={idx}>
                                <a href={video.url.clone()} target="_blank">
                                    <strong>{&*video.title}</strong>
                                    {" - Speaker: "}
                                    {&*video.speaker}
                                </a>
                            </li>
                          }
                        }).collect::<Html>()
                      }
                    </ul>
                }
            }
        }
      </div>
    }
}
