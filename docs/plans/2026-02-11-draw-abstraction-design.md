# Draw Abstraction Design

Core draw abstraction for gesso's rendering pipeline.

## Goals

- Painter's stack model for hierarchical UI drawing
- Backend-agnostic renderer trait (metal, wgpu)
- GPU-friendly data layout from the start
- Minimal primitive set — just Quad initially

## Non-Goals

- Path/vector primitives (future)
- Text rendering (parley integration later)
- Actual GPU backend implementation (separate crates)

## Architecture

### Mental Model

```
DrawContext (logical pixels, painter's stack)
    ↓ paint()
Scene (device pixels, typed primitive vecs)
    ↓ render()
Renderer trait → Metal/Wgpu backends
```

### Module Structure

```
crates/gesso_core/src/
├── lib.rs
├── geometry.rs     # existing
├── scene.rs        # Scene, Quad
├── context.rs      # DrawContext
└── renderer.rs     # Renderer trait
```

## Components

### Scene

Holds primitives in device pixels, ready for GPU submission.

```rust
pub struct Scene {
    pub(crate) quads: Vec<Quad>,
    // Future: shadows, paths, sprites
}

impl Scene {
    pub fn new() -> Self;
    pub fn clear(&mut self);  // reuse allocation frame-to-frame
    pub fn push_quad(&mut self, quad: Quad);
}
```

Typed vectors per primitive type enable efficient batching — all quads can be drawn in one call.

### Quad

Single primitive covering most UI needs: solid rects, rounded rects, borders.

```rust
#[derive(Clone, Debug)]
pub struct Quad {
    pub bounds: DeviceRect,
    pub background: palette::Srgba,
    pub border_color: palette::Srgba,
    pub border_widths: Edges<f32>,
    pub corner_radii: Corners<f32>,
}
```

All measurements in device pixels — conversion from logical happens at draw time.

### DrawContext

Painter's stack with closure-scoped transform/clip.

```rust
pub struct DrawContext<'a> {
    scene: &'a mut Scene,
    transform_stack: Vec<Transform2<LogicalPixels>>,
    clip_stack: Vec<Rect>,
    scale_factor: ScaleFactor,
}

impl<'a> DrawContext<'a> {
    pub fn new(scene: &'a mut Scene, scale_factor: ScaleFactor) -> Self;

    // Closure-scoped state — compiler enforces matching
    pub fn with_transform<R>(&mut self, t: Transform2<LogicalPixels>, f: impl FnOnce(&mut Self) -> R) -> R;
    pub fn with_clip<R>(&mut self, bounds: Rect, f: impl FnOnce(&mut Self) -> R) -> R;

    // Convenience methods
    pub fn paint_quad(&mut self, bounds: Rect, fill: impl Into<palette::Srgba>);

    // Full control
    pub fn paint(&mut self, quad: Quad);
}
```

Transform/clip stacks use logical pixels. Conversion to device pixels happens inside `paint()` before pushing to Scene.

### Renderer Trait

Backend abstraction with static dispatch via generics.

```rust
pub trait Renderer {
    type Surface;

    fn render(&mut self, scene: &Scene, surface: &mut Self::Surface);
}
```

Backends (future crates):
- `gesso_metal` — MetalRenderer
- `gesso_wgpu` — WgpuRenderer

For testing, a `NullRenderer` or `DebugRenderer` can log/count primitives without GPU.

## Dependencies

- `palette` — Color types (Srgba, Hsla), conversions, blending
- `gesso_core::geometry` — Point, Rect, Size, Edges, Corners, ScaleFactor, Transform2

## Design Decisions

### Painter's Stack over Flat Buffer

Closure-scoped `with_transform`/`with_clip` mirrors UI hierarchy naturally. Compiler enforces push/pop matching. More ergonomic than manual push/pop.

### Early Pixel Conversion

Logical → device conversion at draw time, not render time. Scene stores device pixels, ready for GPU. Simpler renderer, no per-primitive conversion at render time.

### Typed Vectors per Primitive

`Vec<Quad>` instead of `Vec<DrawCommand>` enum. Cache-friendly, enables batching all quads in one draw call. Add new vecs when new primitives are needed.

### Trait with Static Dispatch

`Renderer` trait with generics (`Window<R: Renderer>`) rather than `Box<dyn Renderer>`. Zero overhead, extensible.

### palette for Color

Use `palette` crate rather than hand-rolled color types. Proper color science, SIMD support, well-tested. Custom color code causes maintenance problems.

## Future Additions

When needed:
- `Shadow` primitive for drop shadows
- `Path` primitive for vector shapes (requires tessellation)
- Sprite primitives for text glyphs (parley integration)
- Arena allocation if profiling shows Vec churn

## Performance Considerations

Designed for GPU-friendliness without premature optimization:

- `Vec::clear()` reuses allocation frame-to-frame
- Typed vecs enable batching
- Device pixel storage avoids per-primitive conversion at render time
- Static dispatch avoids dyn overhead
- glam (via glamour) provides SIMD for transform math

Benchmarks will be added early to monitor performance as features are added.
