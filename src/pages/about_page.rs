use iced::Task;

use crate::message::AboutMessage;
use crate::views::about_view::AboutView;

/// About page showing application information
#[derive(Default)]
pub struct AboutPage;

impl AboutPage {
    /// Creates a new about page
    pub fn new() -> (Self, Task<AboutMessage>) {
        (Self, Task::none())
    }

    /// Updates the about page state (no state updates needed for this page)
    pub fn update(&mut self, _message: AboutMessage) -> Task<AboutMessage> {
        Task::none()
    }

    /// Renders the about page view
    pub fn view(&self) -> iced::Element<'_, AboutMessage> {
        AboutView::new().view()
    }
}
