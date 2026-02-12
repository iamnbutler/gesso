# Geometry Module Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement core geometry primitives for gesso's 2D UI framework.

**Architecture:** Thin wrappers around glamour with LogicalPixels/DevicePixels unit markers. Layout-focused types (Edges, Corners) are generic over scalar. ScaleFactor enables HiDPI conversion.

**Tech Stack:** Rust, glam 0.32, glamour 0.18

---

## Task 1: Create gesso_core Crate with Dependencies

**Files:**
- Create: `crates/gesso_core/Cargo.toml`
- Create: `crates/gesso_core/src/lib.rs`
- Modify: `Cargo.toml` (workspace)

**Step 1: Create the crate directory structure**

```bash
mkdir -p crates/gesso_core/src
```

**Step 2: Create Cargo.toml for gesso_core**

Create `crates/gesso_core/Cargo.toml`:

```toml
[package]
name = "gesso_core"
version.workspace = true
edition.workspace = true

[dependencies]
glam = "0.32"
glamour = "0.18"

[dev-dependencies]
```

**Step 3: Create lib.rs with geometry module stub**

Create `crates/gesso_core/src/lib.rs`:

```rust
pub mod geometry;
```

**Step 4: Create empty geometry.rs**

Create `crates/gesso_core/src/geometry.rs`:

```rust
//! Core geometry primitives for gesso.
```

**Step 5: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with no errors

**Step 6: Commit**

```bash
git add crates/gesso_core
git commit -m "Add gesso_core crate with glam/glamour deps"
```

---

## Task 2: Unit Markers (LogicalPixels, DevicePixels)

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write the failing test for LogicalPixels**

Add to `crates/gesso_core/src/geometry.rs`:

```rust
//! Core geometry primitives for gesso.

#[cfg(test)]
mod tests {
    use super::*;
    use glamour::Unit;

    #[test]
    fn logical_pixels_is_f32_unit() {
        // LogicalPixels should implement Unit with f32 scalar
        fn assert_unit<U: Unit<Scalar = f32>>() {}
        assert_unit::<LogicalPixels>();
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `LogicalPixels`"

**Step 3: Write minimal implementation**

Add above the tests module in `crates/gesso_core/src/geometry.rs`:

```rust
//! Core geometry primitives for gesso.

/// Logical pixels - DPI-independent coordinate space.
pub struct LogicalPixels;

impl glamour::Unit for LogicalPixels {
    type Scalar = f32;
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Write the failing test for DevicePixels**

Add to the tests module:

```rust
    #[test]
    fn device_pixels_is_f32_unit() {
        fn assert_unit<U: Unit<Scalar = f32>>() {}
        assert_unit::<DevicePixels>();
    }
```

**Step 6: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `DevicePixels`"

**Step 7: Implement DevicePixels**

Add after LogicalPixels:

```rust
/// Device pixels - physical pixel coordinate space.
pub struct DevicePixels;

impl glamour::Unit for DevicePixels {
    type Scalar = f32;
}
```

**Step 8: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 9: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add LogicalPixels and DevicePixels unit markers"
```

---

## Task 3: Type Aliases (Point, Size, Rect, Vector)

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write failing test for Point alias**

Add to tests module:

```rust
    #[test]
    fn point_is_logical_point2() {
        let p: Point = glamour::Point2::new(10.0, 20.0);
        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 20.0);
    }
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `Point`"

**Step 3: Implement type aliases**

Add after the unit markers:

```rust
// Logical space type aliases
pub type Point = glamour::Point2<LogicalPixels>;
pub type Size = glamour::Size2<LogicalPixels>;
pub type Rect = glamour::Rect<LogicalPixels>;
pub type Vector = glamour::Vector2<LogicalPixels>;

// Device space type aliases
pub type DevicePoint = glamour::Point2<DevicePixels>;
pub type DeviceSize = glamour::Size2<DevicePixels>;
pub type DeviceRect = glamour::Rect<DevicePixels>;
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Write test for DevicePoint alias**

Add to tests module:

```rust
    #[test]
    fn device_point_is_device_point2() {
        let p: DevicePoint = glamour::Point2::new(100.0, 200.0);
        assert_eq!(p.x, 100.0);
        assert_eq!(p.y, 200.0);
    }
```

**Step 6: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS (already implemented)

**Step 7: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add Point, Size, Rect, Vector type aliases"
```

---

## Task 4: ScaleFactor Newtype

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write failing test for ScaleFactor construction**

Add to tests module:

```rust
    #[test]
    fn scale_factor_construction() {
        let scale = ScaleFactor(2.0);
        assert_eq!(scale.0, 2.0);
    }
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `ScaleFactor`"

**Step 3: Implement ScaleFactor struct**

Add after type aliases:

```rust
/// Scale factor for converting between logical and device pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaleFactor(pub f32);
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Write failing test for scale_point**

Add to tests module:

```rust
    #[test]
    fn scale_factor_scales_point() {
        let scale = ScaleFactor(2.0);
        let logical = Point::new(10.0, 20.0);
        let device = scale.scale_point(logical);
        assert_eq!(device.x, 20.0);
        assert_eq!(device.y, 40.0);
    }
```

**Step 6: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "no method named `scale_point`"

**Step 7: Implement scale methods**

Add impl block for ScaleFactor:

```rust
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
```

**Step 8: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 9: Write test for unscale_point**

Add to tests module:

```rust
    #[test]
    fn scale_factor_unscales_point() {
        let scale = ScaleFactor(2.0);
        let device = DevicePoint::new(20.0, 40.0);
        let logical = scale.unscale_point(device);
        assert_eq!(logical.x, 10.0);
        assert_eq!(logical.y, 20.0);
    }
```

**Step 10: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 11: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add ScaleFactor with scale/unscale methods"
```

---

## Task 5: Edges<T>

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write failing test for Edges construction**

Add to tests module:

```rust
    #[test]
    fn edges_all_constructor() {
        let edges: Edges<f32> = Edges::all(10.0);
        assert_eq!(edges.top, 10.0);
        assert_eq!(edges.right, 10.0);
        assert_eq!(edges.bottom, 10.0);
        assert_eq!(edges.left, 10.0);
    }
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `Edges`"

**Step 3: Implement Edges struct**

Add after ScaleFactor:

```rust
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
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Write failing test for symmetric constructor**

Add to tests module:

```rust
    #[test]
    fn edges_symmetric_constructor() {
        let edges: Edges<f32> = Edges::symmetric(10.0, 20.0);
        assert_eq!(edges.top, 10.0);
        assert_eq!(edges.bottom, 10.0);
        assert_eq!(edges.left, 20.0);
        assert_eq!(edges.right, 20.0);
    }
```

**Step 6: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "no function or associated item named `symmetric`"

**Step 7: Implement symmetric**

Add to the `impl<T: Copy> Edges<T>` block:

```rust
    pub fn symmetric(vertical: T, horizontal: T) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
```

**Step 8: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 9: Write failing test for horizontal/vertical sums**

Add to tests module:

```rust
    #[test]
    fn edges_horizontal_vertical_sums() {
        let edges: Edges<f32> = Edges {
            top: 1.0,
            right: 2.0,
            bottom: 3.0,
            left: 4.0,
        };
        assert_eq!(edges.horizontal(), 6.0); // left + right
        assert_eq!(edges.vertical(), 4.0);   // top + bottom
    }
```

**Step 10: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "no method named `horizontal`"

**Step 11: Implement horizontal/vertical**

Add new impl block:

```rust
impl<T: Copy + std::ops::Add<Output = T>> Edges<T> {
    pub fn horizontal(&self) -> T {
        self.left + self.right
    }

    pub fn vertical(&self) -> T {
        self.top + self.bottom
    }
}
```

**Step 12: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 13: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add Edges<T> with constructors and sum methods"
```

---

## Task 6: Corners<T>

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write failing test for Corners::all**

Add to tests module:

```rust
    #[test]
    fn corners_all_constructor() {
        let corners: Corners<f32> = Corners::all(5.0);
        assert_eq!(corners.top_left, 5.0);
        assert_eq!(corners.top_right, 5.0);
        assert_eq!(corners.bottom_right, 5.0);
        assert_eq!(corners.bottom_left, 5.0);
    }
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `Corners`"

**Step 3: Implement Corners struct**

Add after Edges:

```rust
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
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Write failing test for top_bottom constructor**

Add to tests module:

```rust
    #[test]
    fn corners_top_bottom_constructor() {
        let corners: Corners<f32> = Corners::top_bottom(10.0, 5.0);
        assert_eq!(corners.top_left, 10.0);
        assert_eq!(corners.top_right, 10.0);
        assert_eq!(corners.bottom_left, 5.0);
        assert_eq!(corners.bottom_right, 5.0);
    }
```

**Step 6: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "no function or associated item named `top_bottom`"

**Step 7: Implement top_bottom**

Add to the `impl<T: Copy> Corners<T>` block:

```rust
    pub fn top_bottom(top: T, bottom: T) -> Self {
        Self {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
        }
    }
```

**Step 8: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 9: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add Corners<T> with constructors"
```

---

## Task 7: Axis Enum

**Files:**
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Write failing test for Axis::invert**

Add to tests module:

```rust
    #[test]
    fn axis_invert() {
        assert_eq!(Axis::Horizontal.invert(), Axis::Vertical);
        assert_eq!(Axis::Vertical.invert(), Axis::Horizontal);
    }
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p gesso_core`
Expected: FAIL with "cannot find type `Axis`"

**Step 3: Implement Axis**

Add after Corners:

```rust
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p gesso_core`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/gesso_core/src/geometry.rs
git commit -m "Add Axis enum"
```

---

## Task 8: Public Exports and Re-exports

**Files:**
- Modify: `crates/gesso_core/src/lib.rs`
- Modify: `crates/gesso_core/src/geometry.rs`

**Step 1: Add glamour::Unit re-export**

Add at top of `geometry.rs`:

```rust
//! Core geometry primitives for gesso.

// Re-export Unit trait for users defining custom coordinate spaces
pub use glamour::Unit;
```

**Step 2: Update lib.rs to re-export geometry types**

Replace contents of `crates/gesso_core/src/lib.rs`:

```rust
pub mod geometry;

pub use geometry::*;
```

**Step 3: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with no errors

**Step 4: Commit**

```bash
git add crates/gesso_core/src/lib.rs crates/gesso_core/src/geometry.rs
git commit -m "Export geometry types from gesso_core"
```

---

## Task 9: Wire Up gesso Crate

**Files:**
- Modify: `crates/gesso/Cargo.toml`
- Modify: `crates/gesso/src/main.rs`

**Step 1: Add gesso_core dependency**

Update `crates/gesso/Cargo.toml`:

```toml
[package]
name = "gesso"
version.workspace = true
edition.workspace = true

[dependencies]
gesso_core = { path = "../gesso_core" }
```

**Step 2: Re-export from gesso**

Replace `crates/gesso/src/main.rs` with `crates/gesso/src/lib.rs`:

```bash
mv crates/gesso/src/main.rs crates/gesso/src/lib.rs
```

Then replace contents of `crates/gesso/src/lib.rs`:

```rust
pub use gesso_core::*;
```

**Step 3: Verify it compiles**

Run: `cargo check -p gesso`
Expected: Compiles with no errors

**Step 4: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 5: Commit**

```bash
git add crates/gesso
git commit -m "Wire gesso crate to re-export gesso_core"
```

---

## Summary

After completing all tasks you will have:

- `gesso_core` crate with geometry module
- `LogicalPixels` and `DevicePixels` unit markers implementing `glamour::Unit`
- Type aliases: `Point`, `Size`, `Rect`, `Vector` (and Device variants)
- `ScaleFactor` newtype with scale/unscale methods
- `Edges<T>` with `all`, `symmetric`, `horizontal`, `vertical`
- `Corners<T>` with `all`, `top_bottom`
- `Axis` enum with `invert`
- All types exported from both `gesso_core` and `gesso` crates
