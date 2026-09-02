//! Core geometry primitives for motif.

// Re-export Unit trait for users defining custom coordinate spaces
pub use glamour::Unit;

// Re-export glamour geometry traits so callers don't need `use glamour::*`
pub use glamour::{Contains, Intersection, Union};

/// Logical pixels - DPI-independent coordinate space.
pub struct LogicalPixels;

impl glamour::Unit for LogicalPixels {
    type Scalar = f32;
}

/// Device pixels - physical pixel coordinate space.
pub struct DevicePixels;

impl glamour::Unit for DevicePixels {
    type Scalar = f32;
}

// Logical space type aliases
pub type Point = glamour::Point2<LogicalPixels>;
pub type Size = glamour::Size2<LogicalPixels>;
pub type Rect = glamour::Rect<LogicalPixels>;
pub type Vector = glamour::Vector2<LogicalPixels>;

// Device space type aliases
pub type DevicePoint = glamour::Point2<DevicePixels>;
pub type DeviceSize = glamour::Size2<DevicePixels>;
pub type DeviceRect = glamour::Rect<DevicePixels>;

/// Scale factor for converting between logical and device pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaleFactor(pub f32);

impl ScaleFactor {
    pub fn scale_point(&self, p: Point) -> DevicePoint {
        DevicePoint::new(p.x * self.0, p.y * self.0)
    }

    pub fn scale_size(&self, s: Size) -> DeviceSize {
        DeviceSize::new(s.width * self.0, s.height * self.0)
    }

    pub fn scale_rect(&self, r: Rect) -> DeviceRect {
        DeviceRect::new(self.scale_point(r.origin), self.scale_size(r.size))
    }

    pub fn unscale_point(&self, p: DevicePoint) -> Point {
        Point::new(p.x / self.0, p.y / self.0)
    }

    pub fn unscale_size(&self, s: DeviceSize) -> Size {
        Size::new(s.width / self.0, s.height / self.0)
    }

    pub fn unscale_rect(&self, r: DeviceRect) -> Rect {
        Rect::new(self.unscale_point(r.origin), self.unscale_size(r.size))
    }
}

/// Edge values for padding, margin, border widths.
/// Follows CSS order: top, right, bottom, left.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Copy> Edges<T> {
    pub fn all(value: T) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn symmetric(vertical: T, horizontal: T) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

impl<T: Copy + std::ops::Add<Output = T>> Edges<T> {
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }
}

/// Corner values for border radii.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T: Copy> Corners<T> {
    pub fn all(value: T) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    pub fn top_bottom(top: T, bottom: T) -> Self {
        Self {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
        }
    }
}

/// Axis in 2D space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn invert(self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Contains trait ───────────────────────────────────────────────────────

    #[test]
    fn rect_contains_interior_point() {
        let r = Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 100.0));
        // Point well inside the rect
        assert!(r.contains(&Point::new(50.0, 50.0)));
    }

    #[test]
    fn rect_contains_min_corner_inclusive() {
        let r = Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 100.0));
        // Min corner is inclusive
        assert!(r.contains(&Point::new(10.0, 10.0)));
    }

    #[test]
    fn rect_does_not_contain_exterior_point() {
        let r = Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 100.0));
        assert!(!r.contains(&Point::new(9.0, 50.0)));
        assert!(!r.contains(&Point::new(50.0, 9.0)));
        assert!(!r.contains(&Point::new(200.0, 50.0)));
    }

    // ── Intersection trait ───────────────────────────────────────────────────

    #[test]
    fn overlapping_rects_produce_intersection() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        let b = Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0));
        let result = a.intersection(&b).expect("overlapping rects should intersect");
        assert_eq!(result.origin.x, 50.0);
        assert_eq!(result.origin.y, 50.0);
        assert_eq!(result.size.width, 50.0);
        assert_eq!(result.size.height, 50.0);
    }

    #[test]
    fn disjoint_rects_produce_no_intersection() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let b = Rect::new(Point::new(20.0, 0.0), Size::new(10.0, 10.0));
        assert!(a.intersection(&b).is_none());
    }

    #[test]
    fn rect_intersects_overlapping_rect() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        let b = Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 100.0));
        assert!(a.intersects(&b));
    }

    #[test]
    fn rect_does_not_intersect_disjoint_rect() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let b = Rect::new(Point::new(20.0, 0.0), Size::new(10.0, 10.0));
        assert!(!a.intersects(&b));
    }

    #[test]
    fn device_rect_intersection_works() {
        // Confirms the trait re-export is generic over unit type
        let a = DeviceRect::new(DevicePoint::new(0.0, 0.0), DeviceSize::new(100.0, 100.0));
        let b = DeviceRect::new(DevicePoint::new(50.0, 50.0), DeviceSize::new(100.0, 100.0));
        let result = a.intersection(&b).expect("device rects should intersect");
        assert_eq!(result.origin.x, 50.0);
        assert_eq!(result.origin.y, 50.0);
    }

    // ── Union trait ──────────────────────────────────────────────────────────

    #[test]
    fn rect_union_produces_bounding_box() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(50.0, 50.0));
        let b = Rect::new(Point::new(40.0, 40.0), Size::new(50.0, 50.0));
        let u = a.union(b);
        assert_eq!(u.origin.x, 0.0);
        assert_eq!(u.origin.y, 0.0);
        assert_eq!(u.size.width, 90.0);
        assert_eq!(u.size.height, 90.0);
    }

    #[test]
    fn rect_union_with_disjoint_rect() {
        let a = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let b = Rect::new(Point::new(90.0, 90.0), Size::new(10.0, 10.0));
        let u = a.union(b);
        assert_eq!(u.origin.x, 0.0);
        assert_eq!(u.origin.y, 0.0);
        assert_eq!(u.size.width, 100.0);
        assert_eq!(u.size.height, 100.0);
    }

    // ── Scale factor ─────────────────────────────────────────────────────────

    #[test]
    fn scale_factor_roundtrip() {
        let scale = ScaleFactor(2.0);
        let original = Point::new(10.0, 20.0);
        let scaled = scale.scale_point(original);
        let back = scale.unscale_point(scaled);
        assert_eq!(original.x, back.x);
        assert_eq!(original.y, back.y);
    }

    #[test]
    fn scale_factor_rect_roundtrip() {
        let scale = ScaleFactor(1.5);
        let original = Rect::new(Point::new(5.0, 10.0), Size::new(100.0, 200.0));
        let scaled = scale.scale_rect(original);
        let back = scale.unscale_rect(scaled);
        assert_eq!(original.origin.x, back.origin.x);
        assert_eq!(original.origin.y, back.origin.y);
        assert_eq!(original.size.width, back.size.width);
        assert_eq!(original.size.height, back.size.height);
    }

    #[test]
    fn edges_sums() {
        let edges = Edges {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        };
        assert_eq!(edges.horizontal(), 6.0); // 4 + 2
        assert_eq!(edges.vertical(), 4.0); // 1 + 3
    }
}
