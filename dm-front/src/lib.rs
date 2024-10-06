mod components;
mod model;

pub use components::*;
use dioxus::prelude::*;
pub use model::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    
    #[layout(Footer)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
}
