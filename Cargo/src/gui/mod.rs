mod file_column;
pub mod scene_manager;
pub mod scene_builder;

use iced::{widget::text, widget::Column};
use crate::gui::file_column::file_column;
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
}

fn update(state: &mut MyState, message: Messages) {
    match message {
        Messages::Exit => placeholder(),
        Messages::ImportCharacter => placeholder(),
        Messages::ImportBackground => placeholder(),
        Messages::ImportSound => placeholder(),
    }
}

fn view(state: &MyState) -> Column<'_, Messages> {
    file_column(state)
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