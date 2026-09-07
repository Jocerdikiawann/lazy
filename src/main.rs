use iced::Theme;
use log::LevelFilter;

use crate::ui::rest_client::RestClient;

mod client;
mod db;
mod enums;
mod ui;
mod util;

fn main() -> iced::Result {
    log::set_max_level(LevelFilter::max());

    iced::application("Lazy", RestClient::update, RestClient::view)
        .theme(|_| Theme::Dark)
        .run_with(RestClient::new)
}
