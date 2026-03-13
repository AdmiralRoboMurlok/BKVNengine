mod file_column;
pub mod scene_manager;
pub mod scene_builder;
pub mod file_column_msg_handler;
pub mod toolbar_enums;

use iced::widget::{row, Column, Row, column};
use crate::gui::file_column::file_column;
use crate::gui::scene_manager::scene_bar;
use crate::gui::scene_builder::scene_view;
use crate::gui::file_column_msg_handler::*;
use crate::gui::toolbar_enums::*;
use crate::placeholder;

#[derive(Default)]
struct MyState {
    current_state: u64,
    file_state: Option<FilesOptions>,
    edit_state: Option<EditOptions>,
    view_state: Option<ViewOptions>,
    help_state: Option<HelpOptions>,
}

#[derive(Debug, Clone)]
pub enum Messages {
    Exit,
    ImportCharacter,
    ImportBackground,
    ImportSound,
    FileHandler(FilesOptions),
    EditHandler(EditOptions),
    ViewHandler(ViewOptions),
    HelpHandler(HelpOptions),
    PlaceholderMsg,
}

fn update(state: &mut MyState, message: Messages) {
    match message {
        Messages::Exit => placeholder(),
        Messages::ImportCharacter => import_character(),
        Messages::ImportBackground => import_background(),
        Messages::ImportSound => import_sound(),
        Messages::FileHandler(option) => toolbar_file_handler(&option),
        Messages::EditHandler(option) => toolbar_edit_handler(&option),
        Messages::ViewHandler(option) => toolbar_view_handler(&option),
        Messages::HelpHandler(option) => toolbar_help_handler(&option),
        Messages::PlaceholderMsg => placeholder()
    }
}

fn scene(state: &MyState) -> Column<'_, Messages> {
    column![
        scene_view(state),
        scene_bar(state),
    ]
}

fn view(state: &MyState) -> Row<'_, Messages> {
    row![
        file_column(state),
        scene(state)
    ]
}

fn new() -> MyState {
    let states = MyState {
        current_state: u64::default(),
        file_state: None,
        edit_state: None,
        view_state: None,
        help_state: None,
    };
    states
}

pub fn initialize_gui() -> iced::Result {
    iced::application(new, update, view).run()
}