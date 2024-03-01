use crate::ui::widget::Widget;

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Text {
            content: content.to_string(),
        }
    }

    pub fn set_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }
}

impl Widget for Text {
    fn render(&self) -> String {
        self.content.clone()
    }
}
