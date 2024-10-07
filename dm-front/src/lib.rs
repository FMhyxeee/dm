mod components;
mod model;
mod page;

use components::Footer;
use dioxus::prelude::*;

use crate::page::Home;
use crate::page::Settings;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    
    #[layout(Footer)]
        #[route("/")]
        Home {},
        #[route("/settings")]
        Settings {},
}
