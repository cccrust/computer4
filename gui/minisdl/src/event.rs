//! Event types.

use crate::input::{Keycode, MouseButton};

/// Every event that MiniSDL can emit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    // ── Window ────────────────────────────────────────────────────────────────
    /// The user (or code) requested the application to exit.
    Quit,
    /// The window was resized.
    WindowResized { width: u32, height: u32 },
    /// The window gained keyboard focus.
    WindowFocusGained,
    /// The window lost keyboard focus.
    WindowFocusLost,

    // ── Keyboard ──────────────────────────────────────────────────────────────
    /// A key was pressed.
    KeyDown { key: Keycode, repeat: bool },
    /// A key was released.
    KeyUp { key: Keycode },
    /// Text input (already decoded from the OS).
    TextInput { text: String },

    // ── Mouse ─────────────────────────────────────────────────────────────────
    /// The mouse cursor moved.
    MouseMotion { x: i32, y: i32, dx: i32, dy: i32 },
    /// A mouse button was pressed.
    MouseButtonDown { button: MouseButton, x: i32, y: i32 },
    /// A mouse button was released.
    MouseButtonUp { button: MouseButton, x: i32, y: i32 },
    /// The scroll wheel moved.
    MouseWheel { x: i32, y: i32 },

    // ── Custom ────────────────────────────────────────────────────────────────
    /// Application-defined event with an arbitrary integer payload.
    User { id: u32, data: i64 },
}

impl Event {
    /// Returns `true` for the `Quit` variant.
    pub fn is_quit(&self) -> bool {
        matches!(self, Self::Quit)
    }

    /// Returns `true` for any keyboard event.
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Self::KeyDown { .. } | Self::KeyUp { .. } | Self::TextInput { .. })
    }

    /// Returns `true` for any mouse event.
    pub fn is_mouse(&self) -> bool {
        matches!(
            self,
            Self::MouseMotion { .. }
                | Self::MouseButtonDown { .. }
                | Self::MouseButtonUp { .. }
                | Self::MouseWheel { .. }
        )
    }
}
