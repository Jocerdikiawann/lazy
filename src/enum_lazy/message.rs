use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    MethodSelected(crate::enum_lazy::method::HttpMethod),
    SendRequest,
    ResponseReceived(Result<(String, String, String), String>),
    RequestEditorAction(text_editor::Action),
    ResponseEditorAction(text_editor::Action),
}
