# Progress

Material Design 3 progress indicator components.

## Types

| Type | Description |
|------|-------------|
| Linear | Horizontal progress bar |
| Circular | Spinning circle indicator |

## Linear Progress

### Determinate

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|ui| {
        // 50% progress
        ui.spawn_linear_progress(&theme, 0.5);
    });
}
```

### Indeterminate

```rust
commands.spawn(Node::default()).with_children(|ui| {
    // Animated indeterminate progress
    ui.spawn_indeterminate_progress(&theme);
});
```

Note:

- The linear progress indicator fill is ensured automatically, even if you spawn only `LinearProgressBuilder::build()` directly.

## Circular Progress

### Determinate

```rust
commands.spawn(Node::default()).with_children(|ui| {
    // 75% circular progress
    ui.spawn_circular_progress(&theme, 0.75);
});
```

### Indeterminate

```rust
commands.spawn(Node::default()).with_children(|ui| {
    // Spinning indicator
    ui.spawn_indeterminate_circular_progress(&theme);
});
```

## Custom Size

```rust
commands.spawn(Node::default()).with_children(|ui| {
    // Custom width for linear
    ui.spawn_linear_progress_with(
        &theme,
        LinearProgressBuilder::new().progress(0.5).width(Val::Px(200.0)),
    );

    // Custom size for circular
    ui.spawn_circular_progress_with(
        &theme,
        CircularProgressBuilder::new().progress(0.5).size(64.0),
    );
});
```

## Custom Track Color

```rust
// Track/indicator colors are derived from the active theme.
// For per-instance customization, you can override colors by directly editing the spawned components.
```

## Updating Progress

```rust
fn update_progress(
    mut progress_query: Query<&mut MaterialLinearProgress>,
    time: Res<Time>,
) {
    for mut progress in progress_query.iter_mut() {
        progress.progress = (progress.progress + time.delta_secs() * 0.1) % 1.0;
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `progress` | `f32` | `0.0` | Progress value (0.0-1.0) |
| `mode` | `ProgressMode` | `Determinate` | Determinate vs indeterminate |
| `four_color` | `bool` | `false` | Four-color styling (reserved) |

## Animation

Indeterminate progress uses MD3 motion tokens for smooth animation.
