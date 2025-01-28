use maud::*;
use web_sys::*;
use crate::uicalo::UICalO;
use crate::aabb;
use crate::ui;

/// Responsible for all ui related state and tasks
pub struct UITaskList {
    tasks: Vec<UICalO>,
    ghost_remove: Option<usize>,
    ghost_add: Option<(UICalO, usize)>,
    target: Element,
}
impl UITaskList {
    pub fn new(tasks: Vec<UICalO>, target: Element) -> Self {
        Self {
            tasks,
            ghost_remove: None,
            ghost_add: None,
            target,
        }
    }

}
/// Core Features
impl ui::UIElement for UITaskList {
    fn draw(&self) {
        let markup = self.render();
        self.target.set_inner_html(markup.into_string().as_str());
    }
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
        to_render.into_iter()
            .map(|x| x.render_as_task())
            .reduce(|a, b| html!{(a)(b)})
            .unwrap()
    }
}


/// Dragging Features
impl ui::Draggable for UITaskList {
    /// This is called when a drag is started
    fn start_drag(&mut self, start: aabb::Point) -> Option<(UICalO, aabb::Point)> {
        for (index, bound) in self.tasks.iter()
            .map(|x| x.get_bounds()).enumerate() {
            let collides = aabb::intersects(&bound, &start);
            if collides {
                self.ghost_remove = Some(index);
                let pt = aabb::Point{x:0.0,y:0.0};
                return Some((self.tasks[index].clone(), pt));
            }
        }
        None
    }
    /// This is called when a drag is ended
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
