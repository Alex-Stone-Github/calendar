use wasm_bindgen::prelude::*;
use web_sys::*;
use maud::*;

mod ui;
mod cal;
mod event;
mod uical;
mod uitask;
mod uicalo;
mod wapi;
mod uid;
mod aabb;
mod state;
mod ph;

fn render_all(globals: &state::Globals) {
    use ui::UIElement;
    globals.tasks.draw();
    globals.calendar.draw();
    globals.placeholder.draw();
}

#[wasm_bindgen]
pub fn main() {
    let mut globals = state::setup();
    render_all(&mut globals);
    
    let db = globals.calendar.get_bounds();
    wapi::log(format!("{}, {}, {}, {}", db.x, db.y, db.w, db.h).as_str());
    event::register_events(globals); // move -> drop(globals)
}
