use web_sys::*;
use maud::*;
use crate::aabb;
use crate::cal;
use crate::uid;

/// Represents an object / event in the UI
#[derive(Clone)]
pub struct UICalO {
    pub data: std::rc::Rc<cal::CalendarObject>, 
    pub html_uidentifier: String,
}
impl UICalO {
    pub fn new(data: std::rc::Rc<cal::CalendarObject>, uidgen: &mut uid::UIDGenerator) -> Self {
        Self {
            data,
            html_uidentifier: uidgen.generate().to_string(),
        }
    }
    pub fn get_bounds(&self) -> aabb::Bounds {
        let element = window().unwrap().document().unwrap()
            .get_element_by_id(self.html_uidentifier.as_str())
            .expect("Catastrophic failture");
        let cr = element.get_bounding_client_rect();
        aabb::Bounds {
            x: cr.x(), y: cr.y(), w: cr.width(), h: cr.height()
        }
    }
    pub fn render(&self) -> Markup {
        if let cal::CalendarObject::Task(hdr, _) = self.data.as_ref() {
            return html! {
                div .task #(self.html_uidentifier) {
                    h1 .task_heading {(hdr.name)}
                    p .task_description {(hdr.description)}
                }
            }
        }
        html!()
    }
}
