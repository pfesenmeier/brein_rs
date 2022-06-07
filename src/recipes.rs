use dioxus::prelude::*;
use crate::row::Row;
use crate::nav_bar::NavBar;
use crate::data::{RecipeDatabase, load_recipes};

#[allow(non_snake_case)]
pub fn Recipes(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let recipes = database.get_recipes();

    cx.render(rsx! {
        recipes.iter().map(|recipe| { rsx!(Row { id: recipe.id, name: recipe.name.clone() }) }),
        NavBar()
    })
}
