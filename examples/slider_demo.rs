//! Slider Demo
//!
//! Demonstrates Material Design 3 sliders.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::slider::{spawn_slider_control_with, TickVisibility};

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

    // Opt-in tracing: `MUI_SLIDER_TRACE=1` (and set `RUST_LOG=bevy_material_ui::slider=info`).
    if std::env::var("MUI_SLIDER_TRACE").is_ok() {
        commands.insert_resource(SliderTraceSettings {
            enabled: true,
            ..default()
        });
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("slider_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                width: Val::Percent(100.0),
                max_width: Val::Px(520.0),
                ..default()
            })
            .with_children(|col| {
                // Continuous
                col.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|container| {
                    container.spawn((
                        Text::new("Continuous"),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    container
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(32.0),
                            ..default()
                        })
                        .with_children(|slot| {
                            let slider = MaterialSlider::new(0.0, 100.0).with_value(40.0);
                            spawn_slider_control_with(
                                slot,
                                &theme,
                                slider,
                                TestId::new("slider_demo/continuous"),
                            );
                        });
                });

                // Discrete
                col.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|container| {
                    container.spawn((
                        Text::new("Discrete"),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    container
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(32.0),
                            ..default()
                        })
                        .with_children(|slot| {
                            let mut slider = MaterialSlider::new(0.0, 100.0)
                                .with_value(60.0)
                                .with_step(20.0);
                            slider.show_ticks = true;
                            slider.tick_visibility = TickVisibility::Always;

                            spawn_slider_control_with(
                                slot,
                                &theme,
                                slider,
                                TestId::new("slider_demo/discrete"),
                            );
                        });
                });

                // Vertical
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Vertical"),
                        TextFont {
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        Node {
                            width: Val::Px(64.0),
                            ..default()
                        },
                    ));

                    row.spawn(Node {
                        width: Val::Px(48.0),
                        height: Val::Px(220.0),
                        ..default()
                    })
                    .with_children(|slot| {
                        let slider = MaterialSlider::new(0.0, 1.0).with_value(0.5).vertical();
                        spawn_slider_control_with(
                            slot,
                            &theme,
                            slider,
                            TestId::new("slider_demo/vertical"),
                        );
                    });
                });
            });
        });
}
