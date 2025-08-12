# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Structure

This is a Tauri v2 plugin that provides custom window decorations with cross-platform support. The plugin consists of:

- **Rust Core (`src/`)**: Plugin implementation with platform-specific modules
- **JavaScript/TypeScript API (`guest-js/`, `dist-js/`)**: Frontend bindings for the plugin
- **Example Application (`examples/tauri-app/`)**: React-based demo app showing plugin usage
- **Build System**: Dual package.json + Cargo.toml for both Rust and JS distributions

## Development Commands

### Plugin Development
```bash
# Build the JavaScript API bindings
pnpm build

# Run tests across all platforms
cargo test --all-targets --all-features

# Test the example application
cd examples/tauri-app
pnpm install
pnpm tauri dev
```

### Publishing
```bash
# Build JS bindings before publishing
pnpm build
cargo publish  # Publishes Rust crate
pnpm publish   # Publishes JS bindings
```

## Architecture Overview

### Plugin Structure
The plugin follows Tauri's plugin architecture pattern:
- `build.rs`: Defines plugin commands (`show_snap_overlay`)
- `src/lib.rs`: Main plugin implementation with `WebviewWindowExt` trait
- `src/commands.rs`: Tauri command handlers
- Platform-specific modules: `traffic.rs` (macOS), `dconf.rs` (Linux)

### Core Functionality
The plugin provides the `WebviewWindowExt` trait that extends `WebviewWindow` with:
- `create_overlay_titlebar()`: Creates custom titlebar with drag region
- `set_traffic_lights_inset()` (macOS): Positions window controls
- `make_transparent()` (macOS): Window transparency without private APIs
- `set_window_level()` (macOS): Controls window layering

### Platform-Specific Implementation
- **Windows**: Uses `enigo` crate for snap layout simulation, removes native decorations
- **macOS**: Direct Cocoa/Objective-C bindings for traffic light positioning and transparency
- **Linux**: Uses `linicon` for system icons and `dconf` for reading GNOME preferences

### JavaScript Integration
- Frontend API built with Rollup from `guest-js/index.ts`
- Outputs both ESM and CJS formats to `dist-js/`
- Uses `@tauri-apps/api` for invoke calls to Rust backend

### Event System
Plugin listens for `decorum-page-load` event to inject platform-specific JavaScript:
- `js/titlebar.js`: Creates draggable titlebar regions
- `js/controls.js`: Windows-specific window controls
- `js/linux-controls.js`: Linux-specific controls with system theming

## Development Notes

### Permission Requirements
The plugin requires specific Tauri permissions in `capabilities/default.json`:
```json
"core:window:allow-*",
"decorum:allow-show-snap-overlay"
```

### macOS Threading
All macOS window operations must run on the main thread. The `ensure_main_thread()` helper handles cross-thread dispatch automatically.

### Testing Strategy
- Cross-platform CI runs on Ubuntu, macOS, and Windows
- Example app serves as integration test
- No unit tests currently - relies on example app validation