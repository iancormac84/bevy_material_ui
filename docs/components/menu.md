# Menu

Material Design 3 dropdown menu component.

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|ui| {
        ui.spawn_menu(&theme, |menu| {
            menu.spawn_menu_item_with(
                &theme,
                MenuItemBuilder::new("Cut").leading_icon(ICON_CONTENT_CUT),
            );
            menu.spawn_menu_item_with(
                &theme,
                MenuItemBuilder::new("Copy").leading_icon(ICON_CONTENT_COPY),
            );
            menu.spawn_menu_item_with(
                &theme,
                MenuItemBuilder::new("Paste").leading_icon(ICON_CONTENT_PASTE),
            );
            menu.spawn_menu_divider(&theme);
            menu.spawn_menu_item_with(
                &theme,
                MenuItemBuilder::new("Delete").leading_icon(ICON_DELETE),
            );
        });
    });
}
```

## Nested Menus

Nested submenus are not implemented yet.

## Disabled Items

```rust
commands.spawn(Node::default()).with_children(|ui| {
    ui.spawn_menu(&theme, |menu| {
        menu.spawn_menu_item(&theme, "Active");
        menu.spawn_menu_item_with(&theme, MenuItemBuilder::new("Disabled").disabled(true));
    });
});
```

## Handling Selection

```rust
use bevy_material_ui::menu::MenuItemSelectEvent;

fn handle_menu_selection(
    mut reader: EventReader<MenuItemSelectEvent>,
) {
    for event in reader.read() {
        println!("Menu item selected: {}", event.label);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `anchor` | `MenuAnchor` | `BottomLeft` | Where the menu opens relative to its parent |
| `open` | `bool` | `false` | Visibility state |

## MenuItem Types

| Type | Description |
|------|-------------|
| `Item` | Regular clickable item |
| `Divider` | Visual separator |
| `Submenu` | Nested menu |
