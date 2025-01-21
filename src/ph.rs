use maud::*;

pub struct Placeholder {
    pub x: usize,
    pub y: usize,
    pub content: Markup,
}

impl Placeholder {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            content: html!(),
        }
    }
    pub fn render(&self) -> Markup {
        let css = format!("
            left: {}px;
            top:  {}px;
        ", self.x, self.y);
        html! {
            .placeholder style=(css.as_str()) {
                (self.content)
            }
        }
    }
}
