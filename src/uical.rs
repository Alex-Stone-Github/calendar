use maud::{html, Markup, Render};
use web_sys::*;

use crate::cal;
use crate::ui;
use crate::aabb;
use crate::uicalo::UICalO;

// TODO: Useless now
enum CalendarViewType {
    Week,
}

/// This is a basic calendar render of a week view
pub struct UICalendar {
    agenda: Vec<UICalO>,
    ghost_remove: Option<usize>,
    ghost_add: Option<UICalO>,
    target: Element,
}
impl UICalendar {
    pub fn new(agenda: Vec<UICalO>, target: Element) -> Self {
        Self {
            agenda,
            ghost_remove: None,
            ghost_add: None,
            target,
        }
    }
    pub fn get_bounds(&self) -> aabb::Bounds {
        let src = self.target.get_bounding_client_rect();
        aabb::Bounds {
            x: src.x(),
            y: src.y(),
            w: src.width(),
            h: src.height(),
        }
    }
}

impl ui::Droppable for UICalendar {
    fn preview(&mut self, point: aabb::Point, element: UICalO) -> bool {
        let dim = self.get_bounds();

        // Make sure that the mouse is within the calendar
        if !aabb::intersects(&dim, &point) {
            return false;
        }

        let slot_x_count = 7;
        let slots_per_day = 24 * 60 / cal::MINUTE_INC;
        let slot_y_count = slots_per_day;
        let slot_width_px = dim.w / slot_x_count as f64;
        let slot_height_px = dim.h / slot_y_count as f64;
        let slot_x = (point.x / slot_width_px).floor() as usize;
        let slot_y = (point.y / slot_height_px).floor() as usize;

        let start_minute = slot_y as u32 * cal::MINUTE_INC;
        let sample_time = cal::Time {
            year: 2025,
            month: 0,
            day: slot_y as u32,
            hour: start_minute / 60,
            minute: start_minute % 60,
        };
        let sample_duration = cal::MINUTE_INC;

        // Create a new piece of data to reference
        let mut sample = element.clone();
        sample.data = std::rc::Rc::new(cal::CalendarObject::event(
            element.data.header.clone(),
            sample_time,
            sample_duration,
        ));
        self.ghost_add = Some(sample);

        true
    }
    fn drop(&mut self) {
        ()
    }
}
impl ui::UIElement for UICalendar {
    fn draw(&self) {
        let markup = self.render();
        self.target.set_inner_html(markup.into_string().as_str());
    }
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
            let start_minute = x.data.time.unwrap().hour * 60 + x.data.time.unwrap().minute;
            let start_slot = start_minute/cal::MINUTE_INC+1;
            let end_slot = start_slot + x.data.duration.unwrap()/cal::MINUTE_INC;
            let css = format!("
                background: pink;
                grid-column: {};
                grid-row-start: {};
                grid-row-end: {};
            ", x.data.time.unwrap().day+1, start_slot, end_slot);
            html! {
                .wscheduled style=(css) {
                    (x.render_as_event())
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
