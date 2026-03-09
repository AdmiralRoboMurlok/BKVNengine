mod file_column;
pub mod scene_manager;
pub mod scene_builder;
pub mod file_column_msg_handler;

use iced::widget::{row, Row};
use crate::gui::file_column::file_column;
use crate::gui::scene_manager::scene_bar;
use crate::gui::scene_builder::scene_view;
use crate::gui::file_column_msg_handler::*;
use crate::placeholder;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

//Add a file for the enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCountMacro, EnumIter)]
pub enum FilesOptions {
    NewFile,
    OpenFile,
    SaveFile,
}

impl std::fmt::Display for FilesOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NewFile => "New file",
            Self::OpenFile => "Open file",
            Self::SaveFile => "Save file",
        })
    }
}

#[derive(Default)]
struct MyState {
    current_state: u64,
    file_state: Option<FilesOptions>
}

#[derive(Debug, Clone)]
pub enum Messages {
    Exit,
    ImportCharacter,
    ImportBackground,
    ImportSound,
    FileHandler(FilesOptions),
    PlaceholderMsg,
}

fn update(state: &mut MyState, message: Messages) {
    match message {
        Messages::Exit => placeholder(),
        Messages::ImportCharacter => import_character(),
        Messages::ImportBackground => import_background(),
        Messages::ImportSound => import_sound(),
        Messages::FileHandler(option) => placeholder(),
        Messages::PlaceholderMsg => placeholder(),
    }
}

fn view(state: &MyState) -> Row<'_, Messages> {
    row![
        file_column(state),
        scene_view(state),
        scene_bar(state),
    ]
}

fn new() -> MyState {
    let states = MyState {
        current_state: u64::default(),
        file_state: None,
    };
    states
}

pub fn initialize_gui() -> iced::Result {
    iced::application(new, update, view).run()
}