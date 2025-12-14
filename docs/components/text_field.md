# Text Field

Material Design 3 text input component.

## Variants

| Variant | Description |
|---------|-------------|
| `Filled` | Filled background style |
| `Outlined` | Border outline style |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::InputType;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|ui| {
        ui.spawn_text_field_with(
            &theme,
            TextFieldBuilder::new()
                .label("Email")
                .placeholder("name@example.com")
                .supporting_text("We'll never share your email")
                .input_type(InputType::Email)
                .outlined(),
        );
    });
}
```

## Label, Hint, Placeholder

When you provide both a `label` and a `placeholder`, the behavior matches Material-style semantics:

- Empty + unfocused: the label is shown *inside* the field as the expanded hint.
- Focused (or has content): the label “floats” above the field.
- Empty + focused while label is floating: the placeholder can be shown as a separate overlay.

The caret blink is implemented in a layout-stable way so the label does not “bounce” while the cursor toggles.

## With Icons

```rust
// Leading icon
MaterialTextField::new("Search")
    .leading_icon(ICON_SEARCH)
    .spawn(&mut commands, &theme);

// Trailing icon
MaterialTextField::new("Password")
    .trailing_icon(ICON_VISIBILITY)
    .spawn(&mut commands, &theme);
```

## With Helper Text

```rust
MaterialTextField::new("Email")
    .helper_text("We'll never share your email")
    .spawn(&mut commands, &theme);
```

## With Character Counter

```rust
MaterialTextField::new("Bio")
    .max_length(200)
    .show_counter(true)
    .spawn(&mut commands, &theme);
```

## Error State

```rust
MaterialTextField::new("Email")
    .error(true)
    .error_text("Please enter a valid email")
    .spawn(&mut commands, &theme);
```

## Disabled State

```rust
MaterialTextField::new("Disabled Field")
    .disabled(true)
    .value("Cannot edit this")
    .spawn(&mut commands, &theme);
```

## Password Field

```rust
MaterialTextField::new("Password")
    .password(true)
    .spawn(&mut commands, &theme);
```

## Multiline

```rust
MaterialTextField::new("Description")
    .multiline(true)
    .min_lines(3)
    .max_lines(10)
    .spawn(&mut commands, &theme);
```

## Handling Input

```rust
use bevy_material_ui::text_field::TextFieldChangeEvent;

fn handle_text_changes(
    mut reader: EventReader<TextFieldChangeEvent>,
) {
    for event in reader.read() {
        println!("Text changed to: {}", event.value);
    }
}
```

## Reading Values

```rust
fn read_text_fields(
    fields: Query<&MaterialTextField>,
) {
    for field in fields.iter() {
        println!("Field '{}' value: {}", field.label, field.value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `Option<String>` | `None` | Floating label / hint |
| `variant` | `TextFieldVariant` | `Filled` | Visual style |
| `value` | `String` | `""` | Current text value |
| `placeholder` | `String` | `""` | Placeholder text |
| `leading_icon` | `Option<String>` | `None` | Left icon |
| `trailing_icon` | `Option<String>` | `None` | Right icon |
| `supporting_text` | `Option<String>` | `None` | Supporting text below |
| `error` | `bool` | `false` | Error state |
| `error_text` | `Option<String>` | `None` | Error message |
| `disabled` | `bool` | `false` | Disabled state |
| `input_type` | `InputType` | `Text` | Keyboard + obscuring behavior |
| `max_length` | `Option<usize>` | `None` | Maximum characters |
| `counter_enabled` | `bool` | `false` | Show character counter |

## TextFieldChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The text field entity |
| `value` | `String` | New text value |
