use dioxus::prelude::*;
use brein_rs::{
    nav_bar::NavBar,
    row::Row,
    data::{get_recipes, Recipe}
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let recipes = get_recipes();
    
    cx.render(rsx!{
        recipes.iter().map(|recipe| { rsx!(Row { name: recipe.name.clone() }) }),
        NavBar()
    })
}
