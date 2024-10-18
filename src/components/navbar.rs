use dioxus::prelude::*;

#[component]
pub fn Navbar(
    #[props(into, default = rsx!())] start_content: Element,
    #[props(into, default = rsx!())] center_content: Element,
    #[props(into, default = rsx!())] end_content: Element,
) -> Element {
    rsx! {
        div { class: "navbar bg-base-100",
            div { class: "navbar-start", {start_content} }
            div { class: "navbar-center", {center_content} }
            div { class: "navbar-end", {end_content} }
        }
    }
}
