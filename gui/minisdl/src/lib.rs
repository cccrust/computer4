//! # MiniSDL
//!
//! A lightweight SDL2-like multimedia library written in pure Rust.
//!
//! MiniSDL provides abstractions for:
//! - Window creation and management
//! - 2D software rendering (pixels, lines, rectangles, circles)
//! - Keyboard and mouse input
//! - Event handling
//! - Basic audio (sine wave generation)
//! - Timing / frame-rate control
//!
//! ## Quick Start
//!
//! ```no_run
//! use minisdl::prelude::*;
//!
//! fn main() -> Result<()> {
//!     let mut ctx = MiniSDL::init()?;
//!     let mut window = ctx.create_window("Hello MiniSDL", 800, 600)?;
//!     let mut canvas = window.canvas();
//!
//!     'running: loop {
//!         for event in ctx.poll_events() {
//!             match event {
//!                 Event::Quit => break 'running,
//!                 _ => {}
//!             }
//!         }
//!         canvas.set_draw_color(Color::BLACK);
//!         canvas.clear();
//!         canvas.set_draw_color(Color::RED);
//!         canvas.fill_rect(Rect::new(100, 100, 200, 150))?;
//!         canvas.present()?;
//!         ctx.delay(16);
//!     }
//!     Ok(())
//! }
//! ```

#![allow(dead_code)]

pub mod audio;
pub mod color;
pub mod error;
pub mod event;
pub mod graphics;
pub mod input;
pub mod rect;
pub mod timer;
pub mod window;

pub use error::{MiniSDLError, Result};

/// Re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::error::Result;
    pub use crate::event::Event;
    pub use crate::graphics::Canvas;
    pub use crate::input::{Keycode, MouseButton};
    pub use crate::rect::{Point, Rect};
    pub use crate::timer::Timer;
    pub use crate::window::Window;
    pub use crate::MiniSDL;
}

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use event::Event;
use input::{InputState, Keycode, MouseButton};

/// The central MiniSDL context.  
/// Initialise once with [`MiniSDL::init`] and keep alive for the lifetime of your app.
pub struct MiniSDL {
    pub(crate) input: InputState,
    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) start_time: Instant,
}

impl MiniSDL {
    /// Initialise MiniSDL.
    pub fn init() -> Result<Self> {
        Ok(Self {
            input: InputState::new(),
            event_queue: VecDeque::new(),
            start_time: Instant::now(),
        })
    }

    /// Create a new [`Window`].
    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> Result<window::Window> {
        window::Window::new(title, width, height)
    }

    /// Drain all pending events.  Call once per frame.
    pub fn poll_events(&mut self) -> Vec<Event> {
        // Pump simulated / platform events
        self.pump_events();
        self.event_queue.drain(..).collect()
    }

    /// Push a custom event into the queue.
    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    /// Simulate a key press (useful for testing / headless mode).
    pub fn simulate_key_press(&mut self, key: Keycode) {
        self.input.press_key(key);
        self.event_queue.push_back(Event::KeyDown { key, repeat: false });
    }

    /// Simulate a key release.
    pub fn simulate_key_release(&mut self, key: Keycode) {
        self.input.release_key(key);
        self.event_queue.push_back(Event::KeyUp { key });
    }

    /// Simulate a mouse button press.
    pub fn simulate_mouse_press(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input.press_mouse(button, x, y);
        self.event_queue
            .push_back(Event::MouseButtonDown { button, x, y });
    }

    /// Simulate a mouse button release.
    pub fn simulate_mouse_release(&mut self, button: MouseButton, x: i32, y: i32) {
        self.input.release_mouse(button, x, y);
        self.event_queue
            .push_back(Event::MouseButtonUp { button, x, y });
    }

    /// Simulate mouse movement.
    pub fn simulate_mouse_move(&mut self, x: i32, y: i32, dx: i32, dy: i32) {
        self.input.move_mouse(x, y);
        self.event_queue.push_back(Event::MouseMotion { x, y, dx, dy });
    }

    /// Get a reference to the current input state.
    pub fn input(&self) -> &InputState {
        &self.input
    }

    /// Get a mutable reference to the current input state.
    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    /// Sleep for approximately `ms` milliseconds.
    pub fn delay(&self, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
    }

    /// Milliseconds elapsed since [`MiniSDL::init`] was called.
    pub fn ticks(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    /// Push a Quit event, triggering a graceful shutdown.
    pub fn quit(&mut self) {
        self.event_queue.push_back(Event::Quit);
    }

    // --------------- internal ---------------

    fn pump_events(&mut self) {
        // In a real platform backend this would call OS APIs.
        // Here we just ensure the quit sentinel propagates.
    }
}
