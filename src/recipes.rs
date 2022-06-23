use crate::data::{load_recipes, RecipeDatabase};
use crate::nav_bar::NavBar;
use crate::row::Row;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Recipes(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let recipes = database.get_recipes();

    cx.render(rsx! {
        recipes.iter().map(|recipe| { rsx!(Row { id: recipe.id, name: recipe.name.clone() }) }),
        NavBar()
    })
}
