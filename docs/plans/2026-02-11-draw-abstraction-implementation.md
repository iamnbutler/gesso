# Draw Abstraction Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement Scene, DrawContext, and Renderer trait for gesso's draw pipeline.

**Architecture:** Painter's stack model with closure-scoped transform/clip. Scene holds typed primitive vecs in device pixels. Renderer trait enables backend abstraction with static dispatch.

**Tech Stack:** Rust, palette (color), glamour (geometry via gesso_core)

---

## Task 1: Add palette dependency

**Files:**
- Modify: `crates/gesso_core/Cargo.toml`

**Step 1: Add palette to dependencies**

Update `crates/gesso_core/Cargo.toml`:

```toml
[package]
name = "gesso_core"
version.workspace = true
edition.workspace = true

[dependencies]
glam = "0.32"
glamour = "0.18"
palette = "0.7"

[dev-dependencies]
```

**Step 2: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with palette downloaded

**Step 3: Commit**

```bash
git add crates/gesso_core/Cargo.toml
git commit -m "Add palette dependency for color types"
```

---

## Task 2: Create Scene and Quad

**Files:**
- Create: `crates/gesso_core/src/scene.rs`
- Modify: `crates/gesso_core/src/lib.rs`

**Step 1: Create scene.rs with Quad struct**

Create `crates/gesso_core/src/scene.rs`:

```rust
//! Scene holds primitives for rendering.

use crate::{Corners, DeviceRect, Edges};
use palette::Srgba;

/// A filled/stroked rectangle with optional rounded corners.
#[derive(Clone, Debug)]
pub struct Quad {
    pub bounds: DeviceRect,
    pub background: Srgba,
    pub border_color: Srgba,
    pub border_widths: Edges<f32>,
    pub corner_radii: Corners<f32>,
}

impl Quad {
    pub fn new(bounds: DeviceRect, background: impl Into<Srgba>) -> Self {
        Self {
            bounds,
            background: background.into(),
            border_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
            border_widths: Edges::default(),
            corner_radii: Corners::default(),
        }
    }
}

/// Holds all primitives for a frame, ready for rendering.
#[derive(Default)]
pub struct Scene {
    quads: Vec<Quad>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all primitives, reusing allocations.
    pub fn clear(&mut self) {
        self.quads.clear();
    }

    pub fn push_quad(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn quads(&self) -> &[Quad] {
        &self.quads
    }

    pub fn quad_count(&self) -> usize {
        self.quads.len()
    }
}
```

**Step 2: Add scene module to lib.rs**

Update `crates/gesso_core/src/lib.rs`:

```rust
pub mod geometry;
pub mod scene;

pub use geometry::*;
pub use scene::*;
```

**Step 3: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with no errors

**Step 4: Commit**

```bash
git add crates/gesso_core/src/scene.rs crates/gesso_core/src/lib.rs
git commit -m "Add Scene and Quad primitive"
```

---

## Task 3: Create DrawContext

**Files:**
- Create: `crates/gesso_core/src/context.rs`
- Modify: `crates/gesso_core/src/lib.rs`

**Step 1: Create context.rs**

Create `crates/gesso_core/src/context.rs`:

```rust
//! DrawContext provides a painter's stack for building scenes.

use crate::{Corners, DeviceRect, Edges, Point, Quad, Rect, ScaleFactor, Scene, Size};
use palette::Srgba;

/// Painter's stack for hierarchical drawing.
pub struct DrawContext<'a> {
    scene: &'a mut Scene,
    scale_factor: ScaleFactor,
    offset_stack: Vec<Point>,
    clip_stack: Vec<Rect>,
}

impl<'a> DrawContext<'a> {
    pub fn new(scene: &'a mut Scene, scale_factor: ScaleFactor) -> Self {
        Self {
            scene,
            scale_factor,
            offset_stack: vec![Point::new(0.0, 0.0)],
            clip_stack: Vec::new(),
        }
    }

    /// Current offset (sum of all pushed offsets).
    fn current_offset(&self) -> Point {
        self.offset_stack.last().copied().unwrap_or_default()
    }

    /// Execute closure with additional offset applied.
    pub fn with_offset<R>(&mut self, offset: Point, f: impl FnOnce(&mut Self) -> R) -> R {
        let current = self.current_offset();
        let new_offset = Point::new(current.x + offset.x, current.y + offset.y);
        self.offset_stack.push(new_offset);
        let result = f(self);
        self.offset_stack.pop();
        result
    }

    /// Execute closure with clip bounds applied.
    pub fn with_clip<R>(&mut self, bounds: Rect, f: impl FnOnce(&mut Self) -> R) -> R {
        // Transform clip bounds by current offset
        let offset = self.current_offset();
        let clipped = Rect::new(
            Point::new(bounds.origin.x + offset.x, bounds.origin.y + offset.y),
            bounds.size,
        );
        self.clip_stack.push(clipped);
        let result = f(self);
        self.clip_stack.pop();
        result
    }

    /// Paint a simple filled quad.
    pub fn paint_quad(&mut self, bounds: Rect, fill: impl Into<Srgba>) {
        self.paint(Quad::new(self.to_device_rect(bounds), fill));
    }

    /// Paint a quad with full control.
    pub fn paint(&mut self, quad: Quad) {
        self.scene.push_quad(quad);
    }

    /// Convert logical rect to device rect, applying current offset and scale.
    fn to_device_rect(&self, rect: Rect) -> DeviceRect {
        let offset = self.current_offset();
        let origin = Point::new(rect.origin.x + offset.x, rect.origin.y + offset.y);
        let scaled_origin = self.scale_factor.scale_point(origin);
        let scaled_size = self.scale_factor.scale_size(rect.size);
        DeviceRect::new(scaled_origin, scaled_size)
    }
}
```

**Step 2: Add context module to lib.rs**

Update `crates/gesso_core/src/lib.rs`:

```rust
pub mod context;
pub mod geometry;
pub mod scene;

pub use context::*;
pub use geometry::*;
pub use scene::*;
```

**Step 3: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with no errors

**Step 4: Commit**

```bash
git add crates/gesso_core/src/context.rs crates/gesso_core/src/lib.rs
git commit -m "Add DrawContext with painter's stack"
```

---

## Task 4: Add DrawContext tests

**Files:**
- Modify: `crates/gesso_core/src/context.rs`

**Step 1: Add tests for offset stacking**

Add tests at the bottom of `crates/gesso_core/src/context.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_stacking() {
        let mut scene = Scene::new();
        let scale = ScaleFactor(1.0);
        let mut cx = DrawContext::new(&mut scene, scale);

        // Paint at origin
        cx.paint_quad(
            Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0)),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        // Paint with offset
        cx.with_offset(Point::new(100.0, 50.0), |cx| {
            cx.paint_quad(
                Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0)),
                Srgba::new(0.0, 1.0, 0.0, 1.0),
            );
        });

        assert_eq!(scene.quad_count(), 2);

        let quads = scene.quads();
        // First quad at origin
        assert_eq!(quads[0].bounds.origin.x, 0.0);
        assert_eq!(quads[0].bounds.origin.y, 0.0);
        // Second quad offset by (100, 50)
        assert_eq!(quads[1].bounds.origin.x, 100.0);
        assert_eq!(quads[1].bounds.origin.y, 50.0);
    }

    #[test]
    fn nested_offsets() {
        let mut scene = Scene::new();
        let scale = ScaleFactor(1.0);
        let mut cx = DrawContext::new(&mut scene, scale);

        cx.with_offset(Point::new(10.0, 10.0), |cx| {
            cx.with_offset(Point::new(5.0, 5.0), |cx| {
                cx.paint_quad(
                    Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0)),
                    Srgba::new(1.0, 0.0, 0.0, 1.0),
                );
            });
        });

        let quads = scene.quads();
        // Nested offsets should accumulate: 10+5 = 15
        assert_eq!(quads[0].bounds.origin.x, 15.0);
        assert_eq!(quads[0].bounds.origin.y, 15.0);
    }

    #[test]
    fn scale_factor_applied() {
        let mut scene = Scene::new();
        let scale = ScaleFactor(2.0); // 2x HiDPI
        let mut cx = DrawContext::new(&mut scene, scale);

        cx.paint_quad(
            Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 50.0)),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        );

        let quads = scene.quads();
        // Everything should be scaled by 2
        assert_eq!(quads[0].bounds.origin.x, 20.0);
        assert_eq!(quads[0].bounds.origin.y, 40.0);
        assert_eq!(quads[0].bounds.size.width, 200.0);
        assert_eq!(quads[0].bounds.size.height, 100.0);
    }
}
```

**Step 2: Run tests**

Run: `cargo test -p gesso_core`
Expected: All tests pass

**Step 3: Commit**

```bash
git add crates/gesso_core/src/context.rs
git commit -m "Add DrawContext tests for offset and scale"
```

---

## Task 5: Create Renderer trait

**Files:**
- Create: `crates/gesso_core/src/renderer.rs`
- Modify: `crates/gesso_core/src/lib.rs`

**Step 1: Create renderer.rs**

Create `crates/gesso_core/src/renderer.rs`:

```rust
//! Renderer trait for backend abstraction.

use crate::Scene;

/// Backend-agnostic renderer.
pub trait Renderer {
    /// Surface type for this renderer (e.g., MetalTexture, WgpuSurface).
    type Surface;

    /// Render the scene to the surface.
    fn render(&mut self, scene: &Scene, surface: &mut Self::Surface);
}

/// Debug renderer that counts primitives without GPU.
#[derive(Default)]
pub struct DebugRenderer {
    pub frames_rendered: usize,
    pub last_quad_count: usize,
}

impl Renderer for DebugRenderer {
    type Surface = ();

    fn render(&mut self, scene: &Scene, _surface: &mut Self::Surface) {
        self.frames_rendered += 1;
        self.last_quad_count = scene.quad_count();
    }
}
```

**Step 2: Add renderer module to lib.rs**

Update `crates/gesso_core/src/lib.rs`:

```rust
pub mod context;
pub mod geometry;
pub mod renderer;
pub mod scene;

pub use context::*;
pub use geometry::*;
pub use renderer::*;
pub use scene::*;
```

**Step 3: Verify it compiles**

Run: `cargo check -p gesso_core`
Expected: Compiles with no errors

**Step 4: Commit**

```bash
git add crates/gesso_core/src/renderer.rs crates/gesso_core/src/lib.rs
git commit -m "Add Renderer trait and DebugRenderer"
```

---

## Task 6: Add palette re-export and update llms.txt

**Files:**
- Modify: `crates/gesso_core/src/lib.rs`
- Modify: `llms.txt`

**Step 1: Re-export palette color types**

Update `crates/gesso_core/src/lib.rs`:

```rust
pub mod context;
pub mod geometry;
pub mod renderer;
pub mod scene;

pub use context::*;
pub use geometry::*;
pub use renderer::*;
pub use scene::*;

// Re-export commonly used palette types
pub use palette::{Srgba, Hsla, LinSrgba};
```

**Step 2: Update llms.txt**

Add to Key Files section in `llms.txt`:

```markdown
- [scene.rs](crates/gesso_core/src/scene.rs) - Scene and Quad primitive
- [context.rs](crates/gesso_core/src/context.rs) - DrawContext painter's stack
- [renderer.rs](crates/gesso_core/src/renderer.rs) - Renderer trait, DebugRenderer
```

**Step 3: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 4: Commit**

```bash
git add crates/gesso_core/src/lib.rs llms.txt
git commit -m "Re-export palette types, update llms.txt"
```

---

## Task 7: Update spool and final verification

**Files:**
- Spool stream

**Step 1: Complete spool tasks**

```bash
spool add "Implement Scene and Quad" --stream mlita3ol-lr5j -p p1
spool add "Implement DrawContext" --stream mlita3ol-lr5j -p p1
spool add "Implement Renderer trait" --stream mlita3ol-lr5j -p p1
```

**Step 2: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 3: Verify build**

Run: `cargo build --workspace`
Expected: Builds with no errors

**Step 4: Complete spool tasks and commit**

```bash
spool list --stream mlita3ol-lr5j -f ids | xargs -I {} spool complete {}
git add .spool
git commit -m "Complete draw abstraction implementation"
```

---

## Summary

After completing all tasks:

- `palette` dependency added for color types
- `Scene` holds `Vec<Quad>` in device pixels
- `Quad` struct with bounds, background, border, corners
- `DrawContext` with offset/clip stacks, logical→device conversion
- `Renderer` trait with `DebugRenderer` for testing
- Re-exports for ergonomic API
- Tests for offset stacking and scale factor
