use iced::widget::{button, row, Rule};
use iced::{widget, Alignment, Length};

use crate::message::Message;
use crate::route::Route;

/// Navigation bar component
pub struct Navigation {
    current_route: Route,
}

impl Navigation {
    /// Creates a new navigation component
    pub fn new(current_route: Route) -> Self {
        Self { current_route }
    }

    /// Renders the navigation bar
    pub fn view(self) -> iced::Element<'static, Message> {
        let nav_buttons = Route::all()
            .into_iter()
            .map(|route| {
                let is_active = route == self.current_route;
                let button_style = if is_active {
                    button(route.display_name()).style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color([0.2, 0.4, 0.8, 1.0].into())),
                        text_color: [1.0, 1.0, 1.0, 1.0].into(),
                        border: iced::Border {
                            color: [0.2, 0.4, 0.8, 1.0].into(),
                            width: 2.0,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                    })
                } else {
                    button(route.display_name()).style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color([0.9, 0.9, 0.9, 1.0].into())),
                        text_color: [0.2, 0.2, 0.2, 1.0].into(),
                        border: iced::Border {
                            color: [0.7, 0.7, 0.7, 1.0].into(),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                    })
                };

                let button = if is_active {
                    button_style
                } else {
                    button_style.on_press(Message::Navigate(route))
                };

                button.into()
            })
            .collect::<Vec<_>>();

        widget::column![
            widget::container(
                row(nav_buttons)
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .padding(10)
            )
            .width(Length::Fill)
            .style(|_theme| {
                widget::container::Style {
                    background: Some(iced::Background::Color([0.95, 0.95, 0.95, 1.0].into())),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                    text_color: None,
                }
            }),
            Rule::horizontal(1)
        ]
        .into()
    }
}
