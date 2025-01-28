use web_sys::*;
use maud::*;
use crate::ui;
use crate::aabb;
use crate::uicalo::UICalO;

pub struct Placeholder {
    position: aabb::Point,
    pub content: Option<UICalO>,
    target: Element,
}

impl Placeholder {
    pub fn new(target: Element) -> Self {
        Self {
            position: aabb::Point{x:0.0,y:0.0},
            content: None,
            target,
        }
    }
    pub fn set_content(&mut self, content: Option<UICalO>) {
        self.content = content;
    }
    pub fn get_content_unchecked(&self) -> UICalO {
        self.content.as_ref().unwrap().clone()
    }
    pub fn set_position(&mut self, new_pos: aabb::Point) {
        self.position = new_pos;
    }
}
impl ui::UIElement for Placeholder {
    fn draw(&self) {
        let markup = self.render();
        self.target.set_inner_html(markup.into_string().as_str());
    }
    fn render(&self) -> Markup {
        let css = format!("
            left: {}px;
            top:  {}px;
        ", self.position.x, self.position.y);
        match self.content.clone() {
            Some(content) => html! {
                .placeholder style=(css.as_str()) {
                    (content.render_as_task())
                }
            },
            None => html!(),
        }
    }
}
