//! Core geometry primitives for motif.

// Re-export Unit trait for users defining custom coordinate spaces
pub use glamour::Unit;

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

/// Extension methods for [`Rect`] (logical-space rectangles).
///
/// Import this trait to gain `inset`, `inset_edges`, `outset`, and `outset_edges` on [`Rect`].
///
/// # Examples
///
/// ```rust,ignore
/// use motif_core::{Rect, Point, Size, Edges, RectExt};
///
/// let bounds = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 80.0));
/// let content = bounds.inset(8.0);
/// // content.origin == (8, 8), content.size == (84, 64)
///
/// let card_body = bounds.inset_edges(Edges { top: 16.0, right: 8.0, bottom: 8.0, left: 8.0 });
/// ```
pub trait RectExt {
    /// Returns a new rect shrunk by `amount` on all four sides.
    ///
    /// The origin shifts right and down by `amount`; width and height each shrink by
    /// `2 * amount`. Both dimensions clamp to zero rather than going negative.
    fn inset(self, amount: f32) -> Self;

    /// Returns a new rect shrunk by the per-edge `edges` amounts.
    ///
    /// Origin shifts by `(left, top)`. Width shrinks by `left + right`; height by
    /// `top + bottom`. Both dimensions clamp to zero rather than going negative.
    fn inset_edges(self, edges: Edges<f32>) -> Self;

    /// Returns a new rect grown by `amount` on all four sides.
    ///
    /// The origin shifts left and up by `amount`; width and height each grow by
    /// `2 * amount`.
    fn outset(self, amount: f32) -> Self;

    /// Returns a new rect grown by the per-edge `edges` amounts.
    ///
    /// Origin shifts by `(-left, -top)`. Width grows by `left + right`; height by
    /// `top + bottom`.
    fn outset_edges(self, edges: Edges<f32>) -> Self;
}

impl RectExt for Rect {
    fn inset(self, amount: f32) -> Self {
        self.inset_edges(Edges::all(amount))
    }

    fn inset_edges(self, edges: Edges<f32>) -> Self {
        Rect::new(
            Point::new(self.origin.x + edges.left, self.origin.y + edges.top),
            Size::new(
                (self.size.width - edges.horizontal()).max(0.0),
                (self.size.height - edges.vertical()).max(0.0),
            ),
        )
    }

    fn outset(self, amount: f32) -> Self {
        self.outset_edges(Edges::all(amount))
    }

    fn outset_edges(self, edges: Edges<f32>) -> Self {
        Rect::new(
            Point::new(self.origin.x - edges.left, self.origin.y - edges.top),
            Size::new(
                self.size.width + edges.horizontal(),
                self.size.height + edges.vertical(),
            ),
        )
    }
}

/// Extension methods for [`DeviceRect`] (device-space rectangles).
///
/// Mirror of [`RectExt`] for the device-pixel coordinate space. Useful when computing
/// device-space clip rects with per-edge insets.
pub trait DeviceRectExt {
    /// Returns a new device rect shrunk by `amount` on all four sides.
    fn inset(self, amount: f32) -> Self;
    /// Returns a new device rect shrunk by the per-edge `edges` amounts.
    fn inset_edges(self, edges: Edges<f32>) -> Self;
    /// Returns a new device rect grown by `amount` on all four sides.
    fn outset(self, amount: f32) -> Self;
    /// Returns a new device rect grown by the per-edge `edges` amounts.
    fn outset_edges(self, edges: Edges<f32>) -> Self;
}

impl DeviceRectExt for DeviceRect {
    fn inset(self, amount: f32) -> Self {
        self.inset_edges(Edges::all(amount))
    }

    fn inset_edges(self, edges: Edges<f32>) -> Self {
        DeviceRect::new(
            DevicePoint::new(self.origin.x + edges.left, self.origin.y + edges.top),
            DeviceSize::new(
                (self.size.width - edges.horizontal()).max(0.0),
                (self.size.height - edges.vertical()).max(0.0),
            ),
        )
    }

    fn outset(self, amount: f32) -> Self {
        self.outset_edges(Edges::all(amount))
    }

    fn outset_edges(self, edges: Edges<f32>) -> Self {
        DeviceRect::new(
            DevicePoint::new(self.origin.x - edges.left, self.origin.y - edges.top),
            DeviceSize::new(
                self.size.width + edges.horizontal(),
                self.size.height + edges.vertical(),
            ),
        )
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

    // --- RectExt tests ---

    #[test]
    fn rect_inset_uniform_shrinks_all_sides() {
        let r = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 80.0));
        let inner = r.inset(8.0);
        assert_eq!(inner.origin.x, 8.0);
        assert_eq!(inner.origin.y, 8.0);
        assert_eq!(inner.size.width, 84.0);
        assert_eq!(inner.size.height, 64.0);
    }

    #[test]
    fn rect_inset_edges_per_side() {
        let r = Rect::new(Point::new(10.0, 10.0), Size::new(200.0, 100.0));
        let inner = r.inset_edges(Edges {
            top: 5.0,
            right: 10.0,
            bottom: 15.0,
            left: 20.0,
        });
        assert_eq!(inner.origin.x, 30.0); // 10 + left(20)
        assert_eq!(inner.origin.y, 15.0); // 10 + top(5)
        assert_eq!(inner.size.width, 170.0); // 200 - (20+10)
        assert_eq!(inner.size.height, 80.0); // 100 - (5+15)
    }

    #[test]
    fn rect_inset_larger_than_size_clamps_to_zero() {
        let r = Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let inner = r.inset(20.0);
        // size must not go negative
        assert_eq!(inner.size.width, 0.0);
        assert_eq!(inner.size.height, 0.0);
    }

    #[test]
    fn rect_outset_uniform_grows_all_sides() {
        let r = Rect::new(Point::new(10.0, 10.0), Size::new(80.0, 60.0));
        let outer = r.outset(5.0);
        assert_eq!(outer.origin.x, 5.0); // 10 - 5
        assert_eq!(outer.origin.y, 5.0); // 10 - 5
        assert_eq!(outer.size.width, 90.0); // 80 + 10
        assert_eq!(outer.size.height, 70.0); // 60 + 10
    }

    #[test]
    fn rect_outset_edges_per_side() {
        let r = Rect::new(Point::new(20.0, 20.0), Size::new(100.0, 50.0));
        let outer = r.outset_edges(Edges {
            top: 2.0,
            right: 4.0,
            bottom: 6.0,
            left: 8.0,
        });
        assert_eq!(outer.origin.x, 12.0); // 20 - left(8)
        assert_eq!(outer.origin.y, 18.0); // 20 - top(2)
        assert_eq!(outer.size.width, 112.0); // 100 + (8+4)
        assert_eq!(outer.size.height, 58.0); // 50 + (2+6)
    }

    // --- DeviceRectExt tests ---

    #[test]
    fn device_rect_inset_works() {
        let r = DeviceRect::new(DevicePoint::new(0.0, 0.0), DeviceSize::new(200.0, 100.0));
        let inner = r.inset(10.0);
        assert_eq!(inner.origin.x, 10.0);
        assert_eq!(inner.origin.y, 10.0);
        assert_eq!(inner.size.width, 180.0);
        assert_eq!(inner.size.height, 80.0);
    }

    #[test]
    fn device_rect_outset_works() {
        let r = DeviceRect::new(DevicePoint::new(10.0, 10.0), DeviceSize::new(50.0, 30.0));
        let outer = r.outset(4.0);
        assert_eq!(outer.origin.x, 6.0);
        assert_eq!(outer.origin.y, 6.0);
        assert_eq!(outer.size.width, 58.0);
        assert_eq!(outer.size.height, 38.0);
    }
}
