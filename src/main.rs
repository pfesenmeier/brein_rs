use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            class: "flex text-5xl bg-yellow-600 p-5 fixed bottom-0 inset-x-0",
            "Pickl", 
            i { class: "ml-auto fa-solid fa-bars" }
        }
    })
}

