use iced::Theme;

use crate::ui::rest_client::RestClient;

mod client;
mod db;
mod enums;
mod ui;
mod util;

fn main() -> iced::Result {
    iced::application("Rest Client Native", RestClient::update, RestClient::view)
        .theme(|_| Theme::Dark)
        .run_with(RestClient::new)
}
