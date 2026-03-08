use iced::widget::{text, row, Row, column, Column, button};
use crate::gui::{Messages, MyState};

fn scene_tools(state: &MyState) -> Row<'_, Messages> {
    row![
        button("Add scene").on_press(Messages::PlaceholderMsg),
    ]
}

pub fn scene_bar(state: &MyState) -> Column<'_, Messages> {
    column![
        scene_tools(&state),
    ]
}