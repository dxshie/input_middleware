#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CmdHead {
    pub mac: u32,
    pub rand: u32,
    pub indexpts: u32,
    pub cmd: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CmdData {
    pub u8buff: [u8; 1024],
    pub u16buff: [u16; 512],
    pub cmd_mouse: SoftMouse,
    pub cmd_keyboard: SoftKeyboard,
}

impl CmdData {
    /// # Safety
    /// Accessing the union field is unsafe
    pub fn get_cmd_mouse(&self) -> SoftMouse {
        unsafe { self.cmd_mouse }
    }

    /// # Safety
    /// Accessing the union field is unsafe
    pub fn get_cmd_keyboard(&self) -> SoftKeyboard {
        unsafe { self.cmd_keyboard }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClientTx {
    pub head: CmdHead,
    pub data: CmdData,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SoftMouse {
    pub button: i32, // 8 buttons
    pub x: i32,      // -32767 to 32767
    pub y: i32,      // -32767 to 32767
    pub wheel: i32,  // -32767 to 32767
    pub point: [i32; 10],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SoftKeyboard {
    pub ctrl: char,
    pub resvel: char,
    pub button: [char; 10],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MonitorKeyboardData {
    pub report_id: char,
    pub buttons: char,
    pub data: [char; 10],
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MonitorMouseData {
    pub report_id: char,
    pub button: i32, // 8 buttons
    pub x: i32,      // -32767 to 32767
    pub y: i32,      // -32767 to 32767
    pub wheel: i32,  // -32767 to 32767
}
