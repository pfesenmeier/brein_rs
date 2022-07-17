use crate::data::load_recipes;
use crate::data::RecipeDatabase;
use dioxus::prelude::*;
use std::ffi::OsString;

#[derive(Props, PartialEq, Clone)]
pub struct RecipeModel {
    pub id: u32,
    pub name: String,
    pub path: OsString,
    pub recipe: String,
}

impl RecipeModel {
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
    Edit,
}

#[allow(non_snake_case)]
pub fn Recipe(cx: Scope) -> Element {
    let database = RecipeDatabase::new(load_recipes());
    let route = use_route(&cx);
    if let Some(id) = route.segment("id") {
        if let Ok(id) = u32::from_str_radix(id, 10) {
            if let Some(recipe) = database.get_recipe(id) {
                let edit_state = use_state(&cx, || EditorState::Display);
                let recipe_body = use_state(&cx, || recipe.recipe.clone());

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
                        EditorState::Display => {
                            let options = pulldown_cmark::Options::empty();
                            let parser = pulldown_cmark::Parser::new_ext(recipe_body, options);
                            let mut html_output = String::new();
                            pulldown_cmark::html::push_html(&mut html_output, parser);

                            rsx! {
                                div {
                                    class: "bg-amber-100 h-max text-xl mx-2",
                                    "{recipe.name}",
                                }
                                div {
                                    class: "markdown-editor",
                                    dangerous_inner_html: "{html_output}"
                                },
                            }},
                        EditorState::Edit => {
                            let value = recipe_body.get();
                            rsx!{  
                                textarea {
                                    class: "w-full h-screen",
                                    onchange: move |event| recipe_body.set((*event.value.clone()).to_string()),
                                    value: "{value}"
                                } 
                            }
                        },
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
