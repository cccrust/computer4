//! Shapes demo — exercises every drawing primitive.
//!
//! Run with:  cargo run --example shapes

use minisdl::prelude::*;
use minisdl::graphics::BlendMode;

fn main() -> Result<()> {
    let mut ctx    = minisdl::MiniSDL::init()?;
    let mut window = ctx.create_window("Shapes Demo", 800, 600)?;

    {
        let mut canvas = window.canvas();
        canvas.set_blend_mode(BlendMode::Alpha);

        // ── Background ────────────────────────────────────────────────────
        canvas.fill_gradient_v(
            Rect::new(0, 0, 800, 600),
            Color::rgb(15, 10, 30),
            Color::rgb(40, 20, 60),
        )?;

        // ── Filled + outlined rectangles ──────────────────────────────────
        canvas.set_draw_color(Color::rgb(200, 60, 60));
        canvas.fill_rect(Rect::new(50, 50, 150, 100))?;
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_rect(Rect::new(50, 50, 150, 100))?;

        canvas.set_draw_color(Color::rgb(60, 200, 60));
        canvas.fill_rounded_rect(Rect::new(250, 50, 150, 100), 20)?;

        // ── Circles ───────────────────────────────────────────────────────
        canvas.set_draw_color(Color::rgb(60, 120, 240));
        canvas.fill_circle(650, 100, 60)?;
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_circle(650, 100, 60)?;

        // ── Ellipse ───────────────────────────────────────────────────────
        canvas.set_draw_color(Color::rgba(255, 200, 0, 200));
        canvas.fill_ellipse(400, 200, 120, 60)?;

        // ── Lines & thick lines ───────────────────────────────────────────
        canvas.set_draw_color(Color::rgb(255, 100, 50));
        canvas.draw_thick_line(50, 220, 750, 220, 4)?;

        canvas.set_draw_color(Color::CYAN);
        canvas.draw_lines(&[
            Point::new(50,  250),
            Point::new(200, 300),
            Point::new(350, 250),
            Point::new(500, 320),
            Point::new(650, 260),
            Point::new(750, 280),
        ])?;

        // ── Triangles ─────────────────────────────────────────────────────
        canvas.set_draw_color(Color::rgb(255, 180, 50));
        canvas.fill_triangle(
            Point::new(100, 400),
            Point::new(50,  500),
            Point::new(150, 500),
        )?;
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_triangle(
            Point::new(100, 400),
            Point::new(50,  500),
            Point::new(150, 500),
        )?;

        // ── Bézier curves ─────────────────────────────────────────────────
        canvas.set_draw_color(Color::rgb(200, 80, 200));
        canvas.draw_bezier_quadratic(
            Point::new(200, 500),
            Point::new(400, 350),
            Point::new(600, 500),
            64,
        )?;

        canvas.set_draw_color(Color::rgb(80, 220, 200));
        canvas.draw_bezier_cubic(
            Point::new(200, 550),
            Point::new(300, 430),
            Point::new(500, 560),
            Point::new(700, 450),
            80,
        )?;

        // ── Gradient rect ─────────────────────────────────────────────────
        canvas.fill_gradient_h(
            Rect::new(450, 380, 300, 80),
            Color::rgb(255, 0,   128),
            Color::rgb(0,   128, 255),
        )?;

        // ── Clipping demo ─────────────────────────────────────────────────
        canvas.set_clip_rect(Some(Rect::new(450, 470, 300, 80)));
        canvas.set_draw_color(Color::rgba(255, 255, 0, 150));
        canvas.fill_circle(600, 510, 80)?;
        canvas.set_clip_rect(None);

        canvas.present()?;
    }

    // Save a PPM snapshot
    let ppm = window.save_ppm();
    println!("PPM size: {} bytes  (first 40 chars: {})", ppm.len(), &ppm[..40.min(ppm.len())]);

    // Verify some pixels
    let centre = window.get_pixel(650, 100);
    println!("Pixel at circle centre (650,100): {:?}", centre);

    println!("Shapes demo complete ✓");
    Ok(())
}
