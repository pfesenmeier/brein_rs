use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct RowProps {
    name: String,
}

#[allow(non_snake_case)]
pub fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "text-lg drop-shadow-lg my-2 bg-slate-200 py-1 px-3",
            "{cx.props.name}"
        }
    })
}
