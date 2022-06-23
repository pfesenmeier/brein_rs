use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct RowProps {
    id: u32,
    name: String,
}

#[allow(non_snake_case)]
pub fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
            div {
                class: "flex content-center text-lg drop-shadow-lg my-2 bg-slate-200 py-1 px-3",
                "{cx.props.name}"
                Link {
                  class: "flex self-center ml-auto"
                  to: "/recipe/{cx.props.id}",
    //              i { class: "fa-arrow-right fa-solid" }
                  "=>"
                }
            }
        })
}
