use maud::*;
use web_sys::*;
use crate::uicalo::UICalO;
use crate::aabb;

type TLocation = usize;
type TIdentifier = usize;

pub struct UITaskList {
    pub tasks: Vec<UICalO>,
    pub ghost_remove: Option<TIdentifier>,
    pub ghost_add: Option<(UICalO, TLocation)>,
}
impl UITaskList {
    pub fn draggable_areas(&self) -> Vec<(aabb::Bounds, TLocation)> {
        self.tasks.iter().enumerate()
            .filter(|(i, _)| {
                *i as TIdentifier != self.ghost_remove.unwrap_or(TLocation::MAX)
            })
            .map(|(i, x)| (x.get_bounds(), i as usize))
            .collect()
    }
    pub fn tmp_remove(&mut self, identifier: Option<TIdentifier>) {
        self.ghost_remove = identifier;
    }
    pub fn tmp_add(&mut self, element: Option<(UICalO, TLocation)>) {
        self.ghost_add = element;
    }
    pub fn render(&self) -> Markup {
        let mut task_cpy = self.tasks.clone();
        let mut add_idx = TLocation::MAX;
        if let Some(ghost_ref) = self.ghost_add.as_ref() {
            task_cpy.insert(ghost_ref.1, ghost_ref.0.clone());
            add_idx = ghost_ref.1;
        }
        if let Some(toremove_idx) = self.ghost_remove {
            if add_idx <= toremove_idx {
                /* There is a new element inserted at this position or before so the actual element
                 * we want to delete is shifted to the down the array. Add one to account for this
                 * offset if that is the case */
                task_cpy.remove(toremove_idx + 1);
            }
            else {
                task_cpy.remove(toremove_idx );
            }
        }

        task_cpy.iter().enumerate()
            .map(|(_, t)| t.render())
            .reduce(|a, b| {
            html! {
                (a)
                (b)
            }
        }).unwrap()
    }
}
