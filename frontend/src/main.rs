use dioxus::prelude::*;

enum Services{
    Splash,
    Translate
}

static SERVICE: Atom<Services> = |_| Services::Translate;

fn main() {
    dioxus::web::launch(app);
}

pub fn Head(cs: Scope) -> Element {
    cs.render(rsx!(
        head {
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.css" }
        }
    ))
}

pub fn Nav(cx: Scope) -> Element {
    let set_service = use_set(&cx, SERVICE);
    cx.render(rsx! (
        nav {
            div {
                class: "bg-black",
                div {
                    class: "px-4 py-8 mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8",
                    div {
                        class: "relative flex items-center justify-between",
                        a {
                            class: "inline-flex items-center bg-clip-text text-transparent bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 font-bold text-8xl",
                            "NLP Ops"
                        }
                        ul {
                            class: "flex items-center hidden space-x-8 lg:flex",
                            li {
                                button {
                                    onclick: move |_| set_service(Services::Translate),
                                    class: "font-medium text-xl tracking-wide text-gray-100 transition-colors duration-200 hover:bg-gray-900 rounded-lg px-2 py-2",
                                    "Translation"
                                }
                            },
                            li {
                                button {
                                    onclick: move |_| set_service(Services::Splash),
                                    class: "font-medium text-xl tracking-wide text-gray-100 transition-colors duration-200 hover:bg-gray-900 rounded-lg px-2 py-2",
                                    "Summarization"
                                }
                            },
                            li {
                                button {
                                    class: "font-medium text-xl tracking-wide text-gray-100 transition-colors duration-200 hover:bg-gray-900 rounded-lg px-2 py-2",
                                    "Dialogue"
                                }
                            },
                            li {
                                button {
                                    class: "font-medium text-xl tracking-wide text-gray-100 transition-colors duration-200 hover:bg-gray-900 rounded-lg px-2 py-2",
                                    "Generation"
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

pub fn Translation(cx: Scope) -> Element {
    let output = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        body {
            class: "bg-black h-screen pt-48",
            div {
                class: "mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8 h-96 overflow-hidden bg-gray-900 rounded-lg shadow-md dark:bg-gray-800",
                // img {
                //     class: "object-cover w-full h-64",
                //     src: "https://images.unsplash.com/photo-1550439062-609e1531270e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=500&q=60",
                //     alt: "Article",
                // }
                div {
                    class: "p-6",
                    div {
                        // span {
                        //     class: "text-xs font-medium text-blue-600 uppercase dark:text-blue-400",
                        //     "Lorem"
                        // }
                        h1 {
                            class: "block mt-2 text-4xl font-semibold text-white transition-colors duration-200 transform dark:text-white",
                            "Translate",
                        }
                        // p {
                        //     class: "mt-2 text-sm text-white dark:text-gray-400",
                        //     "Form"
                        // }
                    }
                    div {
                        class: "mt-4",
                        div {
                            class: "flex items-center w-full",
                            div {
                                class: "flex items-center w-full",
                                input {
                                    class: "bg-black border-2 border-purple-400 rounded-md w-1/2 h-64 text-white text-2xl mx-2",
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
                                div {
                                    class: "w-1/2 h-64 border-2 border-purple-400 rounded-md mx-2",
                                    h1 {
                                        class: "text-2xl text-white",
                                        "{output}"
                                    }
                                }
                            }
                    }
                }
            }
            }
        }
    ))
}

pub fn NLP_service(cx: Scope) -> Element {
    let curr_service = use_read(&cx, SERVICE);
    match curr_service {
        Services::Translate => {
            cx.render(rsx!(
                Translation {}
            ))
        }
        Services::Splash => cx.render(rsx!(
            h1 { "BENCHOD" }
        ))
    }
}

async fn handle_prediction(query: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("query", query);
    client.post("http://127.0.0.1:8081/predict")
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
}


fn app(cx: Scope) -> Element {

    cx.render(rsx! (
        Head {}
        Nav {}
        NLP_service {}
    ))
}