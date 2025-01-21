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

fn render_all(globals: &mut state::Globals) {
    use ui::UIElement;
    globals.tasks_html.set_inner_html(
        globals.tasks.render().into_string().as_str()
    );
    globals.calendar_html.set_inner_html(
        globals.calendar.render().into_string().as_str()
    );
    globals.placeholder_html.set_inner_html(
        globals.placeholder.render().into_string().as_str()
    );
}

#[wasm_bindgen]
pub fn main() {
    let mut globals = state::setup();
    globals.placeholder.content = html! {
        h1 {"what on earth is this"}
    };
    globals.placeholder.x = 100;
    globals.placeholder.y = 100;
    render_all(&mut globals);
    event::register_events(globals); // move -> drop(globals)
}
