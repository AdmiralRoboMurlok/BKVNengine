use iced::widget::{text, row, Row, column, Column, button, pick_list};
use crate::gui::{Messages, MyState};
use crate::gui::toolbar_enums::{EditOptions, FilesOptions, ViewOptions};

fn toolbar(state: &MyState) -> Row<'_, Messages> {
    let file_options = [
        FilesOptions::NewFile,
        FilesOptions::OpenFile,
        FilesOptions::SaveFile,
    ];
    
    let edit_options = [
        EditOptions::Copy,
        EditOptions::Paste,
        EditOptions::Delete,
    ];
    
    let view_options = [
        ViewOptions::NotKnownYet,  
    ];

    row![
        pick_list(file_options, state.file_state, Messages::FileHandler).placeholder("Files"),
        pick_list(edit_options, state.edit_state, Messages::EditHandler).placeholder("Edit"),
        pick_list(view_options, state.view_state, Messages::ViewHandler).placeholder("View"),
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

fn filter_buttons(state: &MyState) -> Row<'_, Messages> {
    row![
        
    ]
}

fn scene_actors_list(state: &MyState) -> Column<'_, Messages> {
    column![
        filter_buttons(state),
    ]
}

pub fn file_column(state: &MyState) -> Column<'_, Messages> {
    column![
        toolbar(&state),
        scene_toolbar(&state),
        scene_actors_list(&state),
    ]
}