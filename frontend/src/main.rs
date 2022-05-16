use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn Nav(cx: Scope) -> Element {
    cx.render(rsx! (
        div  {
            class: "bg-black",
            div {
                class: "px-4 py-8 mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8",
                div {
                    class: "relative flex items-center justify-between",
                    a {
                        class: "inline-flex items-center text-blue-700 font-bold text-6xl",
                        "NLP Ops"
                    }
                    ul {
                        class: "flex items-center hidden space-x-8 lg:flex",
                        li {
                            button {
                                class: "font-medium tracking-wide text-gray-100 transition-colors duration-200 hover:text-teal-accent-400",
                                "Translation"
                            }
                        },
                        li {
                            button {
                                class: "font-medium tracking-wide text-gray-100 transition-colors duration-200 hover:text-teal-accent-400",
                                "Summarization"
                            }
                        },
                        li {
                            button {
                                class: "font-medium tracking-wide text-gray-100 transition-colors duration-200 hover:text-teal-accent-400",
                                "Dialogue"
                            }
                        },
                        li {
                            button {
                                class: "font-medium tracking-wide text-gray-100 transition-colors duration-200 hover:text-teal-accent-400",
                                "Generation"
                            }
                        }
                    }
                    div {
                        class: "",
                        button {
                            class: ""
                        }
                    }
                }
            }
        }
    ))
}

async fn handle_prediction(query: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("query", query);
    client.post("http://127.0.0.1:8081/predict")
            .header("Content-Type", "application/json")
            // .body(r#"{"query":"This is in English"}"#)
            .json(&map)
            .send()
            .await
}

fn app(cx: Scope) -> Element {
    let output = use_state(&cx, || "".to_string());
    cx.render(rsx! (
        head {
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.css" }
        }
        nav {
            // div {
            //     class: "flex flex row items center justify-start bg-black",
            //     h1 {
            //         class: "text-white font-bold text-6xl mt-24 mx-24",
            //         "Natural Language Processing (NLP) MLOps"
            //     }
            // }
            Nav {}
        }
        body {
        //     section {
        //         class: "flex flex row items-center justify-end bg-black",
        //         button {
        //             class: "bg-gray-200 hover:bg-gray-400 text-gray-800 text-4xl font-bold py-6 px-4 mx-4 mt-24 mb-24 w-1/6 rounded-lg",
        //             "Translation"
        //         }
        //         button {
        //             class: "bg-gray-200 hover:bg-gray-400 text-gray-800 text-4xl font-bold py-6 px-4 mx-4 mt-24 mb-24 w-1/6 rounded-lg",
        //             "Summarization"
        //         }
        //         button {
        //             class: "bg-gray-200 hover:bg-gray-400 text-gray-800 text-4xl font-bold py-6 px-4 mx-4 mt-24 mb-24 w-1/6 rounded-lg",
        //             "Dialogue"
        //         }
        //         button {
        //             class: "bg-gray-200 hover:bg-gray-400 text-gray-800 text-4xl font-bold py-6 px-4 mx-4 mt-24 mb-24 w-1/6 rounded-lg",
        //             "Translation"
        //         }
        //         button {
        //             class: "bg-gray-200 hover:bg-gray-400 text-gray-800 text-4xl font-bold py-6 px-4 mx-4 mt-24 mb-24 w-1/6 rounded-lg",
        //             "Translation"
        //         }
        //         a {
        //             class: "inline-flex items-center py-1 px-3 text-base mt-4 md:mt-0",
        //             href: "https://github.com/amorx1",
        //             img {
        //                 class: "h-16 w-16",
        //                 src: "https://img.icons8.com/material-outlined/344/github.png",

        //             }
        //         }
        //     }
        //     section {
        //         class: "text-black dark:text-white body-font lg:pt-20",
        //         div {
        //             class: "flex items-center justify-center h-screen",
        //             input {
        //                 class: "border-2 border-purple-400 rounded-md w-80 h-14",
        //                 placeholder: " Enter Query",
        //                 oninput: move |req| {
        //                     cx.spawn({
        //                         let output = output.clone();
        //                         let client = reqwest::Client::new();
        //                         async move {
        //                             let out = handle_prediction(req.value.clone(), &client).await;
        //                             match out {
        //                                 Ok(o) => output.set(o.text().await.unwrap()),
        //                                 Err(e) => output.set(e.to_string())
        //                             }
        //                         }
        //                     })
        //                 }
        //             },
        //             h1 { "{output}" }
        //     }
        // }
        }
))
}