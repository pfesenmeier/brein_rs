use crate::data::load_recipes;
use crate::data::RecipeDatabase;
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

#[derive(PartialEq)]
enum EditorState {
    Display,
    Edit
}

#[allow(non_snake_case)]
pub fn Recipe(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let route = use_route(&cx);
    if let Some(id) = route.segment("id") {
        if let Ok(id) = u32::from_str_radix(id, 10) {
            if let Some(recipe) = database.get_recipe(id) {
                let edit_state = use_state(&cx, || EditorState::Display);

                cx.render(rsx! {
                    div {
                        class: "flex",
                        div {
                          class: "p-1 bg-blue-100",
                          onclick: move |_| edit_state.set(EditorState::Display),
                          "preview"
                        }
                        div {
                          class: "p-1 bg-red-100",
                          onclick: move |_| edit_state.set(EditorState::Edit),
                          "edit"
                        }
                    }
                    match *edit_state.get() {
                    EditorState::Display => rsx! {
                    div {
                        class: "bg-amber-100 h-max text-xl",
                        "{recipe.name}",
                    }
                    div {
                        "{recipe.recipe}"
                    }},
                    EditorState::Edit => rsx!{  "editor" },
                    },
                })
            } else {
               cx.render(rsx! { "Whoops, did not find that recipe" })
            }
        } else {
            cx.render(rsx! { "Whoops, could not parse a u32 from recipe id" })
        }
    } else {
        cx.render(rsx! { "whoops, internal error. Requested a non-existent route segment" })
    }
}
