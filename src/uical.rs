use maud::{html, Markup, Render};
use web_sys::*;

use crate::cal;
use crate::uid;
use crate::aabb;
use crate::uicalo::UICalO;

// What is a location and what is an id
type CLocation = u64; // hash of cal::Time
type CIdentifier = usize;

enum CalendarViewType {
    Week,
}
pub struct UICalendar {
    pub agenda: Vec<UICalO>,
    pub ghost_remove: Option<CIdentifier>,
    pub ghost_add: Option<(UICalO, CLocation)>,
}
impl UICalendar {
    pub fn draggable_areas(&self) -> Vec<(aabb::Bounds, CLocation)> {
        // Whatever is going to happen here
        Vec::new()
    }
    pub fn tmp_remove(&mut self, identifier: Option<CIdentifier>) {
        self.ghost_remove = identifier;
    }
    pub fn tmp_add(&mut self, element: Option<(UICalO, CLocation)>) {
        self.ghost_add = element;
    }
    pub fn render(&self) -> Markup {
        // Always assume a week rendering
        let slots_per_day = 24 * 60 / cal::MINUTE_INC;
        let slots_per_week = slots_per_day * 7;

        html! {
            .wcalendar {
                .wscheduled style="grid" {
                    "Hi there: I am an event"
                }
            }
        }
    }
}
