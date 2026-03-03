use iced::{widget::text, Element};

#[derive(Default)]
struct MyState {
    current_state: u64,
}

#[derive(Debug)]
enum Messages {
    Exit,
}

fn update(state: &mut MyState, message: Messages) {}

fn view(state: &MyState) -> Element<'_, Messages> {
    text("Hi hello").into()
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