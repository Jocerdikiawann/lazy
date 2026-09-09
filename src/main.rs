use iced::Theme;

use crate::ui::rest_client::RestClient;

mod client;
mod db;
mod enums;
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
