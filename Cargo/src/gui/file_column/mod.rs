use iced::Element;
use iced::widget::text;
use crate::gui::{Messages, MyState};

pub fn file_column(state: &MyState) -> Element<'_, Messages> {
    text("Hi hello, from column").into()
}