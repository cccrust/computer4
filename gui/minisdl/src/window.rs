//! Window abstraction and software-rendered canvas.

use crate::{
    color::Color,
    error::{MiniSDLError, Result},
    graphics::Canvas,
};

/// A logical window with an associated software framebuffer.
pub struct Window {
    title:  String,
    width:  u32,
    height: u32,
    /// Raw RGBA pixel buffer – row-major, top-to-bottom.
    framebuffer: Vec<u8>,
}

impl Window {
    /// Create a new window.  Width and height must be at least 1.
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(MiniSDLError::InvalidDimensions { width, height });
        }
        Ok(Self {
            title: title.to_owned(),
            width,
            height,
            framebuffer: vec![0u8; (width * height * 4) as usize],
        })
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    pub fn title(&self)  -> &str { &self.title }
    pub fn width(&self)  -> u32  { self.width  }
    pub fn height(&self) -> u32  { self.height }
    pub fn size(&self)   -> (u32, u32) { (self.width, self.height) }

    /// Update the window title.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    /// Resize the window (clears the framebuffer).
    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(MiniSDLError::InvalidDimensions { width, height });
        }
        self.width  = width;
        self.height = height;
        self.framebuffer = vec![0u8; (width * height * 4) as usize];
        Ok(())
    }

    // ── Canvas creation ───────────────────────────────────────────────────────

    /// Borrow a [`Canvas`] that renders into this window's framebuffer.
    pub fn canvas(&mut self) -> Canvas<'_> {
        Canvas::new(self)
    }

    // ── Framebuffer access ────────────────────────────────────────────────────

    /// Raw RGBA bytes of the current framebuffer.
    pub fn pixels(&self) -> &[u8] {
        &self.framebuffer
    }

    /// Mutable raw RGBA bytes.
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.framebuffer
    }

    /// Read the colour of a single pixel.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        let idx = ((y as u32 * self.width + x as u32) * 4) as usize;
        Some(Color::rgba(
            self.framebuffer[idx],
            self.framebuffer[idx + 1],
            self.framebuffer[idx + 2],
            self.framebuffer[idx + 3],
        ))
    }

    /// Write a single pixel directly (no blending).
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let idx = ((y as u32 * self.width + x as u32) * 4) as usize;
        self.framebuffer[idx]     = color.r;
        self.framebuffer[idx + 1] = color.g;
        self.framebuffer[idx + 2] = color.b;
        self.framebuffer[idx + 3] = color.a;
    }

    /// Export the framebuffer as a PPM image string (useful for testing).
    pub fn save_ppm(&self) -> String {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height);
        for chunk in self.framebuffer.chunks_exact(4) {
            out.push_str(&format!("{} {} {} ", chunk[0], chunk[1], chunk[2]));
        }
        out
    }
}
