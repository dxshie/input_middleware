#[derive(Debug, Clone, Copy)]
pub enum KeyboardKey {
    KEY_NONE,
    KEY_ERRORROLLOVER,
    KEY_POSTFAIL,
    KEY_ERRORUNDEFINED,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_1_EXCLAMATION_MARK,
    KEY_2_AT,
    KEY_3_NUMBER_SIGN,
    KEY_4_DOLLAR,
    KEY_5_PERCENT,
    KEY_6_CARET,
    KEY_7_AMPERSAND,
    KEY_8_ASTERISK,
    KEY_9_OPARENTHESIS,
    KEY_0_CPARENTHESIS,
    KEY_ENTER,
    KEY_ESCAPE,
    KEY_BACKSPACE,
    KEY_TAB,
    KEY_SPACEBAR,
    KEY_MINUS_UNDERSCORE,
    KEY_EQUAL_PLUS,
    KEY_OBRACKET_AND_OBRACE,
    KEY_CBRACKET_AND_CBRACE,
    KEY_BACKSLASH_VERTICAL_BAR,
    KEY_NONUS_NUMBER_SIGN_TILDE,
    KEY_SEMICOLON_COLON,
    KEY_SINGLE_AND_DOUBLE_QUOTE,
    KEY_GRAVE_ACCENT_AND_TILDE,
    KEY_COMMA_AND_LESS,
    KEY_DOT_GREATER,
    KEY_SLASH_QUESTION,
    KEY_CAPS_LOCK,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_F10,
    KEY_F11,
    KEY_F12,
    KEY_PRINTSCREEN,
    KEY_SCROLL_LOCK,
    KEY_PAUSE,
    KEY_INSERT,
    KEY_HOME,
    KEY_PAGEUP,
    KEY_DELETE,
    KEY_END1,
    KEY_PAGEDOWN,
    KEY_RIGHTARROW,
    KEY_LEFTARROW,
    KEY_DOWNARROW,
    KEY_UPARROW,
    KEY_KEYPAD_NUM_LOCK_AND_CLEAR,
    KEY_KEYPAD_SLASH,
    KEY_KEYPAD_ASTERIKS,
    KEY_KEYPAD_MINUS,
    KEY_KEYPAD_PLUS,
    KEY_KEYPAD_ENTER,
    KEY_KEYPAD_1_END,
    KEY_KEYPAD_2_DOWN_ARROW,
    KEY_KEYPAD_3_PAGEDN,
    KEY_KEYPAD_4_LEFT_ARROW,
    KEY_KEYPAD_5,
    KEY_KEYPAD_6_RIGHT_ARROW,
    KEY_KEYPAD_7_HOME,
    KEY_KEYPAD_8_UP_ARROW,
    KEY_KEYPAD_9_PAGEUP,
    KEY_KEYPAD_0_INSERT,
    KEY_KEYPAD_DECIMAL_SEPARATOR_DELETE,
    KEY_NONUS_BACK_SLASH_VERTICAL_BAR,
    KEY_APPLICATION,
    KEY_POWER,
    KEY_KEYPAD_EQUAL,
    KEY_F13,
    KEY_F14,
    KEY_F15,
    KEY_F16,
    KEY_F17,
    KEY_F18,
    KEY_F19,
    KEY_F20,
    KEY_F21,
    KEY_F22,
    KEY_F23,
    KEY_F24,
    KEY_EXECUTE,
    KEY_HELP,
    KEY_MENU,
    KEY_SELECT,
    KEY_STOP,
    KEY_AGAIN,
    KEY_UNDO,
    KEY_CUT,
    KEY_COPY,
    KEY_PASTE,
    KEY_FIND,
    KEY_MUTE,
    KEY_VOLUME_UP,
    KEY_VOLUME_DOWN,
    KEY_LOCKING_CAPS_LOCK,
    KEY_LOCKING_NUM_LOCK,
    KEY_LOCKING_SCROLL_LOCK,
    KEY_KEYPAD_COMMA,
    KEY_KEYPAD_EQUAL_SIGN,
    KEY_INTERNATIONAL1,
    KEY_INTERNATIONAL2,
    KEY_INTERNATIONAL3,
    KEY_INTERNATIONAL4,
    KEY_INTERNATIONAL5,
    KEY_INTERNATIONAL6,
    KEY_INTERNATIONAL7,
    KEY_INTERNATIONAL8,
    KEY_INTERNATIONAL9,
    KEY_LANG1,
    KEY_LANG2,
    KEY_LANG3,
    KEY_LANG4,
    KEY_LANG5,
    KEY_LANG6,
    KEY_LANG7,
    KEY_LANG8,
    KEY_LANG9,
    KEY_ALTERNATE_ERASE,
    KEY_SYSREQ,
    KEY_CANCEL,
    KEY_CLEAR,
    KEY_PRIOR,
    KEY_RETURN,
    KEY_SEPARATOR,
    KEY_OUT,
    KEY_OPER,
    KEY_CLEAR_AGAIN,
    KEY_CRSEL,
    KEY_EXSEL,
    KEY_KEYPAD_00,
    KEY_KEYPAD_000,
    KEY_THOUSANDS_SEPARATOR,
    KEY_DECIMAL_SEPARATOR,
    KEY_CURRENCY_UNIT,
    KEY_CURRENCY_SUB_UNIT,
    KEY_KEYPAD_OPARENTHESIS,
    KEY_KEYPAD_CPARENTHESIS,
    KEY_KEYPAD_OBRACE,
    KEY_KEYPAD_CBRACE,
    KEY_KEYPAD_TAB,
    KEY_KEYPAD_BACKSPACE,
    KEY_KEYPAD_A,
    KEY_KEYPAD_B,
    KEY_KEYPAD_C,
    KEY_KEYPAD_D,
    KEY_KEYPAD_E,
    KEY_KEYPAD_F,
    KEY_KEYPAD_XOR,
    KEY_KEYPAD_CARET,
    KEY_KEYPAD_PERCENT,
    KEY_KEYPAD_LESS,
    KEY_KEYPAD_GREATER,
    KEY_KEYPAD_AMPERSAND,
    KEY_KEYPAD_LOGICAL_AND,
    KEY_KEYPAD_VERTICAL_BAR,
    KEY_KEYPAD_LOGICAL_OR,
    KEY_KEYPAD_COLON,
    KEY_KEYPAD_NUMBER_SIGN,
    KEY_KEYPAD_SPACE,
    KEY_KEYPAD_AT,
    KEY_KEYPAD_EXCLAMATION_MARK,
    KEY_KEYPAD_MEMORY_STORE,
    KEY_KEYPAD_MEMORY_RECALL,
    KEY_KEYPAD_MEMORY_CLEAR,
    KEY_KEYPAD_MEMORY_ADD,
    KEY_KEYPAD_MEMORY_SUBTRACT,
    KEY_KEYPAD_MEMORY_MULTIPLY,
    KEY_KEYPAD_MEMORY_DIVIDE,
    KEY_KEYPAD_PLUSMINUS,
    KEY_KEYPAD_CLEAR,
    KEY_KEYPAD_CLEAR_ENTRY,
    KEY_KEYPAD_BINARY,
    KEY_KEYPAD_OCTAL,
    KEY_KEYPAD_DECIMAL,
    KEY_KEYPAD_HEXADECIMAL,
    KEY_LEFTCONTROL,
    KEY_LEFTSHIFT,
    KEY_LEFTALT,
    KEY_LEFT_GUI,
    KEY_RIGHTCONTROL,
    KEY_RIGHTSHIFT,
    KEY_RIGHTALT,
    KEY_RIGHT_GUI,
}
