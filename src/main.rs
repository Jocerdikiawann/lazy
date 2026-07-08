use iced::widget::{
    Space, button, column, container, horizontal_space, pick_list, row, text, text_editor,
    text_input,
};
use iced::{Element, Length, Padding, Task, Theme, color};
use reqwest::{Client, Method as ReqwestMethod};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    const ALL: [HttpMethod; 5] = [
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Delete,
        HttpMethod::Patch,
    ];
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpMethod::Get => "GET",
                HttpMethod::Post => "POST",
                HttpMethod::Put => "PUT",
                HttpMethod::Delete => "DELETE",
                HttpMethod::Patch => "PATCH",
            }
        )
    }
}

struct RestClient {
    url: String,
    method: HttpMethod,
    request_body: text_editor::Content,

    response_text: text_editor::Content,
    status_text: String,
    time_text: String,

    is_loading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    MethodSelected(HttpMethod),
    SendRequest,
    ResponseReceived(Result<(String, String, String), String>),
    RequestEditorAction(text_editor::Action),
    ResponseEditorAction(text_editor::Action),
}

impl RestClient {
    fn new() -> (Self, Task<Message>) {
        let initial_json = "{\n  \"username\": \"developer@example.com\",\n  \"password\": \"correct-horse-battery-staple\",\n  \"device_id\": \"dev-macbook-pro-2024\"\n}";

        (
            Self {
                url: String::from("https://api.example.com/v1/login"),
                method: HttpMethod::Post,
                request_body: text_editor::Content::with_text(initial_json),
                response_text: text_editor::Content::new(),
                status_text: String::from("-"),
                time_text: String::from("-"),
                is_loading: false,
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

    fn view_navbar(&self) -> Element<'_, Message> {
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

    fn view_sidebar(&self) -> Element<'_, Message> {
        let header = row![
            text("Collections").size(16),
            horizontal_space(),
            text("+").size(16)
        ]
        .align_y(iced::Alignment::Center);

        let search_bar = text_input("Filter collections...", "").padding(8);

        let folder_tree = column![
            text("Auth API").size(14),
            row![
                Space::with_width(20),
                text("POST  Login").size(13).color(color!(0xFFA500))
            ],
            row![
                Space::with_width(20),
                text("GET   Profile").size(13).color(color!(0x4169E1))
            ],
            row![
                Space::with_width(20),
                text("POST  Refresh Token").size(13).color(color!(0xFFA500))
            ],
            Space::with_height(10),
            text("User Management").size(14),
        ]
        .spacing(8);

        let footer =
            column![text("Trash").size(13), text("Collection Settings").size(13)].spacing(10);

        container(
            column![header, search_bar, folder_tree, horizontal_space(), footer]
                .spacing(15)
                .padding(15),
        )
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(|_theme| container::background(color!(0x16161E)))
        .into()
    }

    fn view_request_pane(&self) -> Element<'_, Message> {
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

        let editor = text_editor(&self.request_body).on_action(Message::RequestEditorAction);

        container(column![tabs, sub_tabs, editor].spacing(10).padding(15))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_response_pane(&self) -> Element<'_, Message> {
        let header = row![
            text("Response").size(16),
            horizontal_space(),
            text(format!("{}", self.status_text)).size(12),
            Space::with_width(10),
            text(format!("{}", self.time_text)).size(12),
        ]
        .align_y(iced::Alignment::Center);

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

        let editor = text_editor(&self.response_text).on_action(Message::ResponseEditorAction);

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

    fn view_workspace(&self) -> Element<'_, Message> {
        let top_bar = row![
            text("Auth API > Login").size(12),
            horizontal_space(),
            text("⏱ 2 mins ago").size(12)
        ]
        .padding([10, 15]);

        let method_picker = pick_list(
            &HttpMethod::ALL[..],
            Some(self.method),
            Message::MethodSelected,
        )
        .width(Length::Fixed(90.0));

        let url_input = text_input("Enter URL...", &self.url)
            .on_input(Message::UrlChanged)
            .padding(8);

        let send_btn = button(if self.is_loading {
            text("Sending...")
        } else {
            text("Send")
        })
        .on_press_maybe(if self.is_loading {
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

        let panes = row![self.view_request_pane(), self.view_response_pane()];

        column![top_bar, url_bar, panes]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            self.view_navbar(),
            row![self.view_sidebar(), self.view_workspace()].height(Length::Fill)
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("Rest Client Native", RestClient::update, RestClient::view)
        .theme(|_| Theme::Dark)
        .run_with(RestClient::new)
}
