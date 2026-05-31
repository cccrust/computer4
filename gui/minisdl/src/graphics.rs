//! 2-D software-rendering canvas.
//!
//! `Canvas` borrows the window's framebuffer and provides a full suite of
//! drawing primitives: pixels, lines, rectangles, circles, triangles,
//! Bézier curves, and an optional clipping rectangle.

use crate::{
    color::Color,
    error::{MiniSDLError, Result},
    rect::{Point, Rect},
    window::Window,
};

/// A 2-D software renderer backed by a [`Window`]'s framebuffer.
pub struct Canvas<'w> {
    window:      &'w mut Window,
    draw_color:  Color,
    clip:        Option<Rect>,
    blend:       BlendMode,
}

/// How source pixels are composited over the destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlendMode {
    /// Ignore the source alpha – overwrite directly.
    None,
    /// Standard alpha blending (over operator).
    #[default]
    Alpha,
}

impl<'w> Canvas<'w> {
    pub(crate) fn new(window: &'w mut Window) -> Self {
        Self {
            window,
            draw_color: Color::WHITE,
            clip:       None,
            blend:      BlendMode::Alpha,
        }
    }

    // ── State setters ─────────────────────────────────────────────────────────

    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    pub fn draw_color(&self) -> Color {
        self.draw_color
    }

    pub fn set_clip_rect(&mut self, clip: Option<Rect>) {
        self.clip = clip;
    }

    pub fn clip_rect(&self) -> Option<Rect> {
        self.clip
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend = mode;
    }

    /// Window dimensions as `(width, height)`.
    pub fn output_size(&self) -> (u32, u32) {
        self.window.size()
    }

    // ── Low-level pixel write ─────────────────────────────────────────────────

    /// Write a single pixel, applying the current clip rect and blend mode.
    #[inline]
    pub fn put_pixel(&mut self, x: i32, y: i32) {
        self.put_pixel_color(x, y, self.draw_color);
    }

    #[inline]
    fn put_pixel_color(&mut self, x: i32, y: i32, color: Color) {
        let (w, h) = self.window.size();
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
            return;
        }
        if let Some(clip) = self.clip {
            if !clip.contains(Point::new(x, y)) {
                return;
            }
        }
        match self.blend {
            BlendMode::None => self.window.set_pixel(x, y, color),
            BlendMode::Alpha => {
                if color.a == 255 {
                    self.window.set_pixel(x, y, color);
                } else if color.a > 0 {
                    let dst = self.window.get_pixel(x, y).unwrap_or(Color::BLACK);
                    self.window.set_pixel(x, y, color.blend_over(dst));
                }
            }
        }
    }

    // ── Fill / Clear ──────────────────────────────────────────────────────────

    /// Fill the entire canvas with the current draw colour.
    pub fn clear(&mut self) {
        let color = self.draw_color;
        let (w, h) = self.window.size();
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                self.put_pixel_color(x, y, color);
            }
        }
    }

    /// Submit the current frame (no-op in software mode – the framebuffer is already current).
    pub fn present(&mut self) -> Result<()> {
        Ok(())
    }

    // ── Lines ─────────────────────────────────────────────────────────────────

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> Result<()> {
        bresenham(x0, y0, x1, y1, |x, y| self.put_pixel(x, y));
        Ok(())
    }

    pub fn draw_line_points(&mut self, p0: Point, p1: Point) -> Result<()> {
        self.draw_line(p0.x, p0.y, p1.x, p1.y)
    }

    /// Draw a connected series of line segments.
    pub fn draw_lines(&mut self, points: &[Point]) -> Result<()> {
        for pair in points.windows(2) {
            self.draw_line_points(pair[0], pair[1])?;
        }
        Ok(())
    }

    /// Draw a thick line by plotting multiple parallel thin lines.
    pub fn draw_thick_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, thickness: u32) -> Result<()> {
        if thickness == 1 {
            return self.draw_line(x0, y0, x1, y1);
        }
        let half = thickness as i32 / 2;
        let dx = (x1 - x0) as f32;
        let dy = (y1 - y0) as f32;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let nx = -dy / len;
        let ny =  dx / len;
        for i in -half..=half {
            let ox = (nx * i as f32).round() as i32;
            let oy = (ny * i as f32).round() as i32;
            self.draw_line(x0 + ox, y0 + oy, x1 + ox, y1 + oy)?;
        }
        Ok(())
    }

    // ── Rectangles ────────────────────────────────────────────────────────────

    /// Draw the outline of a rectangle.
    pub fn draw_rect(&mut self, rect: Rect) -> Result<()> {
        let x1 = rect.right() - 1;
        let y1 = rect.bottom() - 1;
        self.draw_line(rect.x, rect.y, x1,      rect.y)?;  // top
        self.draw_line(rect.x, y1,     x1,      y1)?;      // bottom
        self.draw_line(rect.x, rect.y, rect.x,  y1)?;      // left
        self.draw_line(x1,     rect.y, x1,      y1)?;      // right
        Ok(())
    }

    /// Fill a rectangle.
    pub fn fill_rect(&mut self, rect: Rect) -> Result<()> {
        let color = self.draw_color;
        for y in rect.top()..rect.bottom() {
            for x in rect.left()..rect.right() {
                self.put_pixel_color(x, y, color);
            }
        }
        Ok(())
    }

    /// Draw a rectangle with rounded corners.
    pub fn draw_rounded_rect(&mut self, rect: Rect, radius: u32) -> Result<()> {
        let r = radius.min(rect.width / 2).min(rect.height / 2) as i32;
        let x0 = rect.x + r;
        let y0 = rect.y + r;
        let x1 = rect.right() - 1 - r;
        let y1 = rect.bottom() - 1 - r;
        // straight edges
        self.draw_line(x0, rect.y,      x1, rect.y)?;
        self.draw_line(x0, rect.bottom()-1, x1, rect.bottom()-1)?;
        self.draw_line(rect.x,      y0, rect.x,      y1)?;
        self.draw_line(rect.right()-1, y0, rect.right()-1, y1)?;
        // corners
        self.draw_arc(x0, y0, r, 180, 270)?;
        self.draw_arc(x1, y0, r, 270, 360)?;
        self.draw_arc(x0, y1, r,  90, 180)?;
        self.draw_arc(x1, y1, r,   0,  90)?;
        Ok(())
    }

    /// Fill a rounded rectangle.
    pub fn fill_rounded_rect(&mut self, rect: Rect, radius: u32) -> Result<()> {
        let r = radius.min(rect.width / 2).min(rect.height / 2) as i32;
        // centre strip
        self.fill_rect(Rect::new(rect.x, rect.y + r, rect.width, (rect.height as i32 - r * 2).max(0) as u32))?;
        // top/bottom strips
        self.fill_rect(Rect::new(rect.x + r, rect.y, (rect.width as i32 - r * 2).max(0) as u32, r as u32))?;
        self.fill_rect(Rect::new(rect.x + r, rect.bottom() - r, (rect.width as i32 - r * 2).max(0) as u32, r as u32))?;
        // quarter-circle fills
        let cx = [rect.x + r, rect.right() - 1 - r, rect.x + r, rect.right() - 1 - r];
        let cy = [rect.y + r, rect.y + r, rect.bottom() - 1 - r, rect.bottom() - 1 - r];
        for i in 0..4 {
            self.fill_circle(cx[i], cy[i], r as u32)?;
        }
        Ok(())
    }

    // ── Circles ───────────────────────────────────────────────────────────────

    /// Draw the outline of a circle using the midpoint algorithm.
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: u32) -> Result<()> {
        let mut x = radius as i32;
        let mut y = 0i32;
        let mut err = 0i32;
        while x >= y {
            self.put_pixel(cx + x, cy + y);
            self.put_pixel(cx + y, cy + x);
            self.put_pixel(cx - y, cy + x);
            self.put_pixel(cx - x, cy + y);
            self.put_pixel(cx - x, cy - y);
            self.put_pixel(cx - y, cy - x);
            self.put_pixel(cx + y, cy - x);
            self.put_pixel(cx + x, cy - y);
            y += 1;
            err += 2 * y - 1;
            if err > x {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
        Ok(())
    }

    /// Fill a circle.
    pub fn fill_circle(&mut self, cx: i32, cy: i32, radius: u32) -> Result<()> {
        let color = self.draw_color;
        let r = radius as i32;
        for y in -r..=r {
            let half = ((r * r - y * y) as f32).sqrt() as i32;
            for x in -half..=half {
                self.put_pixel_color(cx + x, cy + y, color);
            }
        }
        Ok(())
    }

    /// Draw an ellipse outline.
    pub fn draw_ellipse(&mut self, cx: i32, cy: i32, rx: u32, ry: u32) -> Result<()> {
        let rx = rx as i64;
        let ry = ry as i64;
        let steps = ((rx + ry) as f64 * 2.0 * std::f64::consts::PI) as usize;
        let steps = steps.max(8);
        for i in 0..steps {
            let t = 2.0 * std::f64::consts::PI * i as f64 / steps as f64;
            let x = (cx as f64 + rx as f64 * t.cos()).round() as i32;
            let y = (cy as f64 + ry as f64 * t.sin()).round() as i32;
            self.put_pixel(x, y);
        }
        Ok(())
    }

    /// Fill an ellipse.
    pub fn fill_ellipse(&mut self, cx: i32, cy: i32, rx: u32, ry: u32) -> Result<()> {
        let color = self.draw_color;
        let rx = rx as f32;
        let ry = ry as f32;
        for y in -(ry as i32)..=(ry as i32) {
            let half = (rx * rx * (1.0 - (y as f32 / ry) * (y as f32 / ry))).sqrt() as i32;
            for x in -half..=half {
                self.put_pixel_color(cx + x, cy + y, color);
            }
        }
        Ok(())
    }

    // ── Arcs ──────────────────────────────────────────────────────────────────

    /// Draw an arc from `start_deg` to `end_deg` (degrees, clockwise).
    pub fn draw_arc(&mut self, cx: i32, cy: i32, radius: i32, start_deg: i32, end_deg: i32) -> Result<()> {
        let steps = (radius.abs() as f64 * 2.0 * std::f64::consts::PI / 4.0).max(8.0) as i32;
        for i in 0..steps {
            let t = start_deg as f64 + (end_deg - start_deg) as f64 * i as f64 / steps as f64;
            let rad = t.to_radians();
            let x = (cx as f64 + radius as f64 * rad.cos()).round() as i32;
            let y = (cy as f64 + radius as f64 * rad.sin()).round() as i32;
            self.put_pixel(x, y);
        }
        Ok(())
    }

    // ── Triangles ─────────────────────────────────────────────────────────────

    /// Draw the outline of a triangle.
    pub fn draw_triangle(&mut self, p0: Point, p1: Point, p2: Point) -> Result<()> {
        self.draw_line_points(p0, p1)?;
        self.draw_line_points(p1, p2)?;
        self.draw_line_points(p2, p0)?;
        Ok(())
    }

    /// Fill a triangle using scanline rasterisation.
    pub fn fill_triangle(&mut self, p0: Point, p1: Point, p2: Point) -> Result<()> {
        let color = self.draw_color;
        // Sort by y
        let mut pts = [p0, p1, p2];
        pts.sort_by_key(|p| p.y);
        let [a, b, c] = pts;

        let fill_flat = |canvas: &mut Canvas, y: i32, x0: i32, x1: i32| {
            let (lo, hi) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
            for x in lo..=hi {
                canvas.put_pixel_color(x, y, color);
            }
        };

        if b.y == c.y {
            // flat bottom
            for y in a.y..=b.y {
                let t = if b.y == a.y { 0.0 } else { (y - a.y) as f32 / (b.y - a.y) as f32 };
                let x0 = (a.x as f32 + (b.x - a.x) as f32 * t).round() as i32;
                let x1 = (a.x as f32 + (c.x - a.x) as f32 * t).round() as i32;
                fill_flat(self, y, x0, x1);
            }
        } else if a.y == b.y {
            // flat top
            for y in a.y..=c.y {
                let t = if c.y == a.y { 0.0 } else { (y - a.y) as f32 / (c.y - a.y) as f32 };
                let x0 = (a.x as f32 + (c.x - a.x) as f32 * t).round() as i32;
                let x1 = (b.x as f32 + (c.x - b.x) as f32 * t).round() as i32;
                fill_flat(self, y, x0, x1);
            }
        } else {
            // general – split at b.y
            let t = (b.y - a.y) as f32 / (c.y - a.y) as f32;
            let d = Point::new((a.x as f32 + (c.x - a.x) as f32 * t).round() as i32, b.y);
            // flat-bottom half
            for y in a.y..=b.y {
                let t = (y - a.y) as f32 / (b.y - a.y) as f32;
                let x0 = (a.x as f32 + (b.x - a.x) as f32 * t).round() as i32;
                let x1 = (a.x as f32 + (d.x - a.x) as f32 * t).round() as i32;
                fill_flat(self, y, x0, x1);
            }
            // flat-top half
            for y in b.y..=c.y {
                let t = (y - b.y) as f32 / (c.y - b.y) as f32;
                let x0 = (b.x as f32 + (c.x - b.x) as f32 * t).round() as i32;
                let x1 = (d.x as f32 + (c.x - d.x) as f32 * t).round() as i32;
                fill_flat(self, y, x0, x1);
            }
        }
        Ok(())
    }

    // ── Bézier curves ─────────────────────────────────────────────────────────

    /// Draw a quadratic Bézier curve from `p0` through control point `ctrl` to `p1`.
    pub fn draw_bezier_quadratic(&mut self, p0: Point, ctrl: Point, p1: Point, steps: u32) -> Result<()> {
        let steps = steps.max(2) as usize;
        let mut prev = p0;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let x = lerp2(p0.x as f32, ctrl.x as f32, p1.x as f32, t);
            let y = lerp2(p0.y as f32, ctrl.y as f32, p1.y as f32, t);
            let cur = Point::new(x.round() as i32, y.round() as i32);
            self.draw_line_points(prev, cur)?;
            prev = cur;
        }
        Ok(())
    }

    /// Draw a cubic Bézier curve.
    pub fn draw_bezier_cubic(
        &mut self, p0: Point, c0: Point, c1: Point, p1: Point, steps: u32,
    ) -> Result<()> {
        let steps = steps.max(2) as usize;
        let mut prev = p0;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let x = lerp3(p0.x as f32, c0.x as f32, c1.x as f32, p1.x as f32, t);
            let y = lerp3(p0.y as f32, c0.y as f32, c1.y as f32, p1.y as f32, t);
            let cur = Point::new(x.round() as i32, y.round() as i32);
            self.draw_line_points(prev, cur)?;
            prev = cur;
        }
        Ok(())
    }

    // ── Gradient fill ─────────────────────────────────────────────────────────

    /// Fill a rectangle with a horizontal gradient from `left` to `right`.
    pub fn fill_gradient_h(&mut self, rect: Rect, left: Color, right: Color) -> Result<()> {
        let w = rect.width as f32;
        for x in rect.left()..rect.right() {
            let t = (x - rect.left()) as f32 / w;
            let c = left.lerp(right, t);
            let orig = self.draw_color;
            self.draw_color = c;
            self.draw_line(x, rect.top(), x, rect.bottom() - 1)?;
            self.draw_color = orig;
        }
        Ok(())
    }

    /// Fill a rectangle with a vertical gradient from `top` to `bottom`.
    pub fn fill_gradient_v(&mut self, rect: Rect, top: Color, bottom: Color) -> Result<()> {
        let h = rect.height as f32;
        for y in rect.top()..rect.bottom() {
            let t = (y - rect.top()) as f32 / h;
            let c = top.lerp(bottom, t);
            let orig = self.draw_color;
            self.draw_color = c;
            self.draw_line(rect.left(), y, rect.right() - 1, y)?;
            self.draw_color = orig;
        }
        Ok(())
    }

    // ── Pixel blit ────────────────────────────────────────────────────────────

    /// Copy a raw RGBA pixel buffer onto the canvas at `dst_x, dst_y`.
    pub fn blit_pixels(
        &mut self,
        pixels: &[u8],
        src_width: u32,
        src_height: u32,
        dst_x: i32,
        dst_y: i32,
    ) -> Result<()> {
        if pixels.len() < (src_width * src_height * 4) as usize {
            return Err(MiniSDLError::Generic("pixel buffer too small".into()));
        }
        for y in 0..src_height as i32 {
            for x in 0..src_width as i32 {
                let idx = ((y as u32 * src_width + x as u32) * 4) as usize;
                let c = Color::rgba(pixels[idx], pixels[idx+1], pixels[idx+2], pixels[idx+3]);
                self.put_pixel_color(dst_x + x, dst_y + y, c);
            }
        }
        Ok(())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn bresenham(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut x = x0;
    let mut y = y0;
    let mut err = dx - dy;
    loop {
        plot(x, y);
        if x == x1 && y == y1 { break; }
        let e2 = err * 2;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 <  dx { err += dx; y += sy; }
    }
}

fn lerp2(a: f32, b: f32, c: f32, t: f32) -> f32 {
    let mt = 1.0 - t;
    mt * mt * a + 2.0 * mt * t * b + t * t * c
}

fn lerp3(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let mt = 1.0 - t;
    mt*mt*mt*a + 3.0*mt*mt*t*b + 3.0*mt*t*t*c + t*t*t*d
}
