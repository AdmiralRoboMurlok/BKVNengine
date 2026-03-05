use iced::widget::{text, row, Row};
use iced::widget::text::State;
use crate::gui::{Messages, MyState};

fn toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        text("Hi hello, from column"),
    ]
}

pub fn file_column(state: &MyState) -> Row<'_, Messages> {
    toolbar(state)
}