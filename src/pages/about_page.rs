use crate::message::Message;
use crate::views::about_view::AboutView;

/// About page — purely informational, no interactive state
#[derive(Default)]
pub struct AboutPage;

impl AboutPage {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        AboutView::new().view()
    }
}
