# Moomer

Zoomer application for macOS. A poor man's clone of [tsoding/boomer](https://github.com/tsoding/boomer).

## Quick Start

```bash
$ cargo build --release
$ cargo run --release
```

## Controls

| Control                       | Description                           |
|-------------------------------|---------------------------------------|
| <kbd>q</kbd>                  | Quit the application.                 |
| Drag with left mouse button   | Move the image around.                |
| Scroll wheel or trackpad      | Zoom in/out (centered on cursor).     |

## Features

- Takes a screenshot of your primary display on startup
- Fullscreen viewing without window decorations
- Smooth zoom with trackpad gestures
- Native macOS trackpad support
- Pan by clicking and dragging

## Building

### Dependencies

```bash
$ cargo build --release
```

The following Rust crates are used:

- `screenshots` - Screen capture
- `winit` - Window management and input handling
- `pixels` - Pixel buffer rendering
- `image` - Image processing

## Running

```bash
$ cargo run --release
```

The application will immediately capture your screen and display it in fullscreen mode. Use your trackpad or mouse to zoom and pan around the screenshot.

