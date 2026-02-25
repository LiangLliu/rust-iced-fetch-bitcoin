use std::time::Duration;

use iced::{widget, Subscription, Task, Theme};

use crate::message::{BitcoinMessage, Message};
use crate::pages::{about_page::AboutPage, bitcoin_page::BitcoinPage, settings_page::SettingsPage};
use crate::route::Route;
use crate::views::navigation::Navigation;

/// Main application state
pub struct App {
    /// Current active route
    current_route: Route,
    /// Bitcoin page state
    bitcoin_page: BitcoinPage,
    /// Settings page state — owns config that affects the whole app
    settings_page: SettingsPage,
    /// About page state
    about_page: AboutPage,
}

impl App {
    // ── Lifecycle ────────────────────────────────────────────────

    /// Boot function called once at startup (iced 0.14 `BootFn`)
    pub fn boot() -> (Self, Task<Message>) {
        let (bitcoin_page, bitcoin_task) = BitcoinPage::new();
        let settings_page = SettingsPage::new();
        let about_page = AboutPage::new();

        (
            Self {
                current_route: Route::default(),
                bitcoin_page,
                settings_page,
                about_page,
            },
            bitcoin_task.map(Message::Bitcoin),
        )
    }

    /// Dynamic window title based on current page
    pub fn title(&self) -> String {
        format!("Bitcoin Price Monitor — {}", self.current_route.display_name())
    }

    /// Theme is driven by the Settings page selection
    pub fn theme(&self) -> Theme {
        self.settings_page.selected_theme().clone()
    }

    /// Subscription: auto-refresh BTC prices at the configured interval
    pub fn subscription(&self) -> Subscription<Message> {
        if self.settings_page.auto_refresh_enabled() {
            let secs = self.settings_page.auto_refresh_interval();
            iced::time::every(Duration::from_secs(secs as u64)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    // ── Update ──────────────────────────────────────────────────

    /// Handles application-level messages and delegates to appropriate page
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(route) => {
                self.current_route = route;
                Task::none()
            }
            Message::Bitcoin(msg) => {
                self.bitcoin_page.update(msg).map(Message::Bitcoin)
            }
            Message::Settings(msg) => {
                self.settings_page.update(msg);
                Task::none()
            }
            Message::Tick => {
                // Auto-refresh triggers a Bitcoin price refetch
                self.bitcoin_page
                    .update(BitcoinMessage::Refetch)
                    .map(Message::Bitcoin)
            }
        }
    }

    // ── View ────────────────────────────────────────────────────

    /// Renders the application view
    pub fn view(&self) -> iced::Element<'_, Message> {
        let navigation = Navigation::new(&self.current_route).view();

        let content = match &self.current_route {
            Route::Bitcoin => self.bitcoin_page.view().map(Message::Bitcoin),
            Route::Settings => self.settings_page.view().map(Message::Settings),
            Route::About => self.about_page.view(),
        };

        widget::column![navigation, content].into()
    }
}
