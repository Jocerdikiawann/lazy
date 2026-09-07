use crate::{RestClient, enums::message::Message};
use iced::{
    Element, Length, color,
    highlighter::{self, Highlighter},
    widget::{Space, column, container, horizontal_space, row, text, text_editor},
};

pub fn view<'a>(state: &'a RestClient) -> Element<'a, Message> {
    let header = row![
        text("Response").size(16),
        horizontal_space(),
        text(format!("{}", &state.status_text)).size(12),
        Space::with_width(10),
        text(format!("{}", &state.time_text)).size(12),
    ]
    .align_y(iced::Alignment::Center);

    //TODO: Count headers ambil dari response headers
    //TODO:  Cookies ambil dari response
    let tabs = row![
        text("Body").size(14),
        text("Headers 12").size(14),
        text("Cookies 2").size(14),
    ]
    .spacing(20);

    let sub_tabs = row![
        text("Pretty").size(13),
        text("Raw").size(13),
        text("Preview").size(13),
        horizontal_space(),
        text("Search...").size(13),
    ]
    .spacing(15)
    .padding([10, 0]);

    let editor = text_editor(&state.response_text)
        .highlight_with::<Highlighter>(
            highlighter::Settings {
                theme: highlighter::Theme::Base16Mocha,
                token: "json".to_string(),
            },
            |high, _theme| high.to_format(),
        )
        .on_action(Message::ResponseEditorAction);

    container(
        column![header, Space::with_height(10), tabs, sub_tabs, editor]
            .spacing(10)
            .padding(15),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::background(color!(0x121212)))
    .into()
}
