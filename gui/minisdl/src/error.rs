//! Error types for MiniSDL.

use std::fmt;

/// Every MiniSDL operation that can fail returns this.
pub type Result<T> = std::result::Result<T, MiniSDLError>;

/// All possible errors emitted by MiniSDL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiniSDLError {
    /// Window creation failed.
    WindowCreation(String),
    /// The framebuffer dimensions are invalid.
    InvalidDimensions { width: u32, height: u32 },
    /// A render operation was out of bounds.
    OutOfBounds { x: i32, y: i32 },
    /// Audio subsystem error.
    Audio(String),
    /// Generic / platform error.
    Generic(String),
}

impl fmt::Display for MiniSDLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowCreation(msg) => write!(f, "Window creation error: {}", msg),
            Self::InvalidDimensions { width, height } => {
                write!(f, "Invalid dimensions: {}x{}", width, height)
            }
            Self::OutOfBounds { x, y } => write!(f, "Coordinate out of bounds: ({}, {})", x, y),
            Self::Audio(msg) => write!(f, "Audio error: {}", msg),
            Self::Generic(msg) => write!(f, "MiniSDL error: {}", msg),
        }
    }
}

impl std::error::Error for MiniSDLError {}

impl From<String> for MiniSDLError {
    fn from(s: String) -> Self {
        Self::Generic(s)
    }
}

impl From<&str> for MiniSDLError {
    fn from(s: &str) -> Self {
        Self::Generic(s.to_owned())
    }
}
