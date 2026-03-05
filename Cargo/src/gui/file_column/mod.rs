use iced::widget::{text, row, Row, column, Column};
use crate::gui::{Messages, MyState};

fn toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        text("Hi hello, from column"),
    ]
}

pub fn file_column(state: &MyState) -> Column<'_, Messages> {
    column![
        toolbar(&state)
    ]
}