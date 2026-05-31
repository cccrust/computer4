//! RGBA colour representation.

/// A 32-bit RGBA colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Create a fully-opaque colour from red, green, and blue components.
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a colour with an explicit alpha channel.
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parse a `#RRGGBB` or `#RRGGBBAA` hex string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a))
            }
            _ => None,
        }
    }

    /// Create from a packed `0xRRGGBBAA` u32.
    #[inline]
    pub const fn from_u32(v: u32) -> Self {
        Self {
            r: ((v >> 24) & 0xFF) as u8,
            g: ((v >> 16) & 0xFF) as u8,
            b: ((v >> 8) & 0xFF) as u8,
            a: (v & 0xFF) as u8,
        }
    }

    /// Pack into `0xRRGGBBAA`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | (self.a as u32)
    }

    // ── Alpha blending ────────────────────────────────────────────────────────

    /// Alpha-blend `self` (source) over `dst` (destination).
    pub fn blend_over(self, dst: Color) -> Color {
        if self.a == 255 {
            return self;
        }
        if self.a == 0 {
            return dst;
        }
        let sa = self.a as u32;
        let da = dst.a as u32;
        let out_a = sa + da * (255 - sa) / 255;
        if out_a == 0 {
            return Color::TRANSPARENT;
        }
        let blend = |s: u8, d: u8| -> u8 {
            ((s as u32 * sa + d as u32 * da * (255 - sa) / 255) / out_a) as u8
        };
        Color::rgba(
            blend(self.r, dst.r),
            blend(self.g, dst.g),
            blend(self.b, dst.b),
            out_a as u8,
        )
    }

    /// Linearly interpolate between two colours by `t ∈ [0.0, 1.0]`.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let lerp_u8 = |a: u8, b: u8| -> u8 { (a as f32 + (b as f32 - a as f32) * t) as u8 };
        Color::rgba(
            lerp_u8(self.r, other.r),
            lerp_u8(self.g, other.g),
            lerp_u8(self.b, other.b),
            lerp_u8(self.a, other.a),
        )
    }

    /// Return a copy with the given alpha value.
    #[inline]
    pub const fn with_alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }

    // ── Named constants ───────────────────────────────────────────────────────

    pub const BLACK: Self       = Self::rgb(0,   0,   0);
    pub const WHITE: Self       = Self::rgb(255, 255, 255);
    pub const RED: Self         = Self::rgb(255, 0,   0);
    pub const GREEN: Self       = Self::rgb(0,   255, 0);
    pub const BLUE: Self        = Self::rgb(0,   0,   255);
    pub const YELLOW: Self      = Self::rgb(255, 255, 0);
    pub const CYAN: Self        = Self::rgb(0,   255, 255);
    pub const MAGENTA: Self     = Self::rgb(255, 0,   255);
    pub const ORANGE: Self      = Self::rgb(255, 165, 0);
    pub const PURPLE: Self      = Self::rgb(128, 0,   128);
    pub const GRAY: Self        = Self::rgb(128, 128, 128);
    pub const LIGHT_GRAY: Self  = Self::rgb(192, 192, 192);
    pub const DARK_GRAY: Self   = Self::rgb(64,  64,  64);
    pub const TRANSPARENT: Self = Self::rgba(0,  0,   0,   0);
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_parse() {
        assert_eq!(Color::from_hex("#FF0000"), Some(Color::RED));
        assert_eq!(Color::from_hex("00FF00FF"), Some(Color::rgba(0, 255, 0, 255)));
        assert_eq!(Color::from_hex("ZZZ"), None);
    }

    #[test]
    fn round_trip_u32() {
        let c = Color::rgba(10, 20, 30, 40);
        assert_eq!(Color::from_u32(c.to_u32()), c);
    }

    #[test]
    fn blend_opaque_source() {
        assert_eq!(Color::RED.blend_over(Color::BLUE), Color::RED);
    }
}
