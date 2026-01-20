//! Switch Demo
//!
//! Demonstrates Material Design 3 switches with icons and without.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    let root_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("switch_demo/root", &telemetry)
        .id();

    let section_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(560.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            ChildOf(root_entity),
        ))
        .id();

    let col_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            ChildOf(section_entity),
        ))
        .insert_test_id("switch_demo/column", &telemetry)
        .id();

    // Match the showcase example.
    let wifi = commands.spawn_switch_with(&theme, SwitchBuilder::new().selected(true), "Wi-Fi");
    commands
        .entity(wifi)
        .insert_test_id("switch_demo/switch/wifi", &telemetry);
    commands.entity(col_entity).add_child(wifi);

    let bluetooth =
        commands.spawn_switch_with(&theme, SwitchBuilder::new().selected(false), "Bluetooth");
    commands
        .entity(bluetooth)
        .insert_test_id("switch_demo/switch/bluetooth", &telemetry);
    commands.entity(col_entity).add_child(bluetooth);

    let dark_mode =
        commands.spawn_switch_with(&theme, SwitchBuilder::new().selected(false), "Dark Mode");
    commands
        .entity(dark_mode)
        .insert_test_id("switch_demo/switch/dark_mode", &telemetry);
    commands.entity(col_entity).add_child(dark_mode);
}
