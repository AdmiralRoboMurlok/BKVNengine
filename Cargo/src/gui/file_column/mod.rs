use iced::widget::{text, row, Row, column, Column, button, pick_list, radio};
use crate::gui::{Messages, MyState};
use crate::gui::toolbar_enums::{EditOptions, FilesOptions, ViewOptions, HelpOptions, FilterOptions};

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

    let help_options = [
        HelpOptions::Help,
    ];

    row![
        pick_list(file_options, state.file_state, Messages::FileHandler).placeholder("Files"),
        pick_list(edit_options, state.edit_state, Messages::EditHandler).placeholder("Edit"),
        pick_list(view_options, state.view_state, Messages::ViewHandler).placeholder("View"),
        pick_list(help_options, state.help_state, Messages::HelpHandler).placeholder("Help"),
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
    let image = radio(
        "Images",
        FilterOptions::Image,
        state.filter_selection_state,
        Messages::FilterHandler
    );

    let background = radio(
        "Background",
        FilterOptions::Background,
        state.filter_selection_state,
        Messages::FilterHandler
    );

    let sound = radio(
        "Sound",
        FilterOptions::Sound,
        state.filter_selection_state,
        Messages::FilterHandler
    );

    let all = radio(
        "All",
        FilterOptions::All,
        state.filter_selection_state,
        Messages::FilterHandler
    );

    row![
        image,
        background,
        sound,
        all
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