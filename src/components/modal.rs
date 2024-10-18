use dioxus::prelude::*;

#[component]
pub fn Modal(id: String, #[props(into, default = rsx!())] children: Element) -> Element {
    rsx! {
        dialog { class: "modal", id,
            div { class: "modal-box",
                form { method: "dialog",
                    button { class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                        "✕"
                    }
                    {children}
                }
            }
        }
    }
}
