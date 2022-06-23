extern crate core;

use brein_rs::{recipe, recipes};
use dioxus::{
    prelude::*,
    router::{Route, Router},
};

fn main() {
    dioxus::web::launch(app);
}

#[allow(non_upper_case_globals)]
static app: Component = |cx| {
    cx.render(rsx! {
        Router {
            // see https://github.com/DioxusLabs/dioxus/issues/432
            Route { to: "/recipe/:id", self::recipe::Recipe{} }
            Route { to: "/", self::recipes::Recipes { } }
        }
    })
};
