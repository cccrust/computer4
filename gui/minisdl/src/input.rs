//! Keyboard and mouse input state tracking.

use std::collections::HashSet;

// ── Keycode ───────────────────────────────────────────────────────────────────

/// Virtual key codes, modelled after SDL2's `SDL_Keycode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keycode {
    // Alphabet
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Digits
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,

    // Function keys
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,

    // Navigation / editing
    Return, Escape, Backspace, Tab, Space, Delete, Insert,
    Home, End, PageUp, PageDown,
    Left, Right, Up, Down,

    // Modifiers
    LShift, RShift,
    LCtrl,  RCtrl,
    LAlt,   RAlt,
    LGui,   RGui,

    // Punctuation / symbols
    Minus, Equals, LeftBracket, RightBracket, Backslash,
    Semicolon, Apostrophe, Grave, Comma, Period, Slash,

    // Numpad
    Kp0, Kp1, Kp2, Kp3, Kp4,
    Kp5, Kp6, Kp7, Kp8, Kp9,
    KpPlus, KpMinus, KpMultiply, KpDivide, KpEnter, KpPeriod,

    // Misc
    CapsLock, NumLock, ScrollLock, PrintScreen, Pause,

    /// Any other key (scancode).
    Unknown(u32),
}

impl Keycode {
    /// Returns `true` if this is a modifier key.
    pub fn is_modifier(self) -> bool {
        matches!(
            self,
            Self::LShift | Self::RShift
                | Self::LCtrl | Self::RCtrl
                | Self::LAlt  | Self::RAlt
                | Self::LGui  | Self::RGui
        )
    }

    /// Attempt to convert a lowercase ASCII character to a Keycode.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' => Some(Self::A), 'b' => Some(Self::B), 'c' => Some(Self::C),
            'd' => Some(Self::D), 'e' => Some(Self::E), 'f' => Some(Self::F),
            'g' => Some(Self::G), 'h' => Some(Self::H), 'i' => Some(Self::I),
            'j' => Some(Self::J), 'k' => Some(Self::K), 'l' => Some(Self::L),
            'm' => Some(Self::M), 'n' => Some(Self::N), 'o' => Some(Self::O),
            'p' => Some(Self::P), 'q' => Some(Self::Q), 'r' => Some(Self::R),
            's' => Some(Self::S), 't' => Some(Self::T), 'u' => Some(Self::U),
            'v' => Some(Self::V), 'w' => Some(Self::W), 'x' => Some(Self::X),
            'y' => Some(Self::Y), 'z' => Some(Self::Z),
            '0' => Some(Self::Num0), '1' => Some(Self::Num1),
            '2' => Some(Self::Num2), '3' => Some(Self::Num3),
            '4' => Some(Self::Num4), '5' => Some(Self::Num5),
            '6' => Some(Self::Num6), '7' => Some(Self::Num7),
            '8' => Some(Self::Num8), '9' => Some(Self::Num9),
            ' ' => Some(Self::Space),
            '\n' | '\r' => Some(Self::Return),
            '\t' => Some(Self::Tab),
            _ => None,
        }
    }
}

// ── MouseButton ───────────────────────────────────────────────────────────────

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    /// Extra button 1 (back).
    X1,
    /// Extra button 2 (forward).
    X2,
}

// ── InputState ────────────────────────────────────────────────────────────────

/// Snapshot of all keyboard and mouse state.
pub struct InputState {
    pressed_keys:    HashSet<Keycode>,
    pressed_buttons: HashSet<MouseButton>,
    mouse_x: i32,
    mouse_y: i32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys:    HashSet::new(),
            pressed_buttons: HashSet::new(),
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    // ── Keyboard ──────────────────────────────────────────────────────────────

    pub(crate) fn press_key(&mut self, key: Keycode) {
        self.pressed_keys.insert(key);
    }

    pub(crate) fn release_key(&mut self, key: Keycode) {
        self.pressed_keys.remove(&key);
    }

    /// Returns `true` while `key` is held down.
    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Returns `true` when `key` is NOT held down.
    pub fn is_key_up(&self, key: Keycode) -> bool {
        !self.is_key_down(key)
    }

    /// Snapshot of all currently pressed keys.
    pub fn pressed_keys(&self) -> impl Iterator<Item = &Keycode> {
        self.pressed_keys.iter()
    }

    // ── Mouse ─────────────────────────────────────────────────────────────────

    pub(crate) fn press_mouse(&mut self, btn: MouseButton, x: i32, y: i32) {
        self.pressed_buttons.insert(btn);
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub(crate) fn release_mouse(&mut self, btn: MouseButton, x: i32, y: i32) {
        self.pressed_buttons.remove(&btn);
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub(crate) fn move_mouse(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    /// Returns `true` while `button` is held down.
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    /// Current mouse cursor position.
    pub fn mouse_position(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
