use iced::{Task, widget::text_editor};

use iced::widget::{column, container, row};

use crate::enums::message::Message;
use crate::enums::method::HttpMethod;
use crate::util;
use iced::{Element, Length};
use reqwest::{Client, Method as ReqwestMethod};
use std::str::FromStr;

use super::{navbar, sidebar, workspace};

pub struct RestClient {
    pub url: String,
    pub method: HttpMethod,
    pub request_body: text_editor::Content,

    pub response_text: text_editor::Content,
    pub status_text: String,
    pub time_text: String,

    pub is_loading: bool,
}

impl RestClient {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                url: String::from("https://jsonplaceholder.typicode.com/posts"),
                method: HttpMethod::Post,
                request_body: text_editor::Content::with_text("{}"),
                response_text: text_editor::Content::new(),
                status_text: String::from("-"),
                time_text: String::from("-"),
                is_loading: false,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UrlChanged(new_url) => {
                self.url = new_url;
                Task::none()
            }
            Message::MethodSelected(method) => {
                self.method = method;
                Task::none()
            }
            Message::SendRequest => {
                self.is_loading = true;
                self.status_text = String::from("Loading...");
                self.time_text = String::from("-");
                self.response_text = text_editor::Content::new();

                let url = self.url.clone();
                let method =
                    ReqwestMethod::from_str(&self.method.to_string()).unwrap_or(ReqwestMethod::GET);
                let body_text = self.request_body.text();

                let client = Client::builder()
                    .danger_accept_invalid_hostnames(true)
                    .danger_accept_invalid_certs(true)
                    .build()
                    .unwrap();

                Task::perform(
                    async move {
                        let start_time = std::time::Instant::now();
                        let request = client.request(method, &url);

                        let request = if !body_text.is_empty() {
                            request
                                .header("content-type", "application/json; charset=UTF-8")
                                .body(body_text)
                        } else {
                            request
                        };

                        match request.send().await {
                            Ok(res) => {
                                let status = res.status().to_string();
                                let time = format!("{}ms", start_time.elapsed().as_millis());
                                match res.text().await {
                                    Ok(text) => Ok((text, status, time)),
                                    Err(e) => Err(e.to_string()),
                                }
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    Message::ResponseReceived,
                )
            }
            Message::ResponseReceived(result) => {
                self.is_loading = false;
                match result {
                    Ok((body, status, time)) => {
                        let body = util::make_json_pretty(&body);
                        self.response_text = text_editor::Content::with_text(&body);
                        self.status_text = status;
                        self.time_text = time;
                    }
                    Err(err) => {
                        self.response_text =
                            text_editor::Content::with_text(&format!("Error: {}", err));
                        self.status_text = String::from("Error");
                    }
                };
                Task::none()
            }
            Message::RequestEditorAction(action) => {
                //TODO: LOGIC disini
                self.request_body.perform(action);
                Task::none()
            }
            Message::ResponseEditorAction(action) => {
                if action.is_edit() {
                    Task::none()
                } else {
                    self.response_text.perform(action);
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            navbar::view(),
            row![sidebar::view(), workspace::view(self)].height(Length::Fill)
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
