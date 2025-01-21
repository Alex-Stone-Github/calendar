use web_sys::*;
use maud::*;
use wasm_bindgen::prelude::*;
use js_sys::*;
use crate::aabb;
use crate::uitask;
use crate::wapi;
use crate::state;
use crate::ui;

/// Safe Mutable Access to Globals - Not Arc Because JS Event Loop is Single Threaded
type SharedGlobals = std::rc::Rc<std::sync::Mutex<state::Globals>>;
pub struct EventState {
    pub should_move_ph: bool,
    pub offset: aabb::Point,
}
impl Default for EventState {
    fn default() -> Self {
        Self {
            should_move_ph: false,
            offset: aabb::Point{x: 0.0, y: 0.0},
        }
    }
}

/// Register events and pass in global state to callbacks
pub fn register_events(globals: state::Globals) {
    let shared_globals = std::rc::Rc::new(std::sync::Mutex::new(globals));

    // mouse down callback
    {
        let mdowncbref = std::rc::Rc::clone(&shared_globals);
        let mdowncb = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            mdown_func(event, mdowncbref.clone());
        });
        state::get_document().add_event_listener_with_callback("mousedown", 
            mdowncb.as_ref().unchecked_ref()).unwrap();
        mdowncb.forget();
    }
    // mouse move callback
    {
        let mmovecbref = std::rc::Rc::clone(&shared_globals);
        let mmovecb = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            mmove_func(event, mmovecbref.clone());
        });
        state::get_document().add_event_listener_with_callback("mousemove", 
            mmovecb.as_ref().unchecked_ref()).unwrap();
        mmovecb.forget();
    }
    // mouse up callback, from wasm bindgen
    {
        let mupcbref = std::rc::Rc::clone(&shared_globals);
        let mupcb = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            mup_func(event, mupcbref.clone());
        });
        state::get_document().add_event_listener_with_callback("mouseup",
            mupcb.as_ref().unchecked_ref()).unwrap();
        mupcb.forget();
    }
}

/*######################################################################################
 *######################################################################################
 *###############FUNCTIONS ARE LISTED BELOW#############################################
 *######################################################################################
 *######################################################################################
 */


/// Mousedown Callback for document
pub fn mdown_func(event: MouseEvent, shared_globals: SharedGlobals) {
    wapi::log("DOWN");
    let mut globals = shared_globals.lock().unwrap();

    let point = aabb::Point {
        x: event.x() as f64,
        y: event.y() as f64,
    };

    use ui::Draggable;
    if let Some(element) = globals.tasks.start_drag(point) {
        globals.event_state.should_move_ph = true;
        globals.placeholder.content = element.render();
    }
}

/// Mousemove Callback for document
pub fn mmove_func(event: MouseEvent, shared_globals: SharedGlobals) {
    let mut globals = shared_globals.lock().unwrap();
    if globals.event_state.should_move_ph {
        globals.placeholder.x = event.x() as usize - globals.event_state.offset.x as usize;
        globals.placeholder.y = event.y() as usize - globals.event_state.offset.y as usize;
    }
    crate::render_all(&mut globals); // rerender everything
}

/// Mouseup Callback for document
pub fn mup_func(event: MouseEvent, shared_globals: SharedGlobals) {
    wapi::log("UP");
    let mut globals = shared_globals.lock().unwrap();

    use ui::{Draggable, DragEndState};
    globals.tasks.end_drag(DragEndState::Cancelled);
    globals.placeholder.content = html!();
    crate::render_all(&mut globals); // rerender everything
}
