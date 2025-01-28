use web_sys::*;
use maud::*;
use crate::aabb;
use crate::cal;
use crate::uid;

/// Represents an object / event in the UI
#[derive(Clone)]
pub struct UICalO {
    pub data: std::rc::Rc<cal::CalendarObject>,  // TODO: Introduce RefCell
    html_uidentifier: String,
}
/// UI Primitive for a calendar item
impl UICalO {
    /// Create a new uicalo
    pub fn new(data: cal::CalendarObject, uid: &mut uid::UIDGenerator) -> Self {
        Self {
            data: std::rc::Rc::new(data),
            html_uidentifier: uid.generate().to_string(),
        }
    }
    /// Reterns the bounding box of the element
    pub fn get_bounds(&self) -> aabb::Bounds {
        let element = window().unwrap().document().unwrap()
            .get_element_by_id(self.html_uidentifier.as_str())
            .expect("Catastrophic failture");
        let cr = element.get_bounding_client_rect();
        aabb::Bounds {
            x: cr.x(), y: cr.y(), w: cr.width(), h: cr.height()
        }
    }
    /// Reters the html needed to render this object as an html task
    pub fn render_as_task(&self) -> Markup {
        return html! {
            .task id=(self.html_uidentifier) {
                b {(self.data.header.name)}
                p {(self.data.header.description)}
            }
        }
    }
    pub fn render_as_event(&self) -> Markup {
        // TODO: Change
        self.render_as_task()
    }
}
