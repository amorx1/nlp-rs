use dioxus::prelude::*;
use log::info;

fn main() {
    dioxus::web::launch(app);
}

async fn handle_prediction(query: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("query", query);
    let res = client.post("http://127.0.0.1:8081/predict")
            .header("Content-Type", "application/json")
            // .body(r#"{"query":"This is in English"}"#)
            .json(&map)
            .send()
            .await;

    // let out = match res {
    //     Ok(o) => "OK".to_string(),
    //     Err(o) => "FAIL".to_string(),
    // };
    res
}

fn app(cx: Scope) -> Element {
    let output = use_state(&cx, || "".to_string());
    cx.render(rsx! (
        head {
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
        }
        nav {
            div {
                class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center justify-end",
                a {
                    class: "inline-flex items-center py-1 px-3 text-base mt-4 md:mt-0",
                    href: "https://github.com/amorx1",
                    img {
                        class: "h-10 w-10",
                        src: "https://img.icons8.com/material-outlined/344/github.png",

                    }
                }
            }
        }
        section {
            class: "text-black dark:text-white body-font lg:pt-20",
            div {
                class: "container px-5 pt-32 mx-auto lg:px-4 lg:py-4",
                div {
                    class: "flex flex-col w-full mb-2 text-left md:text-center",
                    h1 {
                        class: "mb-2 text-4xl font-black tracking-tighter bg-clip-text text-transparent bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 dark:text-white dark:text-white lg:text-6xl md:text-4xl",
                        span {
                            "NLP Inferencing "
                        },
                        br {
                            class: "hidden lg:block",
                        },
                        "in Rust : : 🦀"
                    }
                }
            }
        }
        section {
            class: "text-black dark:text-white body-font lg:pt-20",
            div {
                class: "flex items-center justify-center h-screen",
                input {
                    class: "border-2 border-purple-400 rounded-md w-80 h-14",
                    placeholder: " Enter Query",
                    oninput: move |req| {
                        cx.spawn({
                            let output = output.clone();
                            let client = reqwest::Client::new();
                            async move {
                                let out = handle_prediction(req.value.clone(), &client).await;
                                match out {
                                    Ok(o) => output.set(o.text().await.unwrap()),
                                    Err(e) => output.set(e.to_string())
                                }
                            }
                        })
                    }
                },
                h1 { "{output}" }
        }
    }))
}