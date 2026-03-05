mod file_column;

use iced::{widget::text, Element, widget::Row};
use crate::gui::file_column::file_column;

#[derive(Default)]
struct MyState {
    current_state: u64,
}

#[derive(Debug)]
pub enum Messages {
    Exit,
}

fn update(state: &mut MyState, message: Messages) {}

fn view(state: &MyState) -> Row<'_, Messages> {
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