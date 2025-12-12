# List

Material Design 3 list component with selection support.

## Features

- Single and multi-select modes
- Leading/trailing icons and avatars
- Supporting text
- Dividers between items

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands
        .spawn(ListBuilder::new().build())
        .with_children(|list| {
            list.spawn_list_item(&theme, "Item 1", None::<String>);
            list.spawn_list_item(&theme, "Item 2", None::<String>);
            list.spawn_list_item(&theme, "Item 3", None::<String>);
        });
}
```

## With Supporting Text

```rust
commands
    .spawn(ListBuilder::new().build())
    .with_children(|list| {
        list.spawn_list_item(&theme, "Primary Text", Some("Secondary supporting text"));
        list.spawn_list_item(&theme, "Another Item", Some("More details here"));
    });
```

## With Icons

```rust
commands
    .spawn(ListBuilder::new().build())
    .with_children(|list| {
        list.spawn_list_item_with(
            &theme,
            ListItemBuilder::new("Settings").leading_icon(ICON_SETTINGS),
        );
        list.spawn_list_item_with(
            &theme,
            ListItemBuilder::new("Account")
                .leading_icon(ICON_PERSON)
                .trailing_icon(ICON_ARROW_FORWARD),
        );
    });
```

## With Avatars

```rust
// Avatars/images are not built into `ListItemBuilder` yet.
// Use a custom leading child (e.g. an `ImageNode`) inside the list item.
```

## With Dividers

```rust
commands
    .spawn(ListBuilder::new().build())
    .with_children(|list| {
        list.spawn_list_item(&theme, "Item 1", None::<String>);
        list.spawn_list_divider(&theme, false);
        list.spawn_list_item(&theme, "Item 2", None::<String>);
    });
```

## Scrollable List

```rust
commands
    .spawn(ListBuilder::new().max_height(300.0).build_scrollable())
    .with_children(|list| {
        list.spawn_list_item(&theme, "Item 1", None::<String>);
        // ... many items
    });
```

## Selection Modes

Selection is handled by the library. Set the mode on the list:

```rust
commands
    .spawn(ListBuilder::new().selection_mode(ListSelectionMode::Single).build())
    .with_children(|list| {
        list.spawn_list_item(&theme, "One", None::<String>);
        list.spawn_list_item(&theme, "Two", None::<String>);
    });

commands
    .spawn(ListBuilder::new().selection_mode(ListSelectionMode::Multi).build())
    .with_children(|list| {
        list.spawn_list_item(&theme, "A", None::<String>);
        list.spawn_list_item(&theme, "B", None::<String>);
    });
```

## Handling Item Clicks

```rust
use bevy_material_ui::list::ListItemClickEvent;

fn handle_list_item_clicks(
    mut reader: EventReader<ListItemClickEvent>,
) {
    for event in reader.read() {
        println!("List item clicked: {:?}", event.entity);
    }
}
```

## Properties

### ListBuilder

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `selection_mode` | `ListSelectionMode` | `None` | Selection behavior |
| `max_height` | `Option<f32>` | `None` | Max height for `build_scrollable()` |
| `show_scrollbar` | `bool` | `true` | Scrollbar visibility (scrolling still works if hidden) |

### ListItemBuilder

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `headline` | `String` | Required | Primary text |
| `supporting_text` | `Option<String>` | `None` | Secondary text |
| `leading_icon` | `Option<String>` | `None` | Left icon |
| `trailing_icon` | `Option<String>` | `None` | Right icon |
| `selected` | `bool` | `false` | Initial selected state |

## State Layers

List items apply MD3 state layers:
- **Hover**: Surface container high color
- **Pressed**: Surface container highest color
- **Selected**: Primary container color
