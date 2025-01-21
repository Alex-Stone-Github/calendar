use maud::*;
use web_sys::*;
use crate::uicalo::UICalO;
use crate::aabb;
use crate::ui;

/// Responsible for all ui related state and tasks
pub struct UITaskList {
    pub tasks: Vec<UICalO>,
    pub ghost_remove: Option<usize>,
    pub ghost_add: Option<(UICalO, usize)>,
}
/// Core Features
impl ui::UIElement for UITaskList {
    fn render(&self) -> Markup {
        let mut to_render = self.tasks.clone();
        let mut rmidx = usize::MAX;
        if let Some(idx) = self.ghost_remove {
            to_render.remove(idx);
            rmidx = idx;
        }
        if let Some(toinsert) = self.ghost_add.clone() {
            let mut toinsert_idx = toinsert.1;
            if rmidx < toinsert.1 {
                toinsert_idx -= 1;
            }
            to_render.insert(toinsert_idx, toinsert.0);
        }
        to_render.into_iter().map(|x| x.render()).reduce(|a, b| html!{(a)(b)}).unwrap()
    }
}


/// Dragging Features
impl ui::Draggable for UITaskList {
    fn start_drag(&mut self, start: aabb::Point) -> Option<UICalO> {
        for (index, bound) in self.tasks.iter()
            .map(|x| x.get_bounds()).enumerate() {
            let collides = aabb::intersects(&bound, &start);
            if collides {
                self.ghost_remove = Some(index);
                return Some(self.tasks[index].clone());
            }
        }
        None
    }
    fn end_drag(&mut self, condition: ui::DragEndState) {
        match condition {
            ui::DragEndState::Cancelled => (),
            ui::DragEndState::Accepted => {
                if let Some(idx) = self.ghost_remove {
                    self.tasks.remove(idx);
                }
            },
        };
        self.ghost_remove = None;
    }
}
