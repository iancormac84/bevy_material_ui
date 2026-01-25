//! Elevation Demo
//!
//! Demonstrates Material elevation levels and their shadows.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

const CARD_WIDTH: f32 = 140.0;
const CARD_HEIGHT: f32 = 96.0;

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

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(24.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("elevation_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Elevation levels"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(24.0),
                row_gap: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|grid| {
                let levels = [
                    ("Level 0", Elevation::Level0),
                    ("Level 1", Elevation::Level1),
                    ("Level 2", Elevation::Level2),
                    ("Level 3", Elevation::Level3),
                    ("Level 4", Elevation::Level4),
                    ("Level 5", Elevation::Level5),
                ];

                for (idx, (label, elevation)) in levels.iter().enumerate() {
                    spawn_elevation_card(
                        grid,
                        &theme,
                        label,
                        *elevation,
                        &format!("elevation_demo/card/{}", idx),
                        &telemetry,
                    );
                }
            });
        });
}

fn spawn_elevation_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    elevation: Elevation,
    test_id: &str,
    telemetry: &TelemetryConfig,
) {
    let dp_label = format!("{} ({}dp)", label, elevation.dp());

    parent
        .spawn((
            Node {
                width: Val::Px(CARD_WIDTH),
                height: Val::Px(CARD_HEIGHT),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(theme.surface_container),
            BorderRadius::all(Val::Px(12.0)),
            elevation.to_box_shadow(),
        ))
        .insert_test_id(test_id, telemetry)
        .with_children(|card| {
            card.spawn((
                Text::new(dp_label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
        });
}
