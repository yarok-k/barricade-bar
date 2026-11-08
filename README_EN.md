# Barricade Bar

Status bar for the [niri](https://github.com/YaLTeR/niri) Wayland compositor, built with Rust, GTK 3 and Layer Shell.

## Description

Barricade Bar is a lightweight status bar that is drawn on top of the screen (one window per monitor) and reserves space for itself. It is configured with a single YAML file and a CSS file, and both are reloaded on the fly. It uses:
- **GTK 3** and **gtk-layer-shell** to draw the bar on Wayland
- **niri IPC** (`niri-ipc`) to read the keyboard layout
- **wpctl** (WirePlumber / PipeWire) to read the volume level and mute state
- **NetworkManager** (D-Bus, via `nmrs`) to read the network state
- **sysfs** (`/sys/class/power_supply`) to read the battery state
- **Rust** for performance and memory safety

## Features

- Three zones (left, center, right) with any set and order of widgets
- Widgets: `clock`, `audio`, `layout`, `network`, `battery`
- Volume and battery level are revealed with a slide animation on hover
- The battery widget hides itself on machines without a battery
- A separate bar on every monitor, new monitors are picked up automatically
- Optional command bound to a widget, launched on click (for example, a sound control popup)
- Hot reload of the config and of the CSS file, no restart needed
- Network is polled in a background thread, so the UI is never blocked by D-Bus

## Installation

### Requirements

- Rust 1.85+ (edition 2024)
- GTK 3.x
- GTK Layer Shell 0.7+
- niri (the layout widget talks to its IPC socket)
- A Wayland compositor with `wlr-layer-shell` support
- Runtime tools and services: `wpctl` (WirePlumber), NetworkManager
- An icon theme with freedesktop symbolic icons (for example, Adwaita)

### Build

```bash
git clone https://github.com/yarok-k/barricade-bar
cd barricade-bar
cargo build --release
```

### Run

```bash
cargo run --release
```

## Usage

Start the binary from your niri config:

```kdl
spawn-at-startup "/path/to/barricade_bar"
```

### Configuration

The config file is located at `~/.config/barricade/bar.bst` and is created on first run. It is a YAML file with three lists of widgets and an optional path to a CSS file:

```yaml
elements:
  left:
    - "layout"
  center:
    - "clock"
  right:
    - "network"
    - audio: "barricade-sound"   # command launched on click
    - "battery"
stylesheet: "~/.config/barricade/style.css"
```

- An element is either a plain name (`"clock"`) or `name: "command"`. The values `""`, `null` and `none` mean "no command".
- Available elements: `clock`, `audio`, `layout`, `network`, `battery`.
- The command is currently used by the `audio` widget.
- Relative stylesheet paths are resolved from `~/.config/barricade/`, `~/` is expanded to your home directory. An empty value means no custom styles. A missing CSS file is created empty.
- If the config contains an error, the bar keeps its previous state and prints the error to stderr.

### Styling

The bar background has the CSS class `barricade_background`. All other nodes can be styled by the standard GTK 3 names (`button`, `label`, `image`, `revealer`).

```css
.barricade_background {
    background: #1e1e2e;
    color: #cdd6f4;
}
```

## Project Structure

```
src/
├── main.rs              # Entry point, layer-shell windows, CSS hot reload
├── ui.rs                # Bar layout (left / center / right), periodic widget updates
├── lib/
│   ├── mod.rs           # Library modules
│   ├── audio.rs         # Volume / mute / sinks via wpctl
│   ├── battery.rs       # Battery state from /sys/class/power_supply
│   ├── layout.rs        # Keyboard layout via niri IPC
│   ├── network.rs       # Network devices via NetworkManager (nmrs)
│   ├── bluetooth.rs     # Reserved, not implemented yet
│   └── core/
│       ├── parcer.rs    # Configuration file parser
│       ├── placer.rs    # Builds widgets and places them into zones
│       ├── widget.rs    # Common widget trait
│       └── element.rs   # Built widget wrapper
└── widgets/
    ├── mod.rs           # Widget modules
    ├── clock.rs         # Date and time
    ├── audio.rs         # Volume indicator
    ├── layout.rs        # Keyboard layout indicator
    ├── network.rs       # Network indicator
    └── battery.rs       # Battery indicator
```

## Development

### Build and run in development mode

```bash
cargo run
```

### Linting and type checking

```bash
cargo clippy
cargo check
```

## License

GNU General Public License v3.0 or later (GPL-3.0-or-later). See [LICENSE](LICENSE) and [NOTICE](NOTICE).

The GPL is required by the dependency [`niri-ipc`](https://crates.io/crates/niri-ipc), which is licensed under GPL-3.0-or-later. All other dependencies use permissive licenses compatible with the GPL (MIT, Apache-2.0, BSD, Unlicense).
