# Geometry Module Design

Core geometry infrastructure for gesso, an immediate-mode UI framework.

## Goals

- Provide layout-focused geometry primitives for 2D UI
- Leverage glamour/glam for the heavy lifting
- Type-safe coordinate spaces (Logical vs Device) to catch HiDPI bugs
- Expose glamour's `Unit` trait so app developers can define custom spaces (e.g., canvas editors)
- Idiomatic Rust: newtypes, zero-cost abstractions

## Non-Goals

- Path/shape primitives (future module)
- Rendering concerns (unit-quad transforms are renderer-internal)
- f64 precision (f32 sufficient for UI)

## Module Structure

```
crates/
├── gesso/           # main crate, re-exports
└── gesso_core/      # core types
    └── src/
        ├── lib.rs
        └── geometry.rs   # all geometry types
```

Dependencies: `glam = "0.32"`, `glamour = "0.18"`

## Types

### Unit Markers

```rust
pub struct LogicalPixels;
pub struct DevicePixels;

impl glamour::Unit for LogicalPixels {
    type Scalar = f32;
}

impl glamour::Unit for DevicePixels {
    type Scalar = f32;
}
```

### Type Aliases

```rust
// Logical space (user-facing, DPI-independent)
pub type Point = glamour::Point2<LogicalPixels>;
pub type Size = glamour::Size2<LogicalPixels>;
pub type Rect = glamour::Rect<LogicalPixels>;
pub type Vector = glamour::Vector2<LogicalPixels>;

// Device space (physical pixels)
pub type DevicePoint = glamour::Point2<DevicePixels>;
pub type DeviceSize = glamour::Size2<DevicePixels>;
pub type DeviceRect = glamour::Rect<DevicePixels>;
```

### ScaleFactor

Newtype for HiDPI scale factor, enables logical↔device conversion.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScaleFactor(pub f32);

impl ScaleFactor {
    pub fn scale_point(&self, p: Point) -> DevicePoint;
    pub fn scale_size(&self, s: Size) -> DeviceSize;
    pub fn scale_rect(&self, r: Rect) -> DeviceRect;

    pub fn unscale_point(&self, p: DevicePoint) -> Point;
    pub fn unscale_size(&self, s: DeviceSize) -> Size;
    pub fn unscale_rect(&self, r: DeviceRect) -> Rect;
}
```

### Edges

For padding, margin, border widths. Follows CSS order: top, right, bottom, left.

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Copy> Edges<T> {
    pub fn all(value: T) -> Self;
    pub fn symmetric(vertical: T, horizontal: T) -> Self;
}

impl<T: Copy + Add<Output = T>> Edges<T> {
    pub fn horizontal(&self) -> T;  // left + right
    pub fn vertical(&self) -> T;    // top + bottom
}
```

### Corners

For border radii.

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Corners<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_right: T,
    pub bottom_left: T,
}

impl<T: Copy> Corners<T> {
    pub fn all(value: T) -> Self;
    pub fn top_bottom(top: T, bottom: T) -> Self;
}
```

### Axis

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn invert(self) -> Self;
}
```

## Public API

The geometry module exports:
- Unit markers: `LogicalPixels`, `DevicePixels`
- Type aliases: `Point`, `Size`, `Rect`, `Vector`, `DevicePoint`, `DeviceSize`, `DeviceRect`
- Newtypes: `ScaleFactor`, `Edges<T>`, `Corners<T>`, `Axis`
- Re-export `glamour::Unit` trait for custom spaces

## Future Considerations

- Path/shape module for sophisticated drawing (curves, arcs, boolean operations)
- Transform utilities if needed beyond glamour's `Transform2`
- Additional unit types if layout system requires (e.g., `Rems`)
