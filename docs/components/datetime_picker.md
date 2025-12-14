# DateTime Picker

Material Design-style date/time picker component.

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|ui| {
        ui.spawn_datetime_picker_with(
            &theme,
            DateTimePickerBuilder::new()
                .title("Select date and time")
                .confirm_text("OK")
                .cancel_text("Cancel"),
        );
    });
}
```

## Handling Events

```rust
use bevy_material_ui::datetime_picker::{
    DateTimePickerCancelEvent,
    DateTimePickerSubmitEvent,
};

fn handle_datetime_picker_events(
    mut submit: EventReader<DateTimePickerSubmitEvent>,
    mut cancel: EventReader<DateTimePickerCancelEvent>,
) {
    for event in submit.read() {
        // The event payload contains the selected date/time.
        println!("Submitted: {:?}", event);
    }

    for event in cancel.read() {
        println!("Cancelled: {:?}", event);
    }
}
```

## Showcase

The interactive showcase includes a full DateTime Picker view:

```bash
cargo run --example showcase
```
