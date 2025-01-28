use std::hash::Hash;

pub type Minutes = u32;
pub const MINUTE_INC: Minutes = 15;

pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
];
/// This represents a real life "time" on the clock, we use 24 hour time
#[derive(Hash, Clone, Copy)]
pub struct Time {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: Minutes,
}


/// Every "calendar object" must have some basic information in the header
#[derive(Clone)]
pub struct Header {
    pub name: String,
    pub description: String,
}
impl Header {
    pub fn basic(name: String) -> Self {
        Self {
            name,
            description: "".into(),
        }
    }
}
/// This struct represents every possible calendar task, event, whatever
pub struct CalendarObject {
    /// header, potential duration
    pub header: Header,
    pub time: Option<Time>,
    pub duration: Option<Minutes>,
}
impl CalendarObject {
    pub fn task(header: Header) -> Self {
        Self {
            header,
            time: None,
            duration: None,
        }
    }
    pub fn event(header: Header, time: Time, duration: Minutes) -> Self {
        Self {
            header, time: Some(time), duration: Some(duration),
        }
    }
}
