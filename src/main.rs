extern crate core;

use brein_rs::data::{load_recipes, RecipeDatabase};
use brein_rs::{data, nav_bar::NavBar, recipe, row::Row};
use dioxus::{
    prelude::*,
    router::{Route, Router},
};
use std::ffi::OsString;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let recipes = database.get_recipes();

    cx.render(rsx! {
        Router {
            Route { to: "/recipe" self::recipe::Recipe { id: 1, name: "foo".into() path: OsString::new(), recipe: "bar".into()}}
            Route { to: "/" self::Recipes{ recipes: recipes }}
        }
    })
}

#[derive(Props, PartialEq)]
pub struct RecipesProps {
    recipes: Vec<recipe::RecipeProps>,
}

#[allow(non_snake_case)]
pub fn Recipes(cx: Scope<RecipesProps>) -> Element {
    cx.render(rsx! {
        cx.props.recipes.iter().map(|recipe| { rsx!(Row { name: recipe.name.clone() }) }),
        NavBar()
    })
}
