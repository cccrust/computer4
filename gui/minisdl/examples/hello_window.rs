//! Hello window — the simplest possible MiniSDL application.
//!
//! Run with:  cargo run --example hello_window

use minisdl::prelude::*;
use minisdl::audio::{AudioDevice, Oscillator, Waveform};

fn main() -> Result<()> {
    let mut ctx = minisdl::MiniSDL::init()?;
    let mut window = ctx.create_window("Hello MiniSDL", 800, 600)?;
    let mut timer  = minisdl::timer::Timer::new(60);

    // Set up a simple sine-wave beep
    let mut audio  = AudioDevice::new(44100).expect("audio init");
    let osc_idx    = audio.add_oscillator(Oscillator::sine(440.0).with_amplitude(0.3));

    let mut frame: u64 = 0;
    let mut angle: f32 = 0.0;

    // Simulate a 180-frame run (3 s @ 60 fps) without a real OS window
    while frame < 180 {
        // ── Events ─────────────────────────────────────────────────────────
        for event in ctx.poll_events() {
            match event {
                Event::Quit => {
                    println!("Quit event — exiting.");
                    return Ok(());
                }
                Event::KeyDown { key, .. } => {
                    println!("Key pressed: {:?}", key);
                }
                _ => {}
            }
        }

        // ── Update ─────────────────────────────────────────────────────────
        let dt = timer.tick();
        angle += dt * 90.0; // degrees per second

        // Toggle audio at frame 60
        if frame == 60 {
            if let Some(osc) = audio.oscillator_mut(osc_idx) {
                osc.stop();
                println!("Audio stopped at frame {}", frame);
            }
        }

        // ── Draw ───────────────────────────────────────────────────────────
        let mut canvas = window.canvas();

        // Background gradient
        canvas.set_draw_color(Color::rgb(20, 20, 40));
        canvas.clear();

        // Animated circle
        let cx = (400.0 + angle.to_radians().cos() * 150.0) as i32;
        let cy = (300.0 + angle.to_radians().sin() * 150.0) as i32;
        canvas.set_draw_color(Color::CYAN);
        canvas.fill_circle(cx, cy, 30)?;

        // Static rect
        canvas.set_draw_color(Color::rgba(255, 100, 50, 180));
        canvas.fill_rounded_rect(Rect::new(300, 250, 200, 100), 12)?;

        // Triangle
        canvas.set_draw_color(Color::YELLOW);
        canvas.fill_triangle(
            Point::new(400, 200),
            Point::new(340, 280),
            Point::new(460, 280),
        )?;

        canvas.present()?;

        // Log progress every 60 frames
        if frame % 60 == 0 {
            println!(
                "Frame {:4}  FPS: {:.1}  angle: {:.1}°",
                frame,
                timer.fps(),
                angle % 360.0
            );
        }

        timer.delay_to_target();
        frame += 1;
    }

    println!("Done.  Average FPS: {:.1}", timer.average_fps());

    // Generate a 0.5-second WAV snippet to prove audio works
    let wav = audio.generate_wav(0.5);
    println!("Generated WAV: {} bytes", wav.len());

    Ok(())
}
