use iced::Theme;

use crate::ui::rest_client::RestClient;

mod db;
mod domain;
mod enums;
mod infra;
mod logger;
mod ui;
mod util;

fn main() -> iced::Result {
    let _guard = logger::init_logger();

    tracing::info!("Starting App Lazy..");

    iced::application("Lazy", RestClient::update, RestClient::view)
        .theme(|_| Theme::Dark)
        .run_with(RestClient::new)
}
