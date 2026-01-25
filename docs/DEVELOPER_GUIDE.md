# Bevy Material UI - Developer Guide

A comprehensive Material Design 3 (MD3) UI component library for the [Bevy game engine](https://bevyengine.org/). This library provides production-ready UI components following [Google's Material Design 3](https://m3.material.io/) guidelines.

> **Note:** This library is inspired by [material-web](https://github.com/material-components/material-web) and follows the same design patterns adapted for Bevy's ECS architecture.

## Table of Contents

- [Quick Start](#quick-start)
- [Running the Showcase](#running-the-showcase)
- [Architecture](#architecture)
- [Theming](#theming)
- [Components](#components)
  - [Buttons](#buttons)
  - [Icon Buttons](#icon-buttons)
  - [Floating Action Buttons (FAB)](#floating-action-buttons-fab)
  - [Cards](#cards)
  - [Checkboxes](#checkboxes)
  - [Radio Buttons](#radio-buttons)
  - [Switches](#switches)
  - [Sliders](#sliders)
  - [Text Fields](#text-fields)
  - [Progress Indicators](#progress-indicators)
  - [Dialogs](#dialogs)
  - [Lists](#lists)
  - [Menus](#menus)
  - [Tabs](#tabs)
  - [Dividers](#dividers)
  - [Select](#select)
- [Color System](#color-system)
- [Icons](#icons)
- [Accessibility](#accessibility)
- [WebGL Deployment](#webgl-deployment)

---

## Quick Start

Add `bevy_material_ui` to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.17"
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
    // Required: Spawn a 2D camera for UI
    commands.spawn(Camera2d);

    // Create a filled button
    commands.spawn(
        MaterialButtonBuilder::new("Click Me")
            .filled()
            .build(&theme)
    );
}
```

---

## Running the Showcase

The showcase example demonstrates all available components:

```bash
cd bevy_material_ui
cargo run --example showcase
```

---

## Running Tests

Run the full test suite:

```bash
cargo test
```

---

## UI Automated Tests (Telemetry)

The showcase can emit `telemetry.json` for UI automation tooling. Enable it with an
environment variable:

```bash
BEVY_TELEMETRY=1 cargo run --example showcase
```

The file is written to the project root as `telemetry.json` and includes:
- Element bounds keyed by `TestId`
- Recent component events

This opens an interactive window displaying:
- All button variants (Elevated, Filled, Tonal, Outlined, Text)
- Icon buttons and FABs
- Selection controls (Checkbox, Radio, Switch)
- Input components (Slider, Text Field)
- Container components (Card, List)
- Feedback components (Progress, Dialog)
- Navigation components (Tabs, Menu)

---

## Architecture

### ECS Pattern

Bevy Material UI follows Bevy's Entity Component System (ECS) pattern:

- **Components** - Data attached to entities (e.g., `MaterialButton`, `MaterialCard`)
- **Systems** - Logic that operates on components (e.g., `button_interaction_system`)
- **Events/Messages** - Communication between systems (e.g., `ButtonClickEvent`)

### Builder Pattern

Most components use a builder pattern for ergonomic construction:

```rust
// Builder pattern
let button = MaterialButtonBuilder::new("Submit")
    .filled()
    .disabled(false)
    .build(&theme);

// Direct construction
let button = MaterialButton::new("Submit")
    .with_variant(ButtonVariant::Filled);
```

### Plugin Architecture

Each component has its own plugin:

```rust
// Individual plugins
app.add_plugins(ButtonPlugin);
app.add_plugins(CardPlugin);

// Or use the main plugin for everything
app.add_plugins(MaterialUiPlugin);
```

---

## Theming

### Design Tokens

Material Design 3 uses a token-based system where design decisions flow through three levels:

```
Reference Tokens → System Tokens → Component Tokens
     ↓                  ↓                ↓
  Raw values      Design roles      Component styles
  (#6750A4)       (primary)         (button-container)
```

### MaterialTheme Resource

The `MaterialTheme` resource provides access to all theme colors:

```rust
fn setup(theme: Res<MaterialTheme>) {
    // Primary colors
    let primary = theme.primary;
    let on_primary = theme.on_primary;
    let primary_container = theme.primary_container;

    // Surface colors
    let surface = theme.surface;
    let on_surface = theme.on_surface;
    let surface_container = theme.surface_container;

    // Additional roles
    let secondary = theme.secondary;
    let tertiary = theme.tertiary;
    let error = theme.error;
    let outline = theme.outline;
}
```

### Color Scheme

Switch between light and dark modes:

```rust
fn toggle_theme(mut theme: ResMut<MaterialTheme>) {
    theme.color_scheme = match theme.color_scheme {
        ColorScheme::Light => ColorScheme::Dark,
        ColorScheme::Dark => ColorScheme::Light,
    };
    // Regenerate colors
    theme.regenerate();
}
```

### Custom Colors

Generate a theme from a seed color using the HCT color space:

```rust
use bevy_material_ui::color::{Hct, MaterialColorScheme};

// Create HCT color (Hue, Chroma, Tone)
let seed = Hct::new(265.0, 48.0, 40.0); // Purple

// Generate full color scheme
let scheme = MaterialColorScheme::from_seed(seed, ColorScheme::Light);
```

### Spacing Tokens

Consistent spacing values:

```rust
use bevy_material_ui::tokens::Spacing;

Node {
    padding: UiRect::all(Val::Px(Spacing::MEDIUM)),  // 16px
    margin: UiRect::all(Val::Px(Spacing::SMALL)),    // 8px
    row_gap: Val::Px(Spacing::LARGE),                // 24px
    ..default()
}
```

| Token | Value |
|-------|-------|
| `Spacing::EXTRA_SMALL` | 4px |
| `Spacing::SMALL` | 8px |
| `Spacing::MEDIUM` | 16px |
| `Spacing::LARGE` | 24px |
| `Spacing::EXTRA_LARGE` | 32px |

### Corner Radius Tokens

```rust
use bevy_material_ui::tokens::CornerRadius;

BorderRadius::all(Val::Px(CornerRadius::MEDIUM))  // 12px
```

| Token | Value |
|-------|-------|
| `CornerRadius::NONE` | 0px |
| `CornerRadius::EXTRA_SMALL` | 4px |
| `CornerRadius::SMALL` | 8px |
| `CornerRadius::MEDIUM` | 12px |
| `CornerRadius::LARGE` | 16px |
| `CornerRadius::EXTRA_LARGE` | 28px |
| `CornerRadius::FULL` | 9999px |

---

## Components

### Buttons

Buttons help users initiate actions. There are five types:

| Variant | Use Case |
|---------|----------|
| **Elevated** | Need visual separation from patterned backgrounds |
| **Filled** | High emphasis - primary actions like "Save", "Submit" |
| **Filled Tonal** | Medium emphasis - secondary actions like "Next" |
| **Outlined** | Medium emphasis - important but not primary |
| **Text** | Low emphasis - less important actions |

#### Usage

```rust
// Using builder
let button = MaterialButtonBuilder::new("Submit")
    .filled()
    .build(&theme);

commands.spawn(button);

// Using component directly
commands.spawn((
    MaterialButton::new("Cancel").with_variant(ButtonVariant::Outlined),
    Button,
    RippleHost::new(),
    // ... node styling
));
```

#### Handling Clicks

```rust
fn handle_clicks(mut events: MessageReader<ButtonClickEvent>) {
    for event in events.read() {
        println!("Button clicked: {:?}", event.entity);
    }
}
```

#### Button States

```rust
let button = MaterialButtonBuilder::new("Disabled")
    .filled()
    .disabled(true)  // Grayed out, non-interactive
    .build(&theme);
```

---

### Icon Buttons

Icon buttons are compact buttons that display an icon without text.

| Variant | Use Case |
|---------|----------|
| **Standard** | Default, minimal emphasis |
| **Filled** | High emphasis actions |
| **Filled Tonal** | Medium emphasis |
| **Outlined** | Medium emphasis with outline |

```rust
let icon_button = IconButtonBuilder::new("favorite")
    .filled()
    .build(&theme);

// Handle clicks
fn handle_icon_clicks(mut events: MessageReader<IconButtonClickEvent>) {
    for event in events.read() {
        // Handle click
    }
}
```

---

### Floating Action Buttons (FAB)

FABs represent the primary action on a screen.

| Size | Dimensions | Use Case |
|------|------------|----------|
| **Small** | 40×40px | Compact layouts |
| **Regular** | 56×56px | Standard use |
| **Large** | 96×96px | Emphasized primary action |

```rust
let fab = FabBuilder::new("add")
    .regular()              // or .small(), .large()
    .primary_container()    // Color variant
    .build(&theme);
```

---

### Cards

Cards contain content and actions about a single subject.

| Variant | Appearance |
|---------|------------|
| **Elevated** | Raised with shadow |
| **Filled** | Solid background, no shadow |
| **Outlined** | Border, no shadow |

```rust
let card = CardBuilder::new()
    .elevated()
    .clickable(true)
    .build(&theme);

commands.spawn(card).with_children(|parent| {
    // Card content
    parent.spawn((Text::new("Card Title"), ...));
});
```

---

### Checkboxes

Checkboxes allow selecting multiple options from a set.

```rust
let checkbox = CheckboxBuilder::new()
    .checked(true)
    .build(&theme);

// Handle state changes
fn handle_checkbox(mut events: MessageReader<CheckboxChangeEvent>) {
    for event in events.read() {
        println!("Checkbox {} is now {}", event.entity, event.checked);
    }
}
```

#### States

| State | Description |
|-------|-------------|
| Unchecked | Default, empty |
| Checked | Selected, shows checkmark |
| Indeterminate | Partially selected (parent of mixed children) |

---

### Radio Buttons

Radio buttons allow selecting one option from a set.

```rust
// Create a radio group
let group = RadioGroup::new("options");

// Create radio buttons in the same group
let radio1 = RadioBuilder::new()
    .group(group.clone())
    .selected(true)
    .build(&theme);

let radio2 = RadioBuilder::new()
    .group(group.clone())
    .build(&theme);
```

---

### Switches

Switches toggle a single setting on or off.

```rust
let switch = SwitchBuilder::new()
    .selected(true)
    .build(&theme);

// Handle toggle
fn handle_switch(mut events: MessageReader<SwitchChangeEvent>) {
    for event in events.read() {
        println!("Switch is now {}", if event.selected { "ON" } else { "OFF" });
    }
}
```

---

### Sliders

Sliders allow selecting a value from a range.

```rust
let slider = SliderBuilder::new()
    .min(0.0)
    .max(100.0)
    .value(50.0)
    .step(1.0)  // Optional discrete steps
    .build(&theme);

// Handle value changes
fn handle_slider(mut events: MessageReader<SliderChangeEvent>) {
    for event in events.read() {
        println!("Slider value: {}", event.value);
    }
}
```

---

### Text Fields

Text fields let users enter and edit text.

| Variant | Appearance |
|---------|------------|
| **Filled** | Solid background with bottom border |
| **Outlined** | Full border, no background |

```rust
let text_field = TextFieldBuilder::new()
    .label("Email")
    .placeholder("Enter your email")
    .filled()  // or .outlined()
    .build(&theme);

// Handle text changes
fn handle_text(mut events: MessageReader<TextFieldChangeEvent>) {
    for event in events.read() {
        println!("Text: {}", event.value);
    }
}

// Handle submit (Enter key)
fn handle_submit(mut events: MessageReader<TextFieldSubmitEvent>) {
    for event in events.read() {
        println!("Submitted: {}", event.value);
    }
}
```

---

### Progress Indicators

Progress indicators show the status of ongoing processes.

#### Linear Progress

```rust
// Determinate (known progress)
let progress = LinearProgressBuilder::new()
    .progress(0.6)  // 60%
    .build(&theme);

// Indeterminate (unknown duration)
let progress = LinearProgressBuilder::new()
    .indeterminate()
    .build(&theme);
```

#### Circular Progress

```rust
let progress = CircularProgressBuilder::new()
    .progress(0.75)
    .build(&theme);
```

---

### Dialogs

Dialogs interrupt the user flow to request information or confirmation.

```rust
let dialog = DialogBuilder::new()
    .headline("Confirm Action")
    .content("Are you sure you want to proceed?")
    .cancel_button("Cancel")
    .confirm_button("Confirm")
    .build(&theme);

// Handle events
fn handle_dialog(
    mut open_events: MessageReader<DialogOpenEvent>,
    mut close_events: MessageReader<DialogCloseEvent>,
    mut confirm_events: MessageReader<DialogConfirmEvent>,
) {
    for event in confirm_events.read() {
        // User confirmed
    }
}
```

---

### Lists

Lists present content in a continuous vertical index.

```rust
let list = ListBuilder::new().build();

commands.spawn(list).with_children(|parent| {
    // List items
    parent.spawn(
        ListItemBuilder::new()
            .headline("Item 1")
            .supporting_text("Supporting text")
            .leading_icon("star")
            .build(&theme)
    );

    // Divider
    parent.spawn(create_list_divider(&theme));

    parent.spawn(
        ListItemBuilder::new()
            .headline("Item 2")
            .build(&theme)
    );
});
```

For very large lists, you can provide data-backed items and enable virtualization to keep
UI entity count roughly constant while scrolling:

```rust
let items: Vec<ListItemBuilder> = (0..10_000)
    .map(|i| {
        if i % 3 == 0 {
            ListItemBuilder::new(format!("Item {i}"))
                .two_line()
                .supporting_text("Supporting text")
        } else {
            ListItemBuilder::new(format!("Item {i}"))
        }
    })
    .collect();

commands.spawn(
    ListBuilder::new()
        .max_height(360.0)
        .items_from_builders(items)
        .virtualize(true)
        .overscan_rows(3)
        .build_scrollable(),
);
```

---

### Menus

Menus display a list of choices on a temporary surface.

```rust
let menu = MenuBuilder::new()
    .anchor(MenuAnchor::TopLeft)
    .build(&theme);

commands.spawn(menu).with_children(|parent| {
    parent.spawn(
        MenuItemBuilder::new()
            .label("Edit")
            .leading_icon("edit")
            .build(&theme)
    );

    parent.spawn(create_menu_divider(&theme));

    parent.spawn(
        MenuItemBuilder::new()
            .label("Delete")
            .leading_icon("delete")
            .build(&theme)
    );
});
```

---

### Tabs

Tabs organize content across different screens.

| Variant | Use Case |
|---------|----------|
| **Primary** | Top-level destinations |
| **Secondary** | Sub-pages or related content |

```rust
let tabs = TabsBuilder::new()
    .primary()
    .build(&theme);

commands.spawn(tabs).with_children(|parent| {
    parent.spawn(TabBuilder::new(0, "Tab 1").selected(true).build(&theme));
    parent.spawn(TabBuilder::new(1, "Tab 2").selected(false).build(&theme));
    parent.spawn(TabBuilder::new(2, "Tab 3").selected(false).build(&theme));
});

// Handle tab changes
fn handle_tabs(mut events: MessageReader<TabChangeEvent>) {
    for event in events.read() {
        println!("Selected tab index: {}", event.index);
    }
}
```

---

### Dividers

Dividers separate content into groups.

```rust
// Full-width divider
commands.spawn(horizontal_divider(&theme));

// Inset divider (with margins)
commands.spawn(inset_divider(&theme));

// Vertical divider
commands.spawn(vertical_divider(&theme));
```

---

### Select

Select components let users choose from a dropdown list.

```rust
let options = vec![
    SelectOption::new("United States").value("us"),
    SelectOption::new("United Kingdom").value("uk"),
    SelectOption::new("Canada").value("ca"),
];

let select = SelectBuilder::new(options)
    .label("Country")
    .selected(0)
    .build(&theme);

// For large option lists, you can constrain dropdown height and enable scrolling.
// If you have many thousands of options, you can also enable virtualization to keep
// the number of spawned UI rows roughly constant.
let many_options_select = SelectBuilder::new((0..5000).map(|i| SelectOption::new(format!("Option {i}"))).collect())
    .label("Many Options")
    .dropdown_max_height(Val::Px(240.0))
    .virtualize(true)
    .build(&theme);

fn handle_select(mut events: MessageReader<SelectChangeEvent>) {
    for event in events.read() {
        let value = event.option.value.as_deref().unwrap_or("");
        println!("Selected: {} ({})", event.option.label, value);
    }
}
```

---

## Color System

### HCT Color Space

The library uses the HCT (Hue, Chroma, Tone) color space from Material Design 3:

- **Hue**: 0-360° color wheel position
- **Chroma**: Color intensity (0 = gray)
- **Tone**: Lightness (0 = black, 100 = white)

```rust
use bevy_material_ui::color::Hct;

// Create a color
let teal = Hct::new(180.0, 36.0, 50.0);

// Convert to sRGB
let color: Color = teal.to_color();

// Get tonal variations
let light_teal = Hct::new(180.0, 36.0, 90.0);  // Lighter
let dark_teal = Hct::new(180.0, 36.0, 20.0);   // Darker
```

### Tonal Palettes

Generate consistent tonal scales:

```rust
use bevy_material_ui::color::TonalPalette;

let palette = TonalPalette::from_hue_and_chroma(265.0, 48.0);

// Access specific tones
let tone_50 = palette.tone(50);
let tone_90 = palette.tone(90);
```

---

## Icons

Material Symbols icons are supported via the icon system:

```rust
use bevy_material_ui::icons::{MaterialIcon, IconStyle, IconWeight};

// Create an icon
let icon = MaterialIcon::new("favorite")
    .style(IconStyle::Rounded)
    .weight(IconWeight::Regular)
    .size(24.0);

// Use with icon button
let button = IconButtonBuilder::new("settings")
    .filled()
    .build(&theme);
```

### Icon Styles

| Style | Description |
|-------|-------------|
| `Outlined` | Clean, simple lines |
| `Rounded` | Softer, rounded corners |
| `Sharp` | Angular, geometric |

---

## Accessibility

### Focus Rings

Focus rings indicate keyboard navigation:

```rust
commands.spawn((
    MaterialButton::new("Focusable"),
    Focusable,           // Makes it focusable
    FocusRing::new(),    // Shows focus indicator
    // ...
));
```

### ARIA Labels

When creating custom components, ensure proper labeling:

```rust
// Good: Descriptive button text
MaterialButton::new("Submit Order")

// For icon-only buttons, use aria-label equivalent
MaterialIconButton::new("close")  // Ensure screen reader text is provided
```

---

## WebGL Deployment

### Building for Web

Bevy supports WebGL/WebGPU via wasm:

```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen
cargo install wasm-bindgen-cli

# Build
cargo build --release --target wasm32-unknown-unknown --example showcase

# Generate JS bindings
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/examples/showcase.wasm
```

### GitHub Pages

GitHub doesn't support embedded WebGL directly in markdown, but you can:

1. **GitHub Pages**: Deploy a WebGL build to `https://username.github.io/repo/`
2. **Link from README**: Add screenshots and links to the live demo
3. **GitHub Actions**: Auto-deploy on push

Example GitHub Actions workflow (`.github/workflows/deploy.yml`):

```yaml
name: Deploy WebGL to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown

      - name: Build WASM
        run: |
          cargo build --release --target wasm32-unknown-unknown --example showcase
          wasm-bindgen --out-dir ./web --target web target/wasm32-unknown-unknown/release/examples/showcase.wasm

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./web
```

### HTML Wrapper

Create `web/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Bevy Material UI Showcase</title>
    <style>
        body { margin: 0; }
        canvas { width: 100vw; height: 100vh; display: block; }
    </style>
</head>
<body>
    <script type="module">
        import init from './showcase.js';
        init();
    </script>
</body>
</html>
```

---

## Additional Resources

- [Material Design 3 Guidelines](https://m3.material.io/)
- [Material Web Components](https://github.com/material-components/material-web)
- [Bevy Engine](https://bevyengine.org/)
- [Bevy UI Guide](https://bevyengine.org/learn/book/getting-started/ecs/)

---

## Contributing

See the main repository for contribution guidelines. When adding new components:

1. Follow the existing builder pattern
2. Add comprehensive event support
3. Ensure theme integration
4. Add examples to the showcase
5. Document all public APIs
