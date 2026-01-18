//! Icons Demo
//!
//! Demonstrates using embedded Material icons via `MaterialIcon::from_name`.

use bevy::prelude::*;
use bevy_material_ui::icons::MaterialIcon;
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

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("icons_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Icons"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.0),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container),
                BorderRadius::all(Val::Px(12.0)),
            ))
            .with_children(|row| {
                for icon_name in ["check", "home", "settings", "favorite", "search"] {
                    row.spawn((
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(48.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_high),
                        BorderRadius::all(Val::Px(8.0)),
                    ))
                    .with_children(|cell| {
                        if let Some(icon) = MaterialIcon::from_name(icon_name) {
                            cell.spawn(icon.with_size(24.0).with_color(theme.on_surface));
                        } else {
                            cell.spawn((
                                Text::new("?"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(theme.error),
                            ));
                        }
                    });
                }
            });
        });
}
