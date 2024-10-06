mod components;
mod model;


pub use components::*;
pub use model::*;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    
    #[layout(Footer)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
}
