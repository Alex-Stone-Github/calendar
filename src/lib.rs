use wasm_bindgen::prelude::*;
use web_sys::*;
use maud::*;

mod cal;
mod uical;
mod uitask;
mod uicalo;
mod wapi;
mod uid;
mod aabb;
mod state;

#[wasm_bindgen]
pub fn main() {
    let ttrash = std::rc::Rc::new(cal::CalendarObject::Task(
            cal::Header {
                name: String::from("Take out the trash"),
                description: String::from(""),
            }, None
        ));
    let tkit = std::rc::Rc::new(cal::CalendarObject::Task(
            cal::Header {
                name: String::from("Clean the kitchen"),
                description: String::from(""),
            }, None
        ));
    let tball = std::rc::Rc::new(cal::CalendarObject::Task(
            cal::Header {
                name: String::from("Throw red ball"),
                description: String::from("<i>No the blue one</i>"),
            }, None
        ));
    let tisa = std::rc::Rc::new(cal::CalendarObject::Task(
            cal::Header {
                name: String::from("Say hi to Isabelle"),
                description: String::from("<i>What is her favorite color?</i>"),
            }, None
        ));
    let window = window().unwrap();
    let document = window.document().unwrap();
    let tasks= document.create_element("section").unwrap();
    let calendar = document.create_element("section").unwrap();
    let main = document.get_element_by_id("main").unwrap();
    let _ = main.append_child(&tasks);
    let _ = main.append_child(&calendar);

    let mut uidgen = uid::UIDGenerator(0);
    let mut ui: uitask::UITaskList = uitask::UITaskList {
        tasks: vec![
            uicalo::UICalO::new(ttrash.clone(), &mut uidgen),
            uicalo::UICalO::new(tball.clone(), &mut uidgen),
            uicalo::UICalO::new(tkit.clone(), &mut uidgen),
            uicalo::UICalO::new(tisa.clone(), &mut uidgen),
        ],
        ghost_remove: None,
        ghost_add: None,
    };
    ui.tmp_remove(None);
    ui.tmp_add(
            Some(
                (uicalo::UICalO::new(tisa.clone(), &mut uidgen), 1)
            )
        );
    tasks.set_inner_html(
        ui.render().into_string().as_str()
    );
    let calendarui = uical::UICalendar {
        agenda: vec![],
        ghost_remove: None,
        ghost_add: None
    };
    calendar.set_inner_html(
        calendarui.render().into_string().as_str()
    );

    { // register a event
        let cb = 
            Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            mevent(event, &mut ui, &tasks);
        });
        document.add_event_listener_with_callback(
            "click", cb.as_ref().unchecked_ref())
            .expect("Failed to add event listener");
        cb.forget();
    }

}
pub fn mevent(event: MouseEvent, uir: &mut uitask::UITaskList, root: &Element) {
    let string = format!("X: {}, Y: {}", event.x(), event.y());
    wapi::log(string.as_str());

    let bounds = uir.draggable_areas();
    for bound in bounds {
        let x = bound.0.x;
        let y = bound.0.y;
        let w = bound.0.w;
        let h = bound.0.h;

        let collides = aabb::intersects(&bound.0, &aabb::Point {
            x: event.x() as f64, y: event.y() as f64
        });
        if collides {
            wapi::log(
                format!("{}, {}  w:{} h:{} u:{}", x, y, w, h, bound.1)
                .as_str());
            // remove the element
            uir.tmp_remove(Some(bound.1));
            root.set_inner_html(
                uir.render().into_string().as_str()
            );
        }
    }
}

