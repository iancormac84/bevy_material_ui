//! Progress Demo
//!
//! Demonstrates Material Design 3 progress indicators.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, progress_demo_animate_system)
        .run();
}

/// Marker for progress bars animated by the demo.
#[derive(Component, Clone, Copy)]
pub struct DemoProgressOscillator {
    pub speed: f32,
    pub direction: f32,
    pub label: Entity,
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
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("progress_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                width: Val::Percent(100.0),
                max_width: Val::Px(400.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            })
            .with_children(|col| {
                // Animated determinate progress (oscillates up/down)
                spawn_animated_linear_progress(
                    col,
                    &theme,
                    &telemetry,
                    "progress_demo/linear/animated_1",
                    0.15,
                    0.35,
                );
                spawn_animated_linear_progress(
                    col,
                    &theme,
                    &telemetry,
                    "progress_demo/linear/animated_2",
                    0.75,
                    0.55,
                );

                // Indeterminate example
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Indeterminate"),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        Node {
                            width: Val::Px(90.0),
                            ..default()
                        },
                    ));

                    row.spawn(
                        LinearProgressBuilder::new()
                            .indeterminate()
                            .width(Val::Px(200.0))
                            .height_px(8.0)
                            .build(&theme),
                    )
                    .insert_test_id("progress_demo/linear/indeterminate", &telemetry);
                });
            });
        });
}

fn spawn_animated_linear_progress(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    telemetry: &TelemetryConfig,
    test_id: &str,
    initial: f32,
    speed: f32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            let label_entity = row
                .spawn((
                    Text::new(format!(
                        "{:>3}%",
                        (initial.clamp(0.0, 1.0) * 100.0).round() as i32
                    )),
                    TextFont {
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                    Node {
                        width: Val::Px(48.0),
                        ..default()
                    },
                ))
                .id();

            row.spawn((
                DemoProgressOscillator {
                    speed,
                    direction: 1.0,
                    label: label_entity,
                },
                LinearProgressBuilder::new()
                    .progress(initial)
                    .width(Val::Px(200.0))
                    .height_px(8.0)
                    .build(theme),
            ))
            .insert_test_id(test_id, telemetry);
        });
}

fn progress_demo_animate_system(
    time: Res<Time>,
    mut bars: Query<(&mut MaterialLinearProgress, &mut DemoProgressOscillator)>,
    mut labels: Query<&mut Text>,
) {
    for (mut progress, mut osc) in bars.iter_mut() {
        if progress.mode != ProgressMode::Determinate {
            continue;
        }

        let mut value = progress.progress + osc.direction * osc.speed * time.delta_secs();
        if value >= 1.0 {
            value = 1.0;
            osc.direction = -1.0;
        } else if value <= 0.0 {
            value = 0.0;
            osc.direction = 1.0;
        }

        progress.progress = value;

        if let Ok(mut text) = labels.get_mut(osc.label) {
            *text = Text::new(format!("{:>3}%", (value * 100.0).round() as i32));
        }
    }
}
