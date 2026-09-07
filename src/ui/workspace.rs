use crate::RestClient;
use crate::enums::message::Message;
use crate::enums::method::HttpMethod;

use super::{request, response};

use iced::{
    Element, Length, Padding,
    widget::{button, column, horizontal_space, pick_list, row, text, text_input},
};

pub fn view_workspace(state: &'static RestClient) -> Element<'static, Message> {
    //TODO: Workspace
    let top_bar = row![
        text("Auth API > Login").size(12),
        horizontal_space(),
        text("⏱ 2 mins ago").size(12)
    ]
    .padding([10, 15]);

    let method_picker = pick_list(
        &HttpMethod::ALL[..],
        Some(&state.method),
        Message::MethodSelected,
    )
    .width(Length::Fixed(90.0));

    let url_input = text_input("Enter URL...", &state.url)
        .on_input(Message::UrlChanged)
        .padding(8);

    let send_btn = button(if state.is_loading {
        text("Sending...")
    } else {
        text("Send")
    })
    .on_press_maybe(if state.is_loading {
        None
    } else {
        Some(Message::SendRequest)
    })
    .padding([8, 20]);
    let url_bar = row![method_picker, url_input, send_btn]
        .spacing(10)
        .padding(Padding {
            top: 0.0,
            right: 15.0,
            bottom: 10.0,
            left: 15.0,
        })
        .align_y(iced::Alignment::Center);

    let panes = row![request::view(state), response::view(state)];

    column![top_bar, url_bar, panes]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
