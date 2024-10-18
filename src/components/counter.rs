use dioxus::prelude::*;
use enum_iterator::all;
use uuid::Uuid;

use crate::models::{Color, Counter};

#[component]
pub fn TallyCounter(
    counter: Counter,
    onincrement: EventHandler<Uuid>,
    ondecrement: EventHandler<Uuid>,
    ondelete: EventHandler<Uuid>,
    onmoveup: EventHandler<Uuid>,
    onmovetop: EventHandler<Uuid>,
    onmovedown: EventHandler<Uuid>,
    onmovebottom: EventHandler<Uuid>,
    onsetcolor: EventHandler<(Uuid, Color)>,
    ontitlechange: EventHandler<(Uuid, String)>,
    onvaluechange: EventHandler<(Uuid, usize)>,
) -> Element {
    let mut is_open = use_signal(|| false);

    let get_color_name = |color: Color| match color {
        crate::models::Color::System => "base",
        crate::models::Color::Red => "red",
        crate::models::Color::Orange => "orange",
        crate::models::Color::Yellow => "yellow",
        crate::models::Color::Green => "green",
        crate::models::Color::Blue => "blue",
        crate::models::Color::Purple => "purple",
    };

    let is_color_selected = |color: Color| counter.color == color;

    let background_color = match counter.color {
        Color::System => "bg-base-200 hover:bg-base-300",
        Color::Red => "bg-red-200 hover:bg-red-300 dark:bg-red-500 dark:hover:bg-red-400",
        Color::Orange => {
            "bg-orange-200 hover:bg-orange-300 dark:bg-orange-500 dark:hover:bg-orange-400"
        }
        Color::Yellow => {
            "bg-yellow-200 hover:bg-yellow-300 dark:bg-yellow-500 dark:hover:bg-yellow-400"
        }
        Color::Green => "bg-green-200 hover:bg-green-300 dark:bg-green-500 dark:hover:bg-green-400",
        Color::Blue => "bg-blue-200 hover:bg-blue-300 dark:bg-blue-500 dark:hover:bg-blue-400",
        Color::Purple => {
            "bg-purple-200 hover:bg-purple-300 dark:bg-purple-500 dark:hover:bg-purple-400"
        }
    };

    rsx! {
        div { class: "container",
            div { class: "join-item p-2 rounded-lg flex justify-center text-base-content {background_color} shadow-md max-w-[36rem] mx-14 hover:ease-in-out",
                div { class: "join flex flex-row",
                    input {
                        class: "join-item w-full bg-inherit dark:text-white",
                        value: counter.title,
                        oninput: move |e| ontitlechange((counter.id, e.value())),
                        r#type: "text"
                    }
                    input {
                        class: "join-item w-full text-end bg-inherit dark:text-white",
                        oninput: move |e| {
                            if let Ok(value) = e.value().trim().parse::<usize>() {
                                onvaluechange((counter.id, value));
                            }
                        },
                        value: counter.count.to_string()
                    }
                    div { class: "space-x-1 flex flex-row mx-4",
                        button {
                            class: "btn btn-circle {background_color} border-0 hover:ease-in-out",
                            disabled: counter.count == usize::MIN,
                            onclick: move |_| ondecrement(counter.id),
                            img {
                                class: "h-6 w-6 dark:invert",
                                src: asset!("public/assets/minus.svg")
                            }
                        }
                        button {
                            class: "btn btn-circle {background_color} border-0 hover:ease-in-out",
                            disabled: counter.count == usize::MAX,
                            onclick: move |_| onincrement(counter.id),
                            img {
                                class: "h-6 w-6 dark:invert",
                                src: asset!("public/assets/plus.svg")
                            }
                        }
                        details {
                            class: "dropdown",
                            onclick: move |_| is_open.set(!is_open()),
                            "open": is_open(),
                            summary { class: "btn m-1 border-0 {background_color} hover:ease-in-out",
                                img {
                                    class: "h-1/2 w-1/2 dark:invert",
                                    src: asset!("public/assets/more-horizontal.svg")
                                }
                            }
                            ul { class: "menu dropdown-content bg-base-100 rounded-box z-[1] p-2 shadow w-fit",
                                div { class: "flex space-x-0.5 flex-row",
                                    for color in all::<Color>() {

                                        div { class: format!("flex-item {}", if is_color_selected(color) { "indicator" } else { "" }),
                                            if is_color_selected(color) {
                                                span { class: "indicator-item badge badge-secondary" }
                                            }
                                            button {
                                                class: format!(
                                                    "btn btn-circle {}",
                                                    if matches!(color, Color::System) {
                                                        String::new()
                                                    } else {
                                                        let color = get_color_name(color);
                                                        format!("bg-{color}-500")
                                                    },
                                                ),
                                                onclick: move |_| onsetcolor((counter.id, color))
                                            }
                                        }
                                    }
                                }

                                div { class: "space-x-1",
                                    div { class: "!join",
                                        button {
                                            class: "block btn join-item tooltip",
                                            "data-tip": "Move Up",
                                            onclick: move |_| { onmoveup(counter.id) },
                                            img {
                                                class: "h-6 w-6 dark:invert",
                                                src: asset!("public/assets/chevron-up.svg")
                                            }
                                        }
                                        button {
                                            class: "block btn content-center join-item tooltip",
                                            "data-tip": "Move Down",
                                            onclick: move |_| { onmovedown(counter.id) },
                                            img {
                                                class: "h-6 w-6 dark:invert",
                                                src: asset!("public/assets/chevron-down.svg")
                                            }
                                        }
                                    }

                                    div { class: "!join",
                                        button {
                                            class: "block btn join-item tooltip",
                                            "data-tip": "Move Top",
                                            onclick: move |_| { onmovetop(counter.id) },
                                            img {
                                                class: "h-6 w-6 dark:invert",
                                                src: asset!("public/assets/chevron-top.svg")
                                            }
                                        }
                                        button {
                                            class: "block btn content-center join-item tooltip",
                                            "data-tip": "Move Bottom",
                                            onclick: move |_| { onmovebottom(counter.id) },
                                            img {
                                                class: "h-6 w-6 dark:invert",
                                                src: asset!("public/assets/chevron-bottom.svg")
                                            }
                                        }
                                    }

                                    button {
                                        class: "btn tooltip",
                                        "data-tip": "Delete",
                                        onclick: move |_| { ondelete(counter.id) },
                                        div { class: "flex justify-center items-center",
                                            img {
                                                class: "h-6 w-6 dark:invert object-contain",
                                                src: asset!("public/assets/trash.svg")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
