use iced::widget::text_editor::Action;
use iced::widget::{button, column, row, text, text_editor, text_input};
use iced::{Element, Task, Theme};
use reqwest::Client;

struct RestClient {
    url: String,
    response_text: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    SendRequest,
    ResponseReceived(Result<String, String>),
    EditorAction(text_editor::Action),
}

impl RestClient {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                url: String::from("https://httpbin.org/get"),
                response_text: text_editor::Content::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlChanged(new_url) => {
                self.url = new_url;
                Task::none()
            }
            Message::SendRequest => {
                let url = self.url.clone();
                let client = Client::builder()
                    .danger_accept_invalid_hostnames(true)
                    .danger_accept_invalid_certs(true)
                    .build()
                    .unwrap();
                println!("Try sending request");
                Task::perform(
                    async move {
                        match client.get(&url).send().await {
                            Ok(res) => match res.text().await {
                                Ok(text) => {
                                    println!("got a response {}", text);
                                    Ok(text)
                                }
                                Err(e) => Err(e.to_string()),
                            },
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    Message::ResponseReceived,
                )
            }
            Message::ResponseReceived(result) => {
                let content = match result {
                    Ok(text) => text,
                    Err(err) => format!("Error: {}", err),
                };
                self.response_text = text_editor::Content::with_text(&content);
                Task::none()
            }
            Message::EditorAction(action) => {
                if action.is_edit() {
                    Task::none()
                } else {
                    self.response_text.perform(action);
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            text_input("Masukkan URL API...", &self.url)
                .on_input(Message::UrlChanged)
                .padding(10),
            button("Send").on_press(Message::SendRequest).padding(10),
        ]
        .spacing(10);

        column![
            text("Rest Client Native").size(20),
            controls,
            text("Response:"),
            text_editor(&self.response_text).on_action(Message::EditorAction)
        ]
        .spacing(15)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Rest Client Native", RestClient::update, RestClient::view)
        .theme(|_| Theme::Dark)
        .run_with(RestClient::new)
}
