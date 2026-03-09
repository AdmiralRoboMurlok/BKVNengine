use iced::widget::{text, row, Row, column, Column, button, pick_list};
use crate::gui::{Messages, MyState};
use crate::gui::toolbar_enums::FilesOptions;

fn toolbar(state: &MyState) -> Row<'_, Messages> {
    let options = [
        FilesOptions::NewFile,
        FilesOptions::OpenFile,
        FilesOptions::SaveFile,
    ];

    row![
        pick_list(options, state.file_state, Messages::FileHandler).placeholder("Files"),
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