mod client;
mod db;
mod enum_lazy;
mod ui;
mod util;

use iced::highlighter::Highlighter;
use iced::widget::{
    Space, button, column, container, horizontal_space, pick_list, row, text, text_editor,
    text_input,
};

use crate::enum_lazy::method::HttpMethod;
use iced::{Element, Length, Padding, Task, Theme, color, highlighter};
use reqwest::{Client, Method as ReqwestMethod};
use std::str::FromStr;

struct RestClient {
    url: String,
    method: HttpMethod,
    request_body: text_editor::Content,

    response_text: text_editor::Content,
    status_text: String,
    time_text: String,

    is_loading: bool,
}

impl RestClient {
    fn new() -> (Self, Task<Message>) {
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

    fn view_request_pane(&self) -> Element<'_, Message> {
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

        let editor = text_editor(&self.response_text)
            .on_action(Message::ResponseEditorAction)
            .highlight_with::<Highlighter>(
                highlighter::Settings {
                    theme: highlighter::Theme::Base16Mocha,
                    token: "json".to_string(),
                },
                |high, _theme| high.to_format(),
            );

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
        //TODO: Workspace
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
