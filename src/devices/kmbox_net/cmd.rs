use super::cmd_instruction::{
    CMD_BAZER_MOVE, CMD_CONNECT, CMD_DEBUG, CMD_KEYBOARD_ALL, CMD_MASK_MOUSE, CMD_MONITOR,
    CMD_MOUSE_AUTOMOVE, CMD_MOUSE_LEFT, CMD_MOUSE_MIDDLE, CMD_MOUSE_MOVE, CMD_MOUSE_RIGHT,
    CMD_MOUSE_WHEEL, CMD_REBOOT, CMD_SETCONFIG, CMD_SHOWPIC, CMD_UNMASK_ALL,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum CMD {
    CONNECT,
    MOUSE_MOVE,
    MOUSE_LEFT,
    MOUSE_MIDDLE,
    MOUSE_RIGHT,
    MOUSE_WHEEL,
    MOUSE_AUTOMOVE,
    KEYBOARD_ALL,
    REBOOT,
    BAZER_MOVE,
    MONITOR,
    DEBUG,
    MASK_MOUSE,
    UNMASK_ALL,
    SETCONFIG,
    SHOWPIC,
}

impl From<CMD> for u32 {
    fn from(cmd: CMD) -> Self {
        match cmd {
            CMD::CONNECT => CMD_CONNECT,
            CMD::MOUSE_MOVE => CMD_MOUSE_MOVE,
            CMD::MOUSE_LEFT => CMD_MOUSE_LEFT,
            CMD::MOUSE_MIDDLE => CMD_MOUSE_MIDDLE,
            CMD::MOUSE_RIGHT => CMD_MOUSE_RIGHT,
            CMD::MOUSE_WHEEL => CMD_MOUSE_WHEEL,
            CMD::MOUSE_AUTOMOVE => CMD_MOUSE_AUTOMOVE,
            CMD::KEYBOARD_ALL => CMD_KEYBOARD_ALL,
            CMD::REBOOT => CMD_REBOOT,
            CMD::BAZER_MOVE => CMD_BAZER_MOVE,
            CMD::MONITOR => CMD_MONITOR,
            CMD::DEBUG => CMD_DEBUG,
            CMD::MASK_MOUSE => CMD_MASK_MOUSE,
            CMD::UNMASK_ALL => CMD_UNMASK_ALL,
            CMD::SETCONFIG => CMD_SETCONFIG,
            CMD::SHOWPIC => CMD_SHOWPIC,
        }
    }
}

impl From<u32> for CMD {
    fn from(cmd: u32) -> Self {
        match cmd {
            CMD_CONNECT => CMD::CONNECT,
            CMD_MOUSE_MOVE => CMD::MOUSE_MOVE,
            CMD_MOUSE_LEFT => CMD::MOUSE_LEFT,
            CMD_MOUSE_MIDDLE => CMD::MOUSE_MIDDLE,
            CMD_MOUSE_RIGHT => CMD::MOUSE_RIGHT,
            CMD_MOUSE_WHEEL => CMD::MOUSE_WHEEL,
            CMD_MOUSE_AUTOMOVE => CMD::MOUSE_AUTOMOVE,
            CMD_KEYBOARD_ALL => CMD::KEYBOARD_ALL,
            CMD_REBOOT => CMD::REBOOT,
            CMD_BAZER_MOVE => CMD::BAZER_MOVE,
            CMD_MONITOR => CMD::MONITOR,
            CMD_DEBUG => CMD::DEBUG,
            CMD_MASK_MOUSE => CMD::MASK_MOUSE,
            CMD_UNMASK_ALL => CMD::UNMASK_ALL,
            CMD_SETCONFIG => CMD::SETCONFIG,
            CMD_SHOWPIC => CMD::SHOWPIC,
            _ => panic!("Unknown command"),
        }
    }
}
