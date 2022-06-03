use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex text-3xl bg-yellow-600 p-2 fixed bottom-0 inset-x-0",
            "Pickl",
            i { class: "ml-auto fa-solid fa-bars p-1" }
        }
    })
}
