#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use reqwest::Client;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let future = use_resource(|| async move {
        let client = Client::new();
        let res = client
            .get("http://127.0.0.1:3000/list_all")
            .send()
            .await
            .unwrap();
        res.text().await.unwrap()
    });

    match &*future.read_unchecked() {
        Some(s) => {
            rsx! {
                div { "项目列表" },
                div { "{s}" }
            }
        }
        None => {
            rsx! {
                div { "Loading..." }
            }
        }
    }
}
