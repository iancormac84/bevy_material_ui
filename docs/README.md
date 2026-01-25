# Bevy Material UI Documentation

A comprehensive Material Design 3 component library for Bevy game engine.

- Back to main README: [../README.md](../README.md)
- Developer guide: [./DEVELOPER_GUIDE.md](./DEVELOPER_GUIDE.md)

## Components

| Component | Description | Documentation |
|-----------|-------------|---------------|
| [Button](./components/button.md) | Filled, outlined, and text buttons with state layers | [View](./components/button.md) |
| [Card](./components/card.md) | Elevated, filled, and outlined cards | [View](./components/card.md) |
| [Checkbox](./components/checkbox.md) | Checkboxes with animation | [View](./components/checkbox.md) |
| [Chip](./components/chip.md) | Assist, filter, input, and suggestion chips | [View](./components/chip.md) |
| [Dialog](./components/dialog.md) | Modal dialogs with actions | [View](./components/dialog.md) |
| [Divider](./components/divider.md) | Horizontal and vertical dividers | [View](./components/divider.md) |
| [FAB](./components/fab.md) | Floating action buttons | [View](./components/fab.md) |
| [Icon Button](./components/icon_button.md) | Icon-only buttons | [View](./components/icon_button.md) |
| [List](./components/list.md) | Lists with selection support | [View](./components/list.md) |
| [Menu](./components/menu.md) | Dropdown menus | [View](./components/menu.md) |
| [Progress](./components/progress.md) | Linear and circular progress indicators | [View](./components/progress.md) |
| [Radio](./components/radio.md) | Radio button groups | [View](./components/radio.md) |
| [Select](./components/select.md) | Dropdown select components | [View](./components/select.md) |
| [Slider](./components/slider.md) | Range sliders | [View](./components/slider.md) |
| [Snackbar](./components/snackbar.md) | Toast notifications | [View](./components/snackbar.md) |
| [Switch](./components/switch.md) | Toggle switches | [View](./components/switch.md) |
| [Tabs](./components/tabs.md) | Tab navigation | [View](./components/tabs.md) |
| [Text Field](./components/text_field.md) | Input fields with validation | [View](./components/text_field.md) |
| [Tooltip](./components/tooltip.md) | Hover tooltips | [View](./components/tooltip.md) |
| [DateTime Picker](./components/datetime_picker.md) | Date and time picking dialogs | [View](./components/datetime_picker.md) |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_material_ui = { path = "../bevy_material_ui" }
```

Basic setup:

```rust
use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Camera2d::default());
    
    // Your UI here
}
```

## Theme System

The library uses Material Design 3 color tokens. Access colors through `MaterialTheme` resource:

```rust
fn my_system(theme: Res<MaterialTheme>) {
    let primary = theme.primary;
    let surface = theme.surface;
    let on_primary = theme.on_primary;
    // etc.
}
```

## Interactive Showcase

To see all components in action, run the interactive showcase:

```bash
cargo run --example showcase
```

The showcase provides a comprehensive demo of every component with:
- Live interaction and state changes
- Multiple variants and configurations  
- Theme switching (light/dark mode)
- Responsive layout examples

Navigate through the sidebar to explore each component category.

## Running Examples

Run the dedicated layout demo:

```bash
cargo run --example layouts_demo
```

Other examples are available in the `examples/` folder:

```bash
cargo run --example button_demo
cargo run --example select_demo
```

## Running Tests

Run the full test suite:

```bash
cargo test
```

## UI Automated Tests (Telemetry)

The showcase can emit a `telemetry.json` file with element bounds and recent UI events
for automation tooling. Enable it via an environment variable:

```bash
BEVY_TELEMETRY=1 cargo run --example showcase
```

The file is written to the project root as `telemetry.json`.

> 📺 **Video Demo**: For a walkthrough of the UI components, see the [demo video](https://github.com/user/repo/releases) (coming soon).
