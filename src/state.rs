use web_sys::*;
use maud::*;

use crate::uical;
use crate::uid;
use crate::ph;
use crate::uitask;
use crate::uicalo;
use crate::cal;
use crate::event;

pub struct Globals {
    pub uidgen: uid::UIDGenerator,
    pub tasks: uitask::UITaskList,
    pub tasks_html: Element,
    pub calendar: uical::UICalendar,
    pub calendar_html: Element,
    pub placeholder: ph::Placeholder,
    pub placeholder_html: Element,
    pub event_state: event::EventState,
}

pub fn get_window() -> Window {
    window().expect("Cannot Access Window Object")
}
pub fn get_document() -> Document {
    get_window().document().expect("Cannot Access Document Object")
}

pub fn setup() -> Globals {
    let cevent1: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Birthday Party"),
                description: "".into(),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            },
            duration: 60,
        }
    );
    let cevent2: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Birthday Party"),
                description: "".into(),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 2,
                hour: 1,
                minute: 0,
            },
            duration: 30,
        }
    );
    let cevent3: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Other Thing"),
                description: "".into(),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 1,
                hour: 0,
                minute: 15,
            },
            duration: 120,
        }
    );
    let ttrash: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Take out the trash"),
                description: String::from(""),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            },
            duration: 60,
        });
    let tkit: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Clean the kitchen"),
                description: String::from(""),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            },
            duration: 60,
        });
    let tball: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Throw red ball"),
                description: String::from("<i>No the blue one</i>"),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            },
            duration: 60,
        });
    let tisa: std::rc::Rc<cal::CalendarObject> = std::rc::Rc::new(
        cal::CalendarObject {
            header: cal::Header {
                name: String::from("Say hi to Isabelle"),
                description: String::from("<i>What is her favorite color?</i>"),
            },
            time: cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            },
            duration: 60,
        });
    // Create All HTML Elements
    let document = get_document();

    let tasks_html= document.create_element("section").unwrap();
    let calendar_html = document.create_element("section").unwrap();
    let placeholder_html = document.create_element("section").unwrap();
    let main = document.get_element_by_id("main").expect("Invalid Starting HTML Document");
    let _ = main.append_child(&tasks_html);
    let _ = main.append_child(&placeholder_html);
    let _ = main.append_child(&calendar_html);

    let mut generator = uid::UIDGenerator(0);
    let mut ids = (0..7).map(
        |_| generator.generate().to_string()
        ).collect::<Vec<String>>().into_iter();
    Globals {
        uidgen: generator,
        tasks: uitask::UITaskList {
            tasks: vec![
                uicalo::UICalO::new(tisa, ids.next().unwrap()),
                uicalo::UICalO::new(tkit, ids.next().unwrap()),
                uicalo::UICalO::new(tball, ids.next().unwrap()),
                uicalo::UICalO::new(ttrash, ids.next().unwrap()),
            ],
            ghost_remove: None,
            ghost_add: None,
        },
        tasks_html: tasks_html,
        calendar: uical::UICalendar {
            agenda: vec![
                uicalo::UICalO::new(cevent1, ids.next().unwrap()),
                uicalo::UICalO::new(cevent2, ids.next().unwrap()),
                uicalo::UICalO::new(cevent3, ids.next().unwrap()),
            ],
            ghost_remove: None,
            ghost_add: None,
        },
        calendar_html: calendar_html,
        placeholder: ph::Placeholder::new(),
        placeholder_html: placeholder_html,
        event_state: event::EventState::default(),
    }
}

