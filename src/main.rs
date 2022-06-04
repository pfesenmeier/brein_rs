extern crate core;

use brein_rs::data::{load_recipes, RecipeDatabase};
use brein_rs::{nav_bar::NavBar, recipe, row::Row};
use dioxus::{
    prelude::*,
    router::{Route, Router},
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            // see https://github.com/DioxusLabs/dioxus/issues/432
            Route { to: "/recipe/:id", self::recipe::Recipe{} }
            Route { to: "/", self::Recipes { } }
        }
    })
}

#[allow(non_snake_case)]
pub fn Recipes(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let recipes = database.get_recipes();

    cx.render(rsx! {
        recipes.iter().map(|recipe| { rsx!(Row { id: recipe.id, name: recipe.name.clone() }) }),
        NavBar()
    })
}
