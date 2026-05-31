//! Rectangle and Point primitives.

/// A 2-D integer point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Euclidean distance to another point.
    pub fn distance(self, other: Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Manhattan distance.
    pub fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn offset(self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// An axis-aligned rectangle with integer coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    #[inline]
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Construct from two corner points.
    pub fn from_corners(a: Point, b: Point) -> Self {
        let x = a.x.min(b.x);
        let y = a.y.min(b.y);
        let w = (a.x - b.x).unsigned_abs();
        let h = (a.y - b.y).unsigned_abs();
        Self::new(x, y, w, h)
    }

    /// Centre point.
    pub fn center(self) -> Point {
        Point::new(
            self.x + self.width as i32 / 2,
            self.y + self.height as i32 / 2,
        )
    }

    pub fn left(self)   -> i32 { self.x }
    pub fn right(self)  -> i32 { self.x + self.width as i32 }
    pub fn top(self)    -> i32 { self.y }
    pub fn bottom(self) -> i32 { self.y + self.height as i32 }

    /// Returns `true` if the point is inside (or on the boundary of) this rectangle.
    pub fn contains(self, p: Point) -> bool {
        p.x >= self.left()
            && p.x <= self.right()
            && p.y >= self.top()
            && p.y <= self.bottom()
    }

    /// Returns `true` if this rectangle overlaps `other`.
    pub fn intersects(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    /// Compute the intersection of two rectangles, if any.
    pub fn intersection(self, other: Rect) -> Option<Rect> {
        let x = self.left().max(other.left());
        let y = self.top().max(other.top());
        let right  = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        if right > x && bottom > y {
            Some(Rect::new(x, y, (right - x) as u32, (bottom - y) as u32))
        } else {
            None
        }
    }

    /// Smallest rectangle that contains both `self` and `other`.
    pub fn union(self, other: Rect) -> Rect {
        let x = self.left().min(other.left());
        let y = self.top().min(other.top());
        let right  = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        Rect::new(x, y, (right - x) as u32, (bottom - y) as u32)
    }

    /// Inset (positive) or outset (negative) the rectangle on all sides.
    pub fn inset(self, amount: i32) -> Rect {
        Rect::new(
            self.x + amount,
            self.y + amount,
            (self.width as i32 - amount * 2).max(0) as u32,
            (self.height as i32 - amount * 2).max(0) as u32,
        )
    }

    /// Translate the rectangle.
    pub fn offset(self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    pub fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect({}, {}, {}×{})", self.x, self.y, self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_overlap() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        assert_eq!(a.intersection(b), Some(Rect::new(5, 5, 5, 5)));
    }

    #[test]
    fn no_intersection() {
        let a = Rect::new(0, 0, 5, 5);
        let b = Rect::new(10, 10, 5, 5);
        assert_eq!(a.intersection(b), None);
    }

    #[test]
    fn contains_point() {
        let r = Rect::new(0, 0, 100, 100);
        assert!(r.contains(Point::new(50, 50)));
        assert!(!r.contains(Point::new(150, 50)));
    }
}
