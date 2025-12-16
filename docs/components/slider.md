# Slider

Material Design 3 slider component for range selection.

## Types

| Type | Description |
|------|-------------|
| Continuous | Smooth value selection |
| Discrete | Step-based selection with tick marks |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Continuous slider (0-100)
    MaterialSlider::new(0.0, 100.0).spawn(&mut commands, &theme);

    // With initial value
    MaterialSlider::new(0.0, 100.0).with_value(50.0).spawn(&mut commands, &theme);
}
```

## Custom Range

```rust
// Slider from 0 to 1000
MaterialSlider::new(0.0, 1000.0)
    .with_value(500.0)
    .spawn(&mut commands, &theme);
```

## Discrete Slider

```rust
// Discrete slider with a fixed step size
MaterialSlider::new(0.0, 10.0)
    .with_step(1.0)
    .show_ticks()
    .spawn(&mut commands, &theme);

// Discrete slider with a specific count of discrete values
MaterialSlider::new(0.0, 10.0)
    .discrete(11)
    .show_ticks()
    .spawn(&mut commands, &theme);
```

## With Label

```rust
MaterialSlider::new(0.0, 100.0)
    .show_label()
    .spawn(&mut commands, &theme);
```

## Disabled State

```rust
MaterialSlider::new(0.0, 100.0)
    .with_value(30.0)
    .disabled(true)
    .spawn(&mut commands, &theme);

## Orientation and Direction

```rust
use bevy_material_ui::prelude::*;

// Vertical slider
MaterialSlider::new(0.0, 1.0).vertical().spawn(&mut commands, &theme);

// Reversed direction (values increase end -> start)
MaterialSlider::new(0.0, 1.0).reversed().spawn(&mut commands, &theme);
```

## Standalone Spawn Helpers

If you're spawning sliders inside existing layouts and want just the control
(no label wrapper), use:

```rust
use bevy_material_ui::prelude::*;

commands.spawn(Node::default()).with_children(|ui| {
    let slider_entity = bevy_material_ui::slider::spawn_slider_control(
        ui,
        &theme,
        MaterialSlider::new(0.0, 100.0),
    );

    let _slider_entity_with_extra = bevy_material_ui::slider::spawn_slider_control_with(
        ui,
        &theme,
        MaterialSlider::new(0.0, 100.0),
        MyMarker,
    );
});

#[derive(Component)]
struct MyMarker;
```
```

## Handling Changes

```rust
use bevy_material_ui::slider::SliderChangeEvent;

fn handle_slider_changes(
    mut reader: EventReader<SliderChangeEvent>,
) {
    for event in reader.read() {
        println!("Slider value: {}", event.value);
    }
}
```

## Reading Current Value

```rust
fn read_slider_values(
    sliders: Query<&MaterialSlider>,
) {
    for slider in sliders.iter() {
        println!("Current value: {}", slider.value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `value` | `f32` | `min` | Current value |
| `min` | `f32` | `0.0` | Minimum value |
| `max` | `f32` | `100.0` | Maximum value |
| `step` | `Option<f32>` | `None` | Step size for discrete sliders |
| `discrete_value_count` | `Option<usize>` | `None` | Number of discrete values |
| `disabled` | `bool` | `false` | Disabled state |
| `orientation` | `SliderOrientation` | `Horizontal` | Slider orientation |
| `direction` | `SliderDirection` | `StartToEnd` | Value increase direction |

## SliderChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The slider entity |
| `value` | `f32` | New slider value |
| `normalized` | `f32` | Value from 0.0 to 1.0 |

## Visual Elements

- **Track**: Background line
- **Active Track**: Filled portion showing current value
- **Handle**: Draggable thumb
- **Tick Marks**: Discrete step indicators (discrete mode only)
