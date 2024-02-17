use ratatui::{style::Style, widgets::Block};
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct AddDownload {
    pub showing: bool,
    pub textarea: TextArea<'static>,
}

impl AddDownload {
    pub fn open(&mut self) {
        self.showing = true;
    }

    pub fn close(&mut self) {
        self.showing = false
    }
}

impl Default for AddDownload {
    fn default() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("https://example.org/file.txt");
        textarea.set_block(Block::bordered().title("Add a new download"));

        Self {
            showing: false,
            textarea,
        }
    }
}
