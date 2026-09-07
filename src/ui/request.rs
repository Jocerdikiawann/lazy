use crate::{RestClient, enums::message::Message};
use iced::{
    Element, Length,
    widget::{column, container, horizontal_space, row, text, text_editor},
};

pub fn view<'a>(state: &'a RestClient) -> Element<'a, Message> {
    //TODO: Header count ambil dari response headers api
    let tabs = row![
        text("Params").size(14),
        text("Auth").size(14),
        text("Headers 4").size(14),
        text("Body").size(14),
    ]
    .spacing(20);

    let sub_tabs = row![
        text("JSON").size(13),
        text("Form").size(13),
        text("Text").size(13),
        text("GraphQL").size(13),
        horizontal_space(),
    ]
    .spacing(15)
    .padding([10, 0]);

    let editor = text_editor(&state.request_body)
        .height(Length::Fill)
        .on_action(Message::RequestEditorAction);

    container(column![tabs, sub_tabs, editor].spacing(10).padding(15))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
