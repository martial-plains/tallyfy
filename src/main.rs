#![allow(non_snake_case)]
#![warn(clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;

use components::{list::CounterList, modal::Modal, navbar::Navbar};
use dioxus::{
    desktop::{tao::dpi::Size, LogicalSize, WindowBuilder},
    prelude::*,
};

use dioxus_logger::tracing::{info, Level};
use enum_iterator::all;
use models::{Color, Counter};
use uuid::Uuid;

mod components;
mod models;

const STYLE: Asset = asset!("./public/styles/tailwind/tailwind.css");

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_window(
            WindowBuilder::new()
                .with_min_inner_size(Size::Logical(LogicalSize::new(500.0, 580.0)))
                .with_always_on_top(false),
        )
        .with_custom_head(format!(r#"<link rel="stylesheet" href="{STYLE}">"#));
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let get_color_name = |color: Color| match color {
        crate::models::Color::System => "base",
        crate::models::Color::Red => "red",
        crate::models::Color::Orange => "orange",
        crate::models::Color::Yellow => "yellow",
        crate::models::Color::Green => "green",
        crate::models::Color::Blue => "blue",
        crate::models::Color::Purple => "purple",
    };

    let mut counters = use_signal(Vec::new);

    let mut show_filter = use_signal(|| false);

    let mut filter_colors = use_signal(|| {
        all::<Color>()
            .map(|color| (color, false))
            .collect::<HashMap<Color, bool>>()
    });

    let mut filter_text = use_signal(String::new);

    let mut toggle_filter_color = move |color: Color| {
        let mut new_colors = filter_colors();
        new_colors.entry(color).and_modify(|value| {
            *value = !*value;
        });
        filter_colors.set(new_colors);
    };

    let is_color_selected =
        move |color: Color| filter_colors().get(&color).is_some_and(|color| *color);

    let filtered_counters = move || {
        counters()
            .into_iter()
            .filter(|counter: &Counter| {
                is_color_selected(counter.color)
                    || (!filter_text().is_empty() && counter.title.contains(&filter_text()))
            })
            .collect::<Vec<_>>()
    };

    let mut handle_add_counter = move |counter: Counter| {
        let combined: Vec<_> = counters()
            .into_iter()
            .chain(std::iter::once(counter))
            .collect();

        counters.set(combined);
    };

    let handle_increment = move |id: Uuid| {
        counters.set(
            counters()
                .into_iter()
                .map(|counter| {
                    if counter.id == id {
                        Counter {
                            count: counter.count + 1,
                            ..counter
                        }
                    } else {
                        counter
                    }
                })
                .collect(),
        );
    };

    let handle_decrement = move |id: Uuid| {
        counters.set(
            counters()
                .into_iter()
                .map(|counter| {
                    if counter.id == id {
                        Counter {
                            count: counter.count - 1,
                            ..counter
                        }
                    } else {
                        counter
                    }
                })
                .collect(),
        );
    };

    let handle_ontitlechange = move |(id, title): (Uuid, String)| {
        counters.set(
            counters()
                .into_iter()
                .map(|counter| {
                    if counter.id == id {
                        Counter {
                            title: title.clone(),
                            ..counter
                        }
                    } else {
                        counter
                    }
                })
                .collect(),
        );
    };

    let handle_onvaluechange = move |(id, value): (Uuid, usize)| {
        counters.set(
            counters()
                .into_iter()
                .map(|counter| {
                    if counter.id == id {
                        Counter {
                            count: value,
                            ..counter
                        }
                    } else {
                        counter
                    }
                })
                .collect(),
        );
    };

    let handle_ondelete = move |id: Uuid| {
        counters.set(
            counters()
                .into_iter()
                .filter(|counter| counter.id != id)
                .collect(),
        );
    };

    let handle_onmoveup = move |id: Uuid| {
        let mut old_counters = counters();
        let pos = counters()
            .iter()
            .position(|counter| counter.id == id)
            .unwrap();

        if pos > 0 {
            old_counters.swap(pos, pos - 1);
            counters.set(old_counters);
        }
    };

    let handle_onmovetop = move |id: Uuid| {
        let mut new_counters = counters();
        let pos = new_counters
            .iter()
            .position(|counter| counter.id == id)
            .unwrap();
        let counter = new_counters.remove(pos);
        new_counters.insert(0, counter);

        counters.set(new_counters);
    };

    let handle_onmovedown = move |id: Uuid| {
        let mut new_counters = counters();
        let pos = counters()
            .iter()
            .position(|counter| counter.id == id)
            .unwrap();

        if pos < counters().len().saturating_sub(1) {
            new_counters.swap(pos, pos + 1);
            counters.set(new_counters);
        }
    };

    let handle_onmovebottom = move |id: Uuid| {
        let mut new_counters = counters();
        let pos = new_counters
            .iter()
            .position(|counter| counter.id == id)
            .unwrap();

        let counter = new_counters.remove(pos);
        new_counters.push(counter);

        counters.set(new_counters);
    };

    let handle_reorder = move |new_counters| {
        counters.set(new_counters);
    };

    let handle_onsetcolor = move |(id, color): (Uuid, Color)| {
        counters.set(
            counters()
                .into_iter()
                .map(|counter| {
                    if counter.id == id {
                        Counter { color, ..counter }
                    } else {
                        counter
                    }
                })
                .collect(),
        );
    };

    rsx! {
        div { class: "round",
            Navbar {
                start_content: rsx! {
                    button {
                        class: "btn btn-ghost btn-circle",
                        onclick: move |_| handle_add_counter(Counter::default()),
                        img {
                            class: "w-1/2 h-1/2 dark:invert",
                            src: asset!("public/assets/plus.svg"),
                        }
                    }
                },
                center_content: rsx! {
                    p { class: "text-xl select-none", "Tallyfy" }
                },
                end_content: rsx! {
                    button {
                        class: format!(
                            "btn btn-ghost btn-circle {}",
                            if show_filter() { "btn-active" } else { "" },
                        ),
                        onclick: move |_| show_filter.set(!show_filter()),
                        img {
                            class: "w-1/2 h-1/2 dark:invert",
                            src: asset!("public/assets/search.svg"),
                        }
                    }
                    button { class: "btn btn-ghost btn-circle", "onclick": "about_modal.showModal()",
                        img {
                            class: "w-1/2 h-1/2 dark:invert",
                            src: asset!("public/assets/info.svg"),
                        }
                    }
                    Modal { id: "about_modal",
                        h1 { class: "text-xl text-center", "Tallyfy" }
                    }
                },
            }

            div { class: "flex flex-col",

                if show_filter() {
                    div { class: "flex flex-col items-center justify-center",
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
                                        onclick: move |_| {
                                            toggle_filter_color(color);
                                        },
                                    }
                                }
                            }
                        }
                        label { class: "input input-bordered flex items-center gap-2 w-1/2",
                            input {
                                placeholder: "Filter by text...",
                                r#type: "text",
                                class: "grow",
                                value: filter_text(),
                                oninput: move |e| filter_text.set(e.value()),
                            }

                            img {
                                class: "w-6 h-6 dark:invert",
                                src: asset!("public/assets/search.svg"),
                            }
                        }
                    }
                }

                CounterList {
                    counters: if show_filter() { filtered_counters() } else { counters() },
                    onincrement: handle_increment,
                    ondecrement: handle_decrement,
                    onreorder: handle_reorder,
                    ontitlechange: handle_ontitlechange,
                    onvaluechange: handle_onvaluechange,
                    ondelete: handle_ondelete,
                    onmovedown: handle_onmovedown,
                    onmovebottom: handle_onmovebottom,
                    onmoveup: handle_onmoveup,
                    onmovetop: handle_onmovetop,
                    onsetcolor: handle_onsetcolor,
                }
            }
        }
    }
}
