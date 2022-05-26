mod obj;
use obj::components::{ Service, Head, Nav, Body, NLP_service };
use dioxus::prelude::*;

fn app(cx: Scope) -> Element {

    cx.render(rsx! (
        Head {}
        Nav {}
        Body {}
    ))
}

fn main() {
    dioxus::web::launch(app);
}
