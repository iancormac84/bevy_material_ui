//! Badge Demo
//!
//! Demonstrates Material badges (dot, count, text).

use bevy::prelude::*;
use bevy_material_ui::icons::ICON_NOTIFICATIONS;
use bevy_material_ui::prelude::*;

fn spawn_badge_example(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    count: Option<&str>,
    test_id: &str,
    telemetry: &TelemetryConfig,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.surface_container),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .insert_test_id(test_id, telemetry)
        .with_children(|container| {
            if let Some(icon) = MaterialIcon::from_name(ICON_NOTIFICATIONS) {
                container.spawn(icon.with_size(24.0).with_color(theme.on_surface));
            }

            match count {
                None => {
                    container.spawn_small_badge(theme);
                }
                Some("99+") => {
                    container.spawn_badge_with(theme, BadgeBuilder::count(150).max(99));
                }
                Some(c) => {
                    let parsed = c.parse::<u32>().unwrap_or(0);
                    container.spawn_badge_count(theme, parsed);
                }
            }
        });
}

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
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("badge_demo/root", &telemetry)
        .with_children(|root| {
            // Dot badge
            spawn_badge_example(root, &theme, None, "badge_demo/container/dot", &telemetry);

            // Small count
            spawn_badge_example(
                root,
                &theme,
                Some("3"),
                "badge_demo/container/count",
                &telemetry,
            );

            // Large count
            spawn_badge_example(
                root,
                &theme,
                Some("99+"),
                "badge_demo/container/text",
                &telemetry,
            );
        });
}
