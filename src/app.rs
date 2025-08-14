use iced::{widget, Task, Theme};

use crate::message::Message;
use crate::pages::{about_page::AboutPage, bitcoin_page::BitcoinPage, settings_page::SettingsPage};
use crate::route::Route;
use crate::views::navigation::Navigation;

/// Main application state
pub struct App {
    /// Current active route
    current_route: Route,
    /// Bitcoin page state
    bitcoin_page: BitcoinPage,
    /// Settings page state
    settings_page: SettingsPage,
    /// About page state
    about_page: AboutPage,
}

impl Default for App {
    fn default() -> Self {
        let (bitcoin_page, _) = BitcoinPage::new();
        let (settings_page, _) = SettingsPage::new();
        let (about_page, _) = AboutPage::new();
        
        Self {
            current_route: Route::default(),
            bitcoin_page,
            settings_page,
            about_page,
        }
    }
}

impl App {
    /// Creates a new instance of the application
    pub fn new() -> (Self, Task<Message>) {
        let (bitcoin_page, bitcoin_task) = BitcoinPage::new();
        let (settings_page, settings_task) = SettingsPage::new();
        let (about_page, about_task) = AboutPage::new();
        
        (
            Self {
                current_route: Route::default(),
                bitcoin_page,
                settings_page,
                about_page,
            },
            Task::batch([
                bitcoin_task.map(Message::Bitcoin),
                settings_task.map(Message::Settings),
                about_task.map(Message::About),
            ]),
        )
    }

    /// Handles application-level messages and delegates to appropriate handlers
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.current_route = route;
                Task::none()
            }
            Message::Bitcoin(bitcoin_msg) => {
                self.bitcoin_page
                    .update(bitcoin_msg)
                    .map(Message::Bitcoin)
            }
            Message::Settings(settings_msg) => {
                self.settings_page
                    .update(settings_msg)
                    .map(Message::Settings)
            }
            Message::About(about_msg) => {
                self.about_page
                    .update(about_msg)
                    .map(Message::About)
            }
        }
    }

    /// Renders the application view
    pub fn view(&self) -> iced::Element<'_, Message> {
        let navigation = Navigation::new(self.current_route.clone()).view();
        
        let content = match &self.current_route {
            Route::Bitcoin => self.bitcoin_page.view().map(Message::Bitcoin),
            Route::Settings => self.settings_page.view().map(Message::Settings),
            Route::About => self.about_page.view().map(Message::About),
        };

        widget::column![navigation, content].into()
    }

    /// Returns the application theme
    pub fn theme(&self) -> Theme {
        // In the future, this could be dynamic based on settings
        Theme::Nord
    }
}
