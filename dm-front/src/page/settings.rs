use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Settings() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Settings"
    }
}
