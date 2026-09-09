use crate::enums::message::Message;
use iced::{
    Element, color, widget::Space, widget::button, widget::container, widget::horizontal_space,
    widget::row, widget::text,
};

pub fn view() -> Element<'static, Message> {
    let logo = text("Lazy").size(24);
    let tabs = row![
        button(text("Collections")).padding([5, 10]),
        button(text("History")).padding([5, 10]),
        button(text("Environments")).padding([5, 10]),
    ]
    .spacing(15);

    let settings_icon = text("icon_setting");

    container(
        row![
            logo,
            Space::with_width(30),
            tabs,
            horizontal_space(),
            settings_icon
        ]
        .align_y(iced::Alignment::Center)
        .padding([10, 20]),
    )
    .style(|_theme| container::background(color!(0x1A1B26)))
    .into()
}
