use maud::{html, Markup, Render};
use web_sys::*;

use crate::cal;
use crate::ui;
use crate::aabb;
use crate::uicalo::UICalO;

enum CalendarViewType {
    Week,
}
pub struct UICalendar {
    pub agenda: Vec<UICalO>,
    pub ghost_remove: Option<usize>,
    pub ghost_add: Option<UICalO>,
}
impl ui::Droppable for UICalendar {
    fn preview(&mut self, point: aabb::Point, element: UICalO) -> bool {
        let slots_per_day = 24 * 60 / cal::MINUTE_INC;
        //let width_px = something;
    }
    fn drop() {
        ()
    }
}
impl ui::UIElement for UICalendar {
    fn render(&self) -> Markup {
        // add the ghost element
        let mut torender = self.agenda.clone();
        if let Some(element) = self.ghost_add.clone() {
            torender.push(element);
        }

        let slots_per_day = 24 * 60 / cal::MINUTE_INC;
        let week_calendar_slot_css = 
            format!("grid-template-rows: repeat({}, 40px", slots_per_day);

        let calendar_event_html = torender.iter().map(|x| {
            let start_minute = x.data.time.hour * 60 + x.data.time.minute;
            let start_slot = start_minute/cal::MINUTE_INC+1;
            let end_slot = start_slot + x.data.duration/cal::MINUTE_INC;
            let css = format!("
                background: pink;
                grid-column: {};
                grid-row-start: {};
                grid-row-end: {};
            ", x.data.time.day+1, start_slot, end_slot);
            html! {
                .wscheduled style=(css) {
                    (x.render())
                }
            }
        }).reduce(|a, b| html! {
            (a)(b)
        }).unwrap();

        html! {
            .wcalendar style=(week_calendar_slot_css) {
                (calendar_event_html)
            }
        }
    }
}
