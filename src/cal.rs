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
#[derive(Hash)]
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
/// This struct represents every possible calendar task, event, whatever
pub enum CalendarObject {
    /// header, potential duration
    Task(Header, Option<Minutes>),
    /// header, scheduled time, duration
    ScheduledTask(Header, Time, Minutes),
    /// header, time, duration
    Event(Header, Time, Minutes),
    /// header, time, duration
    Frame(Header, Time, Minutes),
}
