# Tabs

Material Design 3 tab navigation component.

## Types

| Type | Description |
|------|-------------|
| Primary | Top-level navigation |
| Secondary | Within a content area |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    let tabs_entity = commands.spawn(TabsBuilder::new().primary().build(&theme)).id();

    commands.entity(tabs_entity).with_children(|tabs| {
        tabs.spawn_tab_with(
            &theme,
            TabBuilder::new(0, "Home").selected(true).variant(TabVariant::Primary),
        );
        tabs.spawn_tab_with(
            &theme,
            TabBuilder::new(1, "Profile").selected(false).variant(TabVariant::Primary),
        );
        tabs.spawn_tab_with(
            &theme,
            TabBuilder::new(2, "Settings").selected(false).variant(TabVariant::Primary),
        );
    });
}
```

## With Icons

```rust
let tabs_entity = commands.spawn(TabsBuilder::new().primary().build(&theme)).id();

commands.entity(tabs_entity).with_children(|tabs| {
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(0, "Home")
            .selected(true)
            .icon(ICON_HOME)
            .variant(TabVariant::Primary),
    );
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(1, "Favorites")
            .selected(false)
            .icon(ICON_FAVORITE)
            .variant(TabVariant::Primary),
    );
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(2, "Settings")
            .selected(false)
            .icon(ICON_SETTINGS)
            .variant(TabVariant::Primary),
    );
});
```

## Secondary Tabs

```rust
let tabs_entity = commands.spawn(TabsBuilder::new().secondary().build(&theme)).id();

commands.entity(tabs_entity).with_children(|tabs| {
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(0, "All").selected(true).variant(TabVariant::Secondary),
    );
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(1, "Active").selected(false).variant(TabVariant::Secondary),
    );
    tabs.spawn_tab_with(
        &theme,
        TabBuilder::new(2, "Completed").selected(false).variant(TabVariant::Secondary),
    );
});
```

## Default Selected Tab

```rust
let tabs_entity = commands
    .spawn(TabsBuilder::new().primary().selected(1).build(&theme))
    .id();

commands.entity(tabs_entity).with_children(|tabs| {
    tabs.spawn_tab_with(&theme, TabBuilder::new(0, "Tab 1").selected(false));
    tabs.spawn_tab_with(&theme, TabBuilder::new(1, "Tab 2").selected(true));
    tabs.spawn_tab_with(&theme, TabBuilder::new(2, "Tab 3").selected(false));
});
```

## Scrollable Tabs

```rust
// Horizontal scrolling for tabs is not implemented yet.
// For now, wrap the tab bar in your own scroll container if needed.
```

## Handling Tab Changes

```rust
use bevy_material_ui::tabs::TabChangeEvent;

fn handle_tab_changes(
    mut reader: EventReader<TabChangeEvent>,
) {
    for event in reader.read() {
        println!("Tab changed to index: {}", event.index);
    }
}
```

## Tab Content Visibility

```rust
// Add `TabContent` to panels to have the library manage visibility.
// Each panel references its owning `tabs_entity`.

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    let tabs_entity = commands.spawn(TabsBuilder::new().primary().build(&theme)).id();

    // ...spawn tabs as children of `tabs_entity`...

    commands.spawn((
        TabContent::new(0, tabs_entity),
        Node { width: Val::Percent(100.0), ..default() },
    ));
    commands.spawn((
        TabContent::new(1, tabs_entity),
        Node { width: Val::Percent(100.0), ..default() },
    ));
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `variant` | `TabVariant` | `Primary` | Tab style |
| `selected` | `usize` | `0` | Selected tab index |

## Tab Structure

Use `MaterialTab`/`TabBuilder` to define each tab:

| Field | Type | Description |
|-------|------|-------------|
| `index` | `usize` | Tab index in the bar |
| `label` | `String` | Tab text |
| `icon` | `Option<String>` | Optional icon |

## TabChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `tabs_entity` | `Entity` | The tabs container entity |
| `tab_entity` | `Entity` | The selected tab entity |
| `index` | `usize` | New selected tab index |
