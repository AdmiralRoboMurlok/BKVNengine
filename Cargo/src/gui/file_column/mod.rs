use iced::widget::{text, row, Row, column, Column, button};
use crate::gui::{Messages, MyState};


fn toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        text("Scene files"),
        button("New character").on_press(Messages::ImportCharacter),
        button("New background").on_press(Messages::ImportBackground),
        button("New music").on_press(Messages::ImportSound),
    ]
}

pub fn file_column(state: &MyState) -> Column<'_, Messages> {
    column![
        toolbar(&state)
    ]
}