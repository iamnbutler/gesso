# motif-debug

CLI for inspecting running motif processes.

## Install

```
cargo install --path crates/motif_debug_cli
```

## Usage

```
motif-debug [OPTIONS] [COMMAND]
```

Single command mode:
```
motif-debug scene.stats
```

REPL mode (no command):
```
motif-debug
motif> scene.stats
motif> quit
```

JSON output for scripting:
```
motif-debug --json scene.stats
```

Connect to specific socket:
```
motif-debug --socket /tmp/motif-debug-12345.sock scene.stats
```

## Commands

### Scene inspection

| Command | Description |
|---------|-------------|
| `scene.stats` | Quad count, text run count, viewport size, scale factor |
| `scene.quads` | List all quads with bounds, color, border, corner radii |
| `scene.text_runs` | List all text runs with origin, font size, glyph count |

### Input inspection

| Command | Description |
|---------|-------------|
| `input.state` | Current cursor position, pressed buttons, modifier keys |

### Input simulation

Simulate user input in the running motif app:

```
input.activate                         # bring app window to front
input.move_to <x> <y>                  # move mouse to window-local coordinates
input.click <x> <y>                    # click at window-local coordinates
input.mouse_down <x> <y>              # press mouse button at coordinates
input.mouse_up <x> <y>                # release mouse button at coordinates
input.drag <x1> <y1> <x2> <y2>        # drag from one point to another
```

Coordinates are in logical pixels relative to the window.

### Screenshots

```
screenshot /path/to/file.png
screenshot                      # auto-generates /tmp/motif-screenshot-{timestamp}.png
```

### Debug overlays

Draw colored rectangles on top of the scene (persist until cleared):

```
draw.quad x y w h r g b a       # coordinates in logical pixels, color 0.0-1.0
draw.quad 100 100 200 50 1 0 0 0.5
```

```
debug.list                      # list all overlays
debug.remove <id>               # remove specific overlay
debug.clear                     # remove all overlays
```

## Socket location

Servers listen at `/tmp/motif-debug-{pid}.sock`. The CLI auto-discovers running servers.
