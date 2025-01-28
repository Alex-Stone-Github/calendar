use maud::Markup;
use crate::uicalo::UICalO;
use crate::aabb::*;

/// This is a ui element in the calendar. It contains UICalOs
pub trait UIElement {
    fn render(&self) -> Markup;
    fn draw(&self);
}

/// These are the types of drag states
pub enum DragEndState {
    Cancelled,
    Accepted,
}
/// Every UI Element can implement this trait to allow drag support
pub trait Draggable {
    /// Called at the start of the drag
    fn start_drag(&mut self, start: Point) -> Option<(UICalO, Point)>;
    /// Called at the end of a mouse drag
    fn end_drag(&mut self, condition: DragEndState);
}

/// Droppable
pub trait Droppable {
    /// Returns whether the spot is an acceptable drop site
    fn preview(&mut self, point: Point, element: UICalO) -> bool;
    /// Solidify the preview as the desired drop location
    fn drop(&mut self); // Solidify a preview
}


// TODO: Work needs to be done
pub trait Slidable {
    fn start_slide(point: Point) -> bool;
    fn preview_slide(point: Point) -> bool;
    fn end_slide(point: Point) -> bool;
}
