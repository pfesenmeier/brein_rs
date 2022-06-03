use dioxus::prelude::*;
use std::ffi::OsString;

#[derive(Props, PartialEq, Clone)]
pub struct RecipeProps {
    pub id: u32,
    pub name: String,
    pub path: OsString,
    pub recipe: String,
}

impl RecipeProps {
    pub fn new(id: u32, name: String, path: OsString, recipe: String) -> Self {
        Self {
            id,
            name,
            path,
            recipe,
        }
    }
}
#[allow(non_snake_case)]
pub fn Recipe(cx: Scope<RecipeProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "bg-amber-100 h-max text-xl",
            "This is my recipe"
        }
        div {
            "step 1: go to the store"
        }
    })
}
