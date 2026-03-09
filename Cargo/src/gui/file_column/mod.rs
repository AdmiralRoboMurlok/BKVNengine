use iced::widget::{text, row, Row, column, Column, button, pick_list};
use crate::gui::{Messages, MyState};


fn toolbar(state: &MyState) -> Row<'_, Messages> {
    row![
        button("Files").on_press(Messages::PlaceholderMsg),
        button("Edit").on_press(Messages::PlaceholderMsg),
        button("View").on_press(Messages::PlaceholderMsg),
        button("Navigate").on_press(Messages::PlaceholderMsg),
        button("Help").on_press(Messages::PlaceholderMsg),
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

fn scene_actors_list(state: &MyState) -> Column<'_, Messages> {
    column![
        
    ]
}

pub fn file_column(state: &MyState) -> Column<'_, Messages> {
    column![
        toolbar(&state),
        scene_toolbar(&state),
        scene_actors_list(&state),
    ]
}