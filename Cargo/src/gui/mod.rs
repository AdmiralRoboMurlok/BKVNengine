mod file_column;
pub mod scene_manager;
pub mod scene_builder;
pub mod file_column_msg_handler;

use iced::{widget::text, widget::Column};
use iced::widget::{row, Row};
use crate::gui::file_column::file_column;
use crate::gui::scene_manager::scene_bar;
use crate::gui::scene_builder::scene_view;
use crate::placeholder;

#[derive(Default)]
struct MyState {
    current_state: u64,
}

#[derive(Debug, Clone)]
pub enum Messages {
    Exit,
    ImportCharacter,
    ImportBackground,
    ImportSound,
    placeholder_msg,
}

fn update(state: &mut MyState, message: Messages) {
    match message {
        Messages::Exit => placeholder(),
        Messages::ImportCharacter => placeholder(),
        Messages::ImportBackground => placeholder(),
        Messages::ImportSound => placeholder(),
        Messages::placeholder_msg => placeholder(),
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
        current_state: u64::default()
    };
    states
}

pub fn initialize_gui() -> iced::Result {
    iced::application(new, update, view).run()
}