use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::counter::TallyCounter,
    models::{Color, Counter},
};

#[component]
pub fn CounterList(
    counters: Vec<Counter>,
    onincrement: EventHandler<Uuid>,
    ondecrement: EventHandler<Uuid>,
    ondelete: EventHandler<Uuid>,
    onmoveup: EventHandler<Uuid>,
    onmovetop: EventHandler<Uuid>,
    onmovedown: EventHandler<Uuid>,
    onmovebottom: EventHandler<Uuid>,
    onreorder: EventHandler<Vec<Counter>>,
    ontitlechange: EventHandler<(Uuid, String)>,
    onvaluechange: EventHandler<(Uuid, usize)>,
    onsetcolor: EventHandler<(Uuid, Color)>,
) -> Element {
    rsx! {
        div { class: "join join-vertical mx-auto justify-center my-7",
            for item in counters {
                TallyCounter {
                    counter: item,
                    onincrement,
                    ondecrement,
                    ondelete,
                    onmovedown,
                    onmovebottom,
                    onmovetop,
                    onmoveup,
                    onvaluechange,
                    ontitlechange,
                    onsetcolor
                }
            }
        }
    }
}
