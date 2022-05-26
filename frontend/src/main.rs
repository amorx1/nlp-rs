use dioxus::prelude::*;

enum Services{
    Splash,
    Translate,
    Summarize
}

static SERVICE: Atom<Services> = |_| Services::Splash;
static LOADING: Atom<bool> = |_| false;

fn main() {
    dioxus::web::launch(app);
}

pub fn Nav(cx: Scope) -> Element {
    let set_service = use_set(&cx, SERVICE);
    cx.render(rsx! (
        nav {
            div {
                class: "bg-white dark:bg-black",
                div {
                    class: "px-4 py-16 mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8",
                    div {
                        class: "relative flex items-center justify-between",
                        button {
                            onclick: move |_| {
                                set_service(Services::Splash);
                                cx.spawn({
                                    async move {
                                        let client = reqwest::Client::new();
                                        let res = send_service("None".to_string(), &client).await;
                                        match res {
                                            _ => {}
                                        }
                                    }
                                });
                            },
                            class: "inline-flex items-center bg-clip-text text-transparent bg-gradient-to-r from-yellow-500 via-orange-500 to-red-500 font-bold text-8xl",
                            "nlp-rs"
                        }
                        ul {
                            class: "flex items-center hidden space-x-8 lg:flex",
                            li {
                                button {
                                    onclick: move |_| { 
                                        set_service(Services::Translate);
                                        cx.spawn({
                                            async move {
                                                let client = reqwest::Client::new();
                                                let res = send_service("Translation".to_string(), &client).await;
                                                match res {
                                                    _ => {}
                                                }
                                            }
                                        });
                                    },
                                    class: "font-medium text-xl tracking-wide text-gray-400 transition-colors duration-200 dark:text-white dark:hover:bg-gray-900 hover:bg-slate-600 hover:text-gray-200 rounded-lg px-2 py-2",
                                    "Translation"
                                }
                            },
                            li {
                                button {
                                    onclick: move |_| {
                                        set_service(Services::Summarize);
                                        cx.spawn({
                                            async move {
                                                let client = reqwest::Client::new();
                                                let res = send_service("Summarization".to_string(), &client).await;
                                                match res {
                                                    _ => {}
                                                }
                                            }
                                        })
                                    },
                                    class: "font-medium text-xl tracking-wide text-gray-400 transition-colors duration-200 dark:text-white dark:hover:bg-gray-900 hover:bg-slate-600 hover:text-gray-200 rounded-lg px-2 py-2",
                                    "Summarization"
                                }
                            },
                            li {
                                button {
                                    class: "font-medium text-xl tracking-wide text-gray-400 transition-colors duration-200 dark:text-white dark:hover:bg-gray-900 hover:bg-slate-600 hover:text-gray-200 rounded-lg px-2 py-2",
                                    "Sentiment Analysis"
                                }
                            },
                            li {
                                button {
                                    class: "font-medium text-xl tracking-wide text-gray-400 transition-colors duration-200 dark:text-white dark:hover:bg-gray-900 hover:bg-slate-600 hover:text-gray-200 rounded-lg px-2 py-2",
                                    "Zero-Shot"
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
    let target = use_state(&cx, || "English".to_string());
    let set_loading = use_set(&cx, LOADING);
    cx.render(rsx!(
            div {
                class: "mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8 h-full overflow-hidden bg-slate-200 rounded-lg shadow-md dark:bg-gray-900",
                div {
                    class: "p-6",
                    div {
                        class: "flex items-center w-full",
                        h1 {
                            class: "block mx-2 mt-2 text-4xl font-semibold text-slate-400 dark:text-white transition-colors duration-200 transform dark:text-white",
                            "Translate   📖",
                        },
                        Loading {}
                    }
                    div {
                        class: "mt-6",
                        div {
                            class: "flex items-center w-full",
                            div {
                                class: "flex items-center w-full",
                                textarea {
                                    class: "bg-white dark:bg-black border-2 border-yellow-500 rounded-md w-1/2 h-64 text-slate-400 dark:text-white text-2xl mx-2",
                                    placeholder: " Enter Query",
                                    oninput: move |req| {
                                        set_loading(true);
                                        cx.spawn({
                                            let output = output.clone();
                                            let target = target.clone();
                                            let client = reqwest::Client::new();
                                            let sl = set_loading.clone();
                                            async move {
                                                let out = handle_prediction(&target, req.value.clone(), &client).await;
                                                match out {
                                                    Ok(o) => {
                                                        sl(false);
                                                        output.set(o.text().await.unwrap());
                                                    },
                                                    Err(e) => output.set(e.to_string())
                                                }
                                            }
                                        })
                                    }
                                },
                                div {
                                    class: "w-1/2 h-64 border-2 bg-white dark:bg-black border-yellow-500 rounded-md mx-2",
                                    h1 {
                                        class: "text-2xl text-slate-400 dark:text-white",
                                        "{output}"
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class:"mt-6",
                        div {
                            class: "flex items-center w-full",
                            div {
                                class: "flex items-center w-full",
                                label { 
                                    class: "text-slate-400 dark:text-white font-bold text-xl mx-2",
                                    "Select target language: " }
                                select {
                                    class: "mx-2 rounded-lg w-full h-12 py-2 bg-slate-200 dark:bg-black border-2 border-yellow-500 text-slate-400 dark:text-white text-xl font-bold",
                                    onchange: move |t| target.set(t.value.to_string()),
                                    name: "Target",
                                    id: "target-selection",
                                    option {
                                        value: "English",
                                        "English"
                                    },
                                    option {
                                        value: "French",
                                        "French"
                                    },
                                    option {
                                        value: "Spanish",
                                        "Spanish"
                                    },
                                    option {
                                        value: "German",
                                        "German"
                                    }
                                },
                            }
                        },
                    }
                }
            }
    ))
}

pub fn Summarization(cx: Scope) -> Element {
    let output = use_state(&cx, || "".to_string());
    let set_loading = use_set(&cx, LOADING);
    cx.render(rsx!(
            div {
                class: "mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8 h-full overflow-hidden bg-slate-200 rounded-lg shadow-md dark:bg-gray-900",
                div {
                    class: "p-6",
                    div {
                        class: "flex items-center w-full",
                        h1 {
                            class: "block mx-2 mt-2 text-4xl font-semibold text-slate-400 dark:text-white transition-colors duration-200 transform dark:text-white",
                            "Summarize  💡",
                        },
                        Loading {}
                    }
                    div {
                        class: "mt-6",
                        div {
                            class: "flex items-center w-full",
                            div {
                                class: "flex items-center w-full",
                                textarea {
                                    class: "bg-white dark:bg-black border-2 border-yellow-500 rounded-md w-1/2 h-64 text-slate-400 dark:text-white text-2xl mx-2",
                                    placeholder: " Enter Text",
                                    oninput: move |req| {
                                        set_loading(true);
                                        cx.spawn({
                                            let output = output.clone();
                                            let sl = set_loading.clone();
                                            let client = reqwest::Client::new();
                                            async move {
                                                let out = handle_summarization(req.value.clone(), &client).await;
                                                match out {
                                                    Ok(o) => {
                                                        sl(false);
                                                        output.set(o.text().await.unwrap())
                                                    },
                                                    Err(e) => {
                                                        sl(false);
                                                        output.set(e.to_string())
                                                    }
                                                }
                                            }
                                        })
                                    }
                                },
                                div {
                                    class: "w-1/2 h-64 border-2 bg-white dark:bg-black border-yellow-500 rounded-md mx-2",
                                    h1 {
                                        class: "text-2xl text-slate-400 dark:text-white",
                                        "{output}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
    ))
}

pub fn Splash(cx: Scope) -> Element {
    cx.render(rsx!(
            div {
                class: "mx-auto sm:max-w-xl md:max-w-full lg:max-w-screen-xl md:px-24 lg:px-8 h-1/2 overflow-hidden bg-slate-200 rounded-lg shadow-md dark:bg-gray-900",
                div {
                    class: "p-6",
                    div {
                        h1 {
                            class: "block mt-2 text-4xl font-semibold text-white transition-colors duration-200 transform dark:text-white",
                            "",
                        }
                    }
                }
            }
    ))
}

pub fn NLP_service(cx: Scope) -> Element {
    let curr_service = use_read(&cx, SERVICE);
    cx.render(rsx!(
        div {
            class: "bg-white dark:bg-black h-1/2",
            match curr_service {
                Services::Splash => cx.render(rsx!(
                    Splash {}
                )),
                Services::Summarize => cx.render(rsx!(
                    Summarization {}
                )),
                Services::Translate => cx.render(rsx!(
                    Translation {}
                ))
            }
           // Loading {}
        }
    ))
}

pub fn Loading(cx: Scope) -> Element {
    let is_loading = use_read(&cx, LOADING);
    if is_loading.clone() {
        cx.render(rsx!(
                span {
                    class: "px-4 pt-1 h-4 w-4",
                    span {
                        class: "animate-ping absolute inline-flex w-4 h-4 rounded-full bg-amber-600 dark:bg-amber-300"
                    }
                }
        ))    
    }
    else {
        cx.render(rsx!(
                div{}
            ))
    }
}

async fn handle_prediction(target: &String, query: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("query", query);
    map.insert("target", target.to_string());
    client.post("http://127.0.0.1:8081/predict")
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
}

async fn handle_summarization(query: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("query", query);
    client.post("http://127.0.0.1:8081/summarize")
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
}

async fn send_service(service: String, client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let mut map = std::collections::HashMap::new();
    map.insert("service", service);
    client.post("http://127.0.0.1:8081/service")
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await
}

fn app(cx: Scope) -> Element {

    cx.render(rsx! (
        Nav {}
        body {
            class: "bg-white dark:bg-black h-full pt-18",
            NLP_service {},
        }   
    ))
}
