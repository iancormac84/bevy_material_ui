//! Ripple Demo
//!
//! Demonstrates the ripple interaction effect.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct RippleDemoButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ripple_input_system)
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
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("ripple_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Click a surface to spawn a ripple"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                ..default()
            })
            .with_children(|row| {
                spawn_ripple_surface(
                    row,
                    &theme,
                    "Primary",
                    theme.primary,
                    theme.on_primary,
                    theme.on_primary,
                    "ripple_demo/primary",
                    &telemetry,
                );

                spawn_ripple_surface(
                    row,
                    &theme,
                    "Secondary",
                    theme.secondary,
                    theme.on_secondary,
                    theme.on_secondary,
                    "ripple_demo/secondary",
                    &telemetry,
                );
            });
        });
}

fn spawn_ripple_surface(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    surface_color: Color,
    ripple_color: Color,
    text_color: Color,
    test_id: &str,
    telemetry: &TelemetryConfig,
) {
    parent
        .spawn((
            RippleDemoButton,
            RippleHost::new().with_color(ripple_color),
            Interaction::None,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(88.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(surface_color),
            BorderRadius::all(Val::Px(16.0)),
        ))
        .insert_test_id(test_id, telemetry)
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

fn ripple_input_system(
    mut events: MessageWriter<SpawnRipple>,
    buttons: Query<(Entity, &Interaction, &ComputedNode), (Changed<Interaction>, With<RippleDemoButton>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let scale = windows
        .single()
        .map(|window| window.scale_factor())
        .unwrap_or(1.0);

    for (entity, interaction, node) in buttons.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let size = node.size() / scale;
        let position = Vec2::new(size.x * 0.5, size.y * 0.5);
        events.write(SpawnRipple { host: entity, position });
    }
}
