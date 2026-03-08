use iced::widget::{row, Row, column, Column};
use crate::gui::{Messages, MyState};

fn scene_layout(state: &MyState) -> Row<'_, Messages> {
    row![

    ]
}

pub fn scene_view(state: &MyState) -> Column<'_, Messages> {
    column![
        scene_layout(&state),
    ]
}