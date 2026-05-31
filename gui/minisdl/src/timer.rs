//! Frame-rate limiter and delta-time tracker.

use std::time::{Duration, Instant};

/// Fixed-step / variable-step frame-rate controller.
pub struct Timer {
    last_tick:  Instant,
    target_fps: u32,
    frame_count: u64,
    accumulated: f32,
}

impl Timer {
    /// Create a timer targeting `fps` frames per second.
    pub fn new(fps: u32) -> Self {
        Self {
            last_tick:   Instant::now(),
            target_fps:  fps.max(1),
            frame_count: 0,
            accumulated: 0.0,
        }
    }

    /// Call once per frame.  Returns the elapsed time since the last call in seconds.
    pub fn tick(&mut self) -> f32 {
        let now  = Instant::now();
        let dt   = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;
        self.frame_count += 1;
        self.accumulated += dt;
        dt
    }

    /// Sleep for whatever time remains in the current frame to hit the target FPS.
    pub fn delay_to_target(&self) {
        let frame_budget = Duration::from_secs_f64(1.0 / self.target_fps as f64);
        let elapsed = self.last_tick.elapsed();
        if elapsed < frame_budget {
            std::thread::sleep(frame_budget - elapsed);
        }
    }

    /// Current instantaneous FPS (inverse of last delta-time).
    pub fn fps(&self) -> f32 {
        let dt = self.last_tick.elapsed().as_secs_f32();
        if dt > 0.0 { 1.0 / dt } else { self.target_fps as f32 }
    }

    /// Total frames ticked since creation.
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Average FPS since creation.
    pub fn average_fps(&self) -> f32 {
        if self.accumulated > 0.0 {
            self.frame_count as f32 / self.accumulated
        } else {
            0.0
        }
    }

    /// Set a new target FPS.
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps.max(1);
    }

    pub fn target_fps(&self) -> u32 {
        self.target_fps
    }
}
