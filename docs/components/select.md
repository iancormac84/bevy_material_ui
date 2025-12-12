# Select

Material Design 3 dropdown select component.

## Variants

| Variant | Description |
|---------|-------------|
| `Filled` | Filled text field style |
| `Outlined` | Outlined text field style |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    let options = vec![
        SelectOption::new("Option 1").value("option1"),
        SelectOption::new("Option 2").value("option2"),
        SelectOption::new("Option 3").value("option3"),
    ];

    commands.spawn(Node::default()).with_children(|ui| {
        ui.spawn_filled_select(&theme, "Choose an option", options);
    });
}
```

## With Default Value

```rust
let options = vec![
    SelectOption::new("United States").value("us"),
    SelectOption::new("United Kingdom").value("uk"),
    SelectOption::new("Canada").value("ca"),
];

commands.spawn(Node::default()).with_children(|ui| {
    ui.spawn_select_with(
        &theme,
        SelectBuilder::new(options)
            .label("Country")
            .selected(0),
    );
});
```

## Outlined Variant

```rust
let options = vec![
    SelectOption::new("Category 1").value("cat1"),
    SelectOption::new("Category 2").value("cat2"),
];

commands.spawn(Node::default()).with_children(|ui| {
    ui.spawn_outlined_select(&theme, "Category", options);
});
```

## With Icons

```rust
let options = vec![
    SelectOption::new("High").value("high").icon(ICON_PRIORITY_HIGH),
    SelectOption::new("Medium").value("medium").icon(ICON_REMOVE),
    SelectOption::new("Low").value("low").icon(ICON_PRIORITY_LOW),
];

commands.spawn(Node::default()).with_children(|ui| {
    ui.spawn_filled_select(&theme, "Priority", options);
});
```

## Disabled State

```rust
let options = vec![SelectOption::new("A").value("a")];

commands.spawn(Node::default()).with_children(|ui| {
    ui.spawn_select_with(
        &theme,
        SelectBuilder::new(options)
            .label("Disabled Select")
            .disabled(true),
    );
});
```

## Handling Selection

```rust
use bevy_material_ui::select::SelectChangeEvent;

fn handle_select_changes(
    mut reader: EventReader<SelectChangeEvent>,
) {
    for event in reader.read() {
        let value = event.option.value.as_deref().unwrap_or("");
        println!("Selected: {} ({})", event.option.label, value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `Option<String>` | `None` | Field label |
| `variant` | `SelectVariant` | `Filled` | Visual style |
| `options` | `Vec<SelectOption>` | Required | Available options |
| `selected_index` | `Option<usize>` | `None` | Selected option index |
| `disabled` | `bool` | `false` | Disabled state |
| `error` | `bool` | `false` | Error state |
| `supporting_text` | `Option<String>` | `None` | Supporting text below |

## SelectChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The select entity |
| `index` | `usize` | Selected option index |
| `option` | `SelectOption` | Selected option data |
