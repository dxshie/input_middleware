use std::ops::Neg;

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl From<ButtonState> for i32 {
    fn from(state: ButtonState) -> Self {
        match state {
            ButtonState::Pressed => 0x1,
            ButtonState::Released => 0x0,
        }
    }
}

impl From<i32> for ButtonState {
    fn from(state: i32) -> Self {
        match state {
            0x1 => ButtonState::Pressed,
            _ => ButtonState::Released,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MwheelState {
    Up(i32),
    Released,
    Down(i32),
}

impl From<MwheelState> for i32 {
    fn from(state: MwheelState) -> Self {
        match state {
            MwheelState::Up(data) => data,
            MwheelState::Released => 0x0,
            MwheelState::Down(data) => data.neg(),
        }
    }
}
