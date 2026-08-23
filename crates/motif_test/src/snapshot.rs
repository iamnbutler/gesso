//! Scene snapshot types for visual regression testing.
//!
//! A [`SceneSnapshot`] captures the rendering primitives of a scene (quads and
//! text runs) at a point in time. Two snapshots can be compared with
//! [`SceneSnapshot::diff`] to detect unintended visual changes.
//!
//! Because snapshots compare the scene's *primitive list* rather than pixels,
//! they work on all platforms — no GPU or display required.
//!
//! # Workflow
//!
//! 1. Render a frame using [`crate::TestHarness::render`].
//! 2. Call [`crate::TestHarness::snapshot`] to capture the current scene.
//! 3. Compare snapshots in tests with [`SceneSnapshot::diff`] or
//!    [`crate::TestHarness::assert_scene_snapshot`].
//!
//! # Example
//!
//! ```
//! use motif_test::TestHarness;
//! use motif_core::{Point, Rect, Size};
//! use palette::Srgba;
//!
//! let mut harness = TestHarness::new(400, 300);
//!
//! harness.render(|cx| {
//!     cx.paint_quad(
//!         Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 50.0)),
//!         Srgba::new(1.0, 0.0, 0.0, 1.0),
//!     );
//! });
//!
//! let snap = harness.snapshot();
//! // At 2× scale factor, device coords are doubled
//! assert_eq!(snap.quads.len(), 1);
//! assert_eq!(snap.quads[0].color_r, 1.0);
//! ```

use motif_core::Scene;

/// A snapshot of a rendered scene for visual regression testing.
///
/// Captures all quads and text runs from a [`Scene`] in a comparable,
/// deterministic form. Primitives are stored in painter's-algorithm order
/// (back to front), matching render order.
#[derive(Debug, Clone, PartialEq)]
pub struct SceneSnapshot {
    /// Quads in back-to-front render order.
    pub quads: Vec<QuadSnapshot>,
    /// Text runs in order.
    pub text_runs: Vec<TextRunSnapshot>,
}

/// Snapshot of a single quad's rendering properties (device pixels).
#[derive(Debug, Clone, PartialEq)]
pub struct QuadSnapshot {
    /// Left edge in device pixels.
    pub x: f32,
    /// Top edge in device pixels.
    pub y: f32,
    /// Width in device pixels.
    pub width: f32,
    /// Height in device pixels.
    pub height: f32,
    /// Background red channel (0.0–1.0).
    pub color_r: f32,
    /// Background green channel (0.0–1.0).
    pub color_g: f32,
    /// Background blue channel (0.0–1.0).
    pub color_b: f32,
    /// Background alpha channel (0.0–1.0).
    pub color_a: f32,
}

/// Snapshot of a single text run's rendering properties.
#[derive(Debug, Clone, PartialEq)]
pub struct TextRunSnapshot {
    /// Baseline start X in device pixels.
    pub x: f32,
    /// Baseline start Y in device pixels.
    pub y: f32,
    /// Number of glyphs in this run.
    pub glyph_count: usize,
    /// Text color red channel (0.0–1.0).
    pub color_r: f32,
    /// Text color green channel (0.0–1.0).
    pub color_g: f32,
    /// Text color blue channel (0.0–1.0).
    pub color_b: f32,
    /// Text color alpha channel (0.0–1.0).
    pub color_a: f32,
    /// Font size in device pixels.
    pub font_size: f32,
}

impl SceneSnapshot {
    /// Capture a snapshot of the current scene state.
    pub fn capture(scene: &Scene) -> Self {
        let quads = scene
            .quads()
            .iter()
            .map(|q| QuadSnapshot {
                x: q.bounds.origin.x,
                y: q.bounds.origin.y,
                width: q.bounds.size.width,
                height: q.bounds.size.height,
                color_r: q.background.red,
                color_g: q.background.green,
                color_b: q.background.blue,
                color_a: q.background.alpha,
            })
            .collect();

        let text_runs = scene
            .text_runs()
            .iter()
            .map(|r| TextRunSnapshot {
                x: r.origin.x,
                y: r.origin.y,
                glyph_count: r.glyphs.len(),
                color_r: r.color.red,
                color_g: r.color.green,
                color_b: r.color.blue,
                color_a: r.color.alpha,
                font_size: r.font_size,
            })
            .collect();

        Self { quads, text_runs }
    }

    /// Return a human-readable description of differences between `self` (expected)
    /// and `actual`. Returns `None` if the snapshots are equal.
    pub fn diff(&self, actual: &SceneSnapshot) -> Option<String> {
        if self == actual {
            return None;
        }

        let mut lines = Vec::new();

        if self.quads.len() != actual.quads.len() {
            lines.push(format!(
                "quad count: expected {}, got {}",
                self.quads.len(),
                actual.quads.len()
            ));
        }

        let min_quads = self.quads.len().min(actual.quads.len());
        for i in 0..min_quads {
            let exp = &self.quads[i];
            let got = &actual.quads[i];
            if exp != got {
                lines.push(format!(
                    "quad[{i}]: expected ({}, {}, {}×{}) rgba({:.2},{:.2},{:.2},{:.2}), \
                     got ({}, {}, {}×{}) rgba({:.2},{:.2},{:.2},{:.2})",
                    exp.x,
                    exp.y,
                    exp.width,
                    exp.height,
                    exp.color_r,
                    exp.color_g,
                    exp.color_b,
                    exp.color_a,
                    got.x,
                    got.y,
                    got.width,
                    got.height,
                    got.color_r,
                    got.color_g,
                    got.color_b,
                    got.color_a,
                ));
            }
        }

        if self.text_runs.len() != actual.text_runs.len() {
            lines.push(format!(
                "text_run count: expected {}, got {}",
                self.text_runs.len(),
                actual.text_runs.len()
            ));
        }

        let min_runs = self.text_runs.len().min(actual.text_runs.len());
        for i in 0..min_runs {
            let exp = &self.text_runs[i];
            let got = &actual.text_runs[i];
            if exp != got {
                lines.push(format!(
                    "text_run[{i}]: expected ({}, {}, {} glyphs, {:.1}px, rgba({:.2},{:.2},{:.2},{:.2})), \
                     got ({}, {}, {} glyphs, {:.1}px, rgba({:.2},{:.2},{:.2},{:.2}))",
                    exp.x,
                    exp.y,
                    exp.glyph_count,
                    exp.font_size,
                    exp.color_r,
                    exp.color_g,
                    exp.color_b,
                    exp.color_a,
                    got.x,
                    got.y,
                    got.glyph_count,
                    got.font_size,
                    got.color_r,
                    got.color_g,
                    got.color_b,
                    got.color_a,
                ));
            }
        }

        if lines.is_empty() {
            None
        } else {
            Some(lines.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use motif_core::{DevicePoint, DeviceRect, DeviceSize, Quad, Scene};
    use palette::Srgba;

    fn make_bounds(x: f32, y: f32, w: f32, h: f32) -> DeviceRect {
        DeviceRect::new(DevicePoint::new(x, y), DeviceSize::new(w, h))
    }

    fn red_quad_scene() -> Scene {
        let mut scene = Scene::new();
        scene.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 100.0, 50.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));
        scene
    }

    // --- SceneSnapshot::capture ---

    #[test]
    fn capture_empty_scene() {
        let scene = Scene::new();
        let snap = SceneSnapshot::capture(&scene);
        assert!(snap.quads.is_empty());
        assert!(snap.text_runs.is_empty());
    }

    #[test]
    fn capture_single_quad_fields() {
        let scene = red_quad_scene();
        let snap = SceneSnapshot::capture(&scene);
        assert_eq!(snap.quads.len(), 1);
        let q = &snap.quads[0];
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.width, 100.0);
        assert_eq!(q.height, 50.0);
        assert_eq!(q.color_r, 1.0);
        assert_eq!(q.color_g, 0.0);
        assert_eq!(q.color_b, 0.0);
        assert_eq!(q.color_a, 1.0);
    }

    #[test]
    fn capture_multiple_quads_order_preserved() {
        let mut scene = Scene::new();
        scene.push_quad(Quad::new(
            make_bounds(10.0, 20.0, 10.0, 10.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));
        scene.push_quad(Quad::new(
            make_bounds(30.0, 40.0, 10.0, 10.0),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        ));
        scene.push_quad(Quad::new(
            make_bounds(50.0, 60.0, 10.0, 10.0),
            Srgba::new(0.0, 0.0, 1.0, 1.0),
        ));

        let snap = SceneSnapshot::capture(&scene);
        assert_eq!(snap.quads.len(), 3);
        assert_eq!(snap.quads[0].x, 10.0);
        assert_eq!(snap.quads[1].x, 30.0);
        assert_eq!(snap.quads[2].x, 50.0);
        assert_eq!(snap.quads[2].color_b, 1.0);
    }

    #[test]
    fn capture_preserves_alpha() {
        let mut scene = Scene::new();
        scene.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 50.0, 50.0),
            Srgba::new(0.5, 0.5, 0.5, 0.25),
        ));
        let snap = SceneSnapshot::capture(&scene);
        assert_eq!(snap.quads[0].color_a, 0.25);
    }

    // --- SceneSnapshot::diff ---

    #[test]
    fn diff_identical_returns_none() {
        let scene = red_quad_scene();
        let snap = SceneSnapshot::capture(&scene);
        assert_eq!(snap.diff(&snap.clone()), None);
    }

    #[test]
    fn diff_empty_scenes_returns_none() {
        let snap = SceneSnapshot::capture(&Scene::new());
        assert_eq!(snap.diff(&snap.clone()), None);
    }

    #[test]
    fn diff_quad_count_mismatch() {
        let snap_a = SceneSnapshot::capture(&red_quad_scene());
        let snap_b = SceneSnapshot::capture(&Scene::new());
        let diff = snap_a.diff(&snap_b).expect("should have diff");
        assert!(diff.contains("quad count"));
        assert!(diff.contains("expected 1, got 0"));
    }

    #[test]
    fn diff_quad_color_change() {
        let mut scene_a = Scene::new();
        scene_a.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 100.0, 50.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));
        let mut scene_b = Scene::new();
        scene_b.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 100.0, 50.0),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        ));

        let diff = SceneSnapshot::capture(&scene_a)
            .diff(&SceneSnapshot::capture(&scene_b))
            .expect("should have diff");
        assert!(diff.contains("quad[0]"));
    }

    #[test]
    fn diff_quad_position_change() {
        let mut scene_a = Scene::new();
        scene_a.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 100.0, 50.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));
        let mut scene_b = Scene::new();
        scene_b.push_quad(Quad::new(
            make_bounds(10.0, 0.0, 100.0, 50.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));

        let diff = SceneSnapshot::capture(&scene_a)
            .diff(&SceneSnapshot::capture(&scene_b))
            .expect("should have diff");
        assert!(diff.contains("quad[0]"));
    }

    #[test]
    fn diff_text_run_count_mismatch() {
        let snap_a = SceneSnapshot {
            quads: vec![],
            text_runs: vec![TextRunSnapshot {
                x: 0.0,
                y: 0.0,
                glyph_count: 3,
                color_r: 1.0,
                color_g: 1.0,
                color_b: 1.0,
                color_a: 1.0,
                font_size: 16.0,
            }],
        };
        let snap_b = SceneSnapshot {
            quads: vec![],
            text_runs: vec![],
        };
        let diff = snap_a.diff(&snap_b).expect("should have diff");
        assert!(diff.contains("text_run count"));
    }

    // --- PartialEq ---

    #[test]
    fn snapshots_equal_same_scene() {
        let scene = red_quad_scene();
        assert_eq!(
            SceneSnapshot::capture(&scene),
            SceneSnapshot::capture(&scene)
        );
    }

    #[test]
    fn snapshots_not_equal_different_quad_count() {
        let snap_a = SceneSnapshot::capture(&red_quad_scene());
        let snap_b = SceneSnapshot::capture(&Scene::new());
        assert_ne!(snap_a, snap_b);
    }

    #[test]
    fn snapshots_not_equal_different_color() {
        let mut scene_a = Scene::new();
        scene_a.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 10.0, 10.0),
            Srgba::new(1.0, 0.0, 0.0, 1.0),
        ));
        let mut scene_b = Scene::new();
        scene_b.push_quad(Quad::new(
            make_bounds(0.0, 0.0, 10.0, 10.0),
            Srgba::new(0.0, 1.0, 0.0, 1.0),
        ));
        assert_ne!(
            SceneSnapshot::capture(&scene_a),
            SceneSnapshot::capture(&scene_b)
        );
    }
}
