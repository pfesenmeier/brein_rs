use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct RowProps {
    name: String
}

#[allow(non_snake_case)]
pub fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "text-l",
            "{cx.props.name}"
        }
    })
}
