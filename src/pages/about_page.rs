use crate::message::AboutMessage;
use crate::views::about_view::AboutView;

/// About page showing application information
#[derive(Default)]
pub struct AboutPage;

impl AboutPage {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, _message: AboutMessage) {
        // No state to update
    }

    pub fn view(&self) -> iced::Element<'_, AboutMessage> {
        AboutView::new().view()
    }
}
