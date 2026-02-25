use iced::widget::{button, container, row, rule};
use iced::{widget, Center, Fill};

use crate::message::Message;
use crate::route::Route;

/// Navigation bar component
pub struct Navigation<'a> {
    current_route: &'a Route,
}

impl<'a> Navigation<'a> {
    pub fn new(current_route: &'a Route) -> Self {
        Self { current_route }
    }

    /// Renders the navigation bar using built-in iced 0.14 button styles
    pub fn view(self) -> iced::Element<'static, Message> {
        let nav_buttons: Vec<iced::Element<'static, Message>> = Route::all()
            .into_iter()
            .map(|route| {
                let is_active = route == *self.current_route;
                let label = route.display_name();

                let btn = if is_active {
                    button(label).style(button::primary)
                } else {
                    button(label)
                        .style(button::secondary)
                        .on_press(Message::Navigate(route))
                };

                btn.into()
            })
            .collect();

        widget::column![
            container(
                row(nav_buttons)
                    .spacing(10)
                    .align_y(Center)
                    .padding(10)
            )
            .width(Fill)
            .style(container::rounded_box),
            rule::horizontal(1)
        ]
        .into()
    }
}
