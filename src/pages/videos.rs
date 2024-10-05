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
        padding-top: 40px;

        a {
          margin: 0 10px;
        }

        li {
            height: 30px;
            line-height: 30px;
        }
    "#
    );

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();

        use_effect_with((), move |_| {
            // let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/assets/videos.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                videos.set(fetched_videos);
            });
        });
    }

    html! {
      <div class={style}>
        <ul>
        {
            videos.iter().enumerate().map(|(idx, video)| {
              html! {
                <li key={idx}>
                    <a href={video.url.clone()} target="_block">{&*video.title}  {"speaker:"} {&*video.speaker}</a>
                </li>
              }
            }).collect::<Html>()
          }
        </ul>
      </div>
    }
}
