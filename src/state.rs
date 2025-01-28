use web_sys::*;

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
    pub calendar: uical::UICalendar,
    pub placeholder: ph::Placeholder,
    pub event_state: event::EventState,
}

pub fn get_window() -> Window {
    window().expect("Cannot Access Window Object")
}
pub fn get_document() -> Document {
    get_window().document().expect("Cannot Access Document Object")
}

pub fn setup() -> Globals {
    // Create All HTML Elements
    let document = get_document();
    let tasks_html= document.get_element_by_id("tasks").expect("NO tasks element!");
    let calendar_html= document.get_element_by_id("calendar").expect("NO calendar element!");
    let placeholder_html= document.get_element_by_id("placeholder").expect("NO placeholder element!");

    let mut generator = uid::UIDGenerator(0);

    use uicalo::UICalO;
    let c1 = UICalO::new(
        cal::CalendarObject::event(
            cal::Header::basic("Fred's Birthday Party".into()),
            cal::Time {
                year: 2025,
                month: 0,
                day: 0,
                hour: 1,
                minute: cal::MINUTE_INC,
            }, cal::MINUTE_INC,
        ), &mut generator
    );
    let c2 = UICalO::new(
        cal::CalendarObject::event(
            cal::Header::basic("Jenniffer's Birthday Party".into()),
            cal::Time {
                year: 2025,
                month: 0,
                day: 1,
                hour: 2,
                minute: cal::MINUTE_INC,
            }, cal::MINUTE_INC * 2,
        ), &mut generator
    );
    let c3 = UICalO::new(
        cal::CalendarObject::event(
            cal::Header::basic("Last's Birthday Party".into()),
            cal::Time {
                year: 2025,
                month: 0,
                day: 2,
                hour: 0,
                minute: cal::MINUTE_INC,
            }, cal::MINUTE_INC * 4,
        ), &mut generator
    );

    let t1 = UICalO::new(
        cal::CalendarObject::task(cal::Header::basic("Simple Task A".into())),
        &mut generator
        );
    let t2 = UICalO::new(
        cal::CalendarObject::task(cal::Header::basic("Simple Task B".into())),
        &mut generator
        );
    let t3 = UICalO::new(
        cal::CalendarObject::task(cal::Header::basic("Simple Task C".into())),
        &mut generator
        );


    Globals {
        uidgen: generator,
        tasks: uitask::UITaskList::new(
            vec![
                t1, t2, t3,
            ], tasks_html),
        calendar: uical::UICalendar::new(
            vec![
                c1, c2, c3,
            ], calendar_html),
        placeholder: ph::Placeholder::new(placeholder_html),
        event_state: event::EventState::default(),
    }
}

