use iced::widget::{text, row, Row, column, Column, button};
use crate::gui::{Messages, MyState};


fn toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        button("Files").on_press(Messages::placeholder_msg),
        button("Edit").on_press(Messages::placeholder_msg),
        button("View").on_press(Messages::placeholder_msg),
        button("Navigate").on_press(Messages::placeholder_msg),
        button("Help").on_press(Messages::placeholder_msg),
    ]
}

fn scene_toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        text("Scene files"),
        button("New character").on_press(Messages::ImportCharacter),
        button("New background").on_press(Messages::ImportBackground),
        button("New music").on_press(Messages::ImportSound),
    ]
}

pub fn file_column(state: &MyState) -> Column<'_, Messages> {
    column![
        toolbar(&state),
        scene_toolbar(&state),
    ]
}