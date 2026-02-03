//! Snackbar Demo
//!
//! Demonstrates snackbars with a simple options panel (duration + action toggle).

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::icons::ICON_CLOSE;
use bevy_material_ui::prelude::*;

#[derive(Resource, Debug, Clone)]
struct SnackbarDemoState {
    duration_seconds: f32,
    show_action: bool,
}

impl Default for SnackbarDemoState {
    fn default() -> Self {
        Self {
            duration_seconds: 4.0,
            show_action: false,
        }
    }
}

#[derive(Component)]
struct SnackbarTrigger;

#[derive(Component)]
struct SnackbarActionToggle;

#[derive(Component)]
struct SnackbarDurationOption(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                snackbar_options_system,
                snackbar_trigger_system,
                handle_snackbar_action_system,
            ),
        )
        .init_resource::<SnackbarDemoState>()
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    // Root container
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
        .insert_test_id("snackbar_demo/root", &telemetry)
        .with_children(|root| {
            // Host lives in the UI tree; snackbars are spawned under it.
            root.spawn_snackbar_host(SnackbarPosition::BottomCenter);

            // Options panel
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|options| {
                // Duration options
                options
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Duration:"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        for (label, duration) in
                            [("2s", 2.0_f32), ("4s", 4.0_f32), ("10s", 10.0_f32)]
                        {
                            let is_default = (duration - 4.0).abs() < 0.01;
                            let chip_for_color =
                                MaterialChip::filter(label).with_selected(is_default);
                            let label_color = chip_for_color.label_color(&theme);

                            row.spawn((
                                SnackbarDurationOption(duration),
                                Interaction::None,
                                ChipBuilder::filter(label)
                                    .selected(is_default)
                                    .build(&theme),
                            ))
                            .with_children(|chip| {
                                chip.spawn((
                                    ChipLabel,
                                    Text::new(label),
                                    TextFont {
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(label_color),
                                ));
                            });
                        }
                    });

                // Action toggle
                options
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Show action:"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        let toggle_label = "Toggle Action";
                        let chip_for_color =
                            MaterialChip::filter(toggle_label).with_selected(false);
                        let label_color = chip_for_color.label_color(&theme);

                        row.spawn((
                            SnackbarActionToggle,
                            Interaction::None,
                            ChipBuilder::filter(toggle_label)
                                .selected(false)
                                .build(&theme),
                        ))
                        .with_children(|chip| {
                            chip.spawn((
                                ChipLabel,
                                Text::new(toggle_label),
                                TextFont {
                                    font_size: FontSize::Px(12.0),
                                    ..default()
                                },
                                TextColor(label_color),
                            ));
                        });
                    });
            });

            // Trigger button
            root.spawn(Node {
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            })
            .with_children(|row| {
                let trigger_label = "Show Snackbar";
                let trigger_button =
                    MaterialButton::new(trigger_label).with_variant(ButtonVariant::Filled);
                let trigger_text_color = trigger_button.text_color(&theme);

                row.spawn((
                    SnackbarTrigger,
                    Interaction::None,
                    MaterialButtonBuilder::new(trigger_label)
                        .filled()
                        .build(&theme),
                ))
                .insert_test_id("snackbar_demo/show", &telemetry)
                .with_children(|btn| {
                    btn.spawn((
                        ButtonLabel,
                        Text::new(trigger_label),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(trigger_text_color),
                    ));
                });
            });

            // Snackbar preview (static example)
            root.spawn((
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(48.0),
                    padding: UiRect::horizontal(Val::Px(16.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme.inverse_surface),
                BorderRadius::all(Val::Px(4.0)),
                BoxShadow::from(ShadowStyle {
                    color: Color::BLACK.with_alpha(0.2),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(2.0),
                    spread_radius: Val::Px(0.0),
                    blur_radius: Val::Px(4.0),
                }),
            ))
            .with_children(|snackbar| {
                snackbar.spawn((
                    Text::new("Item deleted"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.inverse_on_surface),
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));

                snackbar
                    .spawn((
                        Interaction::None,
                        MaterialButtonBuilder::new("UNDO").text().build(&theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new("UNDO"),
                            TextFont {
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                            TextColor(theme.inverse_primary),
                        ));
                    });

                snackbar
                    .spawn((
                        Button,
                        Interaction::None,
                        Node {
                            width: Val::Px(32.0),
                            height: Val::Px(32.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(9999.0)),
                    ))
                    .with_children(|btn| {
                        if let Some(icon) = MaterialIcon::from_name(ICON_CLOSE) {
                            btn.spawn(icon.with_size(24.0).with_color(theme.inverse_on_surface));
                        }
                    });
            });
        });
}

fn snackbar_options_system(
    mut state: ResMut<SnackbarDemoState>,
    mut duration_clicks: Query<
        (&Interaction, &SnackbarDurationOption),
        (Changed<Interaction>, With<MaterialChip>),
    >,
    mut duration_chips: Query<(&SnackbarDurationOption, &mut MaterialChip)>,
    mut action_clicks: Query<
        (&Interaction, Entity),
        (
            Changed<Interaction>,
            With<SnackbarActionToggle>,
            With<MaterialChip>,
        ),
    >,
    mut action_chips: Query<&mut MaterialChip, With<SnackbarActionToggle>>,
) {
    let mut new_duration = None;
    for (interaction, opt) in duration_clicks.iter_mut() {
        if *interaction == Interaction::Pressed {
            new_duration = Some(opt.0);
        }
    }

    if let Some(duration) = new_duration {
        state.duration_seconds = duration;
        for (opt, mut chip) in duration_chips.iter_mut() {
            chip.selected = (opt.0 - duration).abs() < 0.01;
        }
    }

    let mut toggled = false;
    for (interaction, _entity) in action_clicks.iter_mut() {
        if *interaction == Interaction::Pressed {
            toggled = true;
        }
    }

    if toggled {
        state.show_action = !state.show_action;
        for mut chip in action_chips.iter_mut() {
            chip.selected = state.show_action;
        }
    }
}

fn snackbar_trigger_system(
    state: Res<SnackbarDemoState>,
    mut clicks: Query<(&Interaction, Entity), (Changed<Interaction>, With<SnackbarTrigger>)>,
    mut show: MessageWriter<ShowSnackbar>,
) {
    for (interaction, _entity) in clicks.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let event = if state.show_action {
            ShowSnackbar::with_action("Saved successfully", "UNDO")
        } else {
            ShowSnackbar::message("Item deleted")
        }
        .duration(state.duration_seconds);

        show.write(event);
    }
}

fn handle_snackbar_action_system(mut actions: MessageReader<SnackbarActionEvent>) {
    for ev in actions.read() {
        info!("Snackbar action clicked: {}", ev.action);
    }
}
