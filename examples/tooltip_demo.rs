//! Tooltip Demo
//!
//! Demonstrates the canonical tooltip options + demo layout.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .init_resource::<TooltipDemoOptions>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tooltip_demo_options_system,
                tooltip_demo_apply_system,
                tooltip_demo_style_system,
            ),
        )
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
        .insert_test_id("tooltip_demo/root", &telemetry)
        .with_children(|root| {
            // Options panel
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(12.0)),
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(theme.surface_container_lowest),
                BorderRadius::all(Val::Px(12.0)),
            ))
            .insert_test_id("tooltip_demo/options", &telemetry)
            .with_children(|options| {
                // Position options
                options
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Position:"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        for (label, pos) in [
                            ("Top", TooltipPosition::Top),
                            ("Bottom", TooltipPosition::Bottom),
                            ("Left", TooltipPosition::Left),
                            ("Right", TooltipPosition::Right),
                        ] {
                            let selected = pos == TooltipPosition::Bottom;
                            let button = MaterialButton::new(label).with_variant(if selected {
                                ButtonVariant::FilledTonal
                            } else {
                                ButtonVariant::Outlined
                            });
                            let label_color = button.text_color(&theme);

                            row.spawn((
                                TooltipPositionOption(pos),
                                Interaction::None,
                                MaterialButtonBuilder::new(label)
                                    .variant(if selected {
                                        ButtonVariant::FilledTonal
                                    } else {
                                        ButtonVariant::Outlined
                                    })
                                    .build(&theme),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    ButtonLabel,
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

                // Delay options
                options
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Delay:"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        for (label, delay) in
                            [("0.15s", 0.15_f32), ("0.5s", 0.5_f32), ("1.0s", 1.0_f32)]
                        {
                            let selected = (delay - 0.5).abs() < 0.01;
                            let button = MaterialButton::new(label).with_variant(if selected {
                                ButtonVariant::FilledTonal
                            } else {
                                ButtonVariant::Outlined
                            });
                            let label_color = button.text_color(&theme);

                            row.spawn((
                                TooltipDelayOption(delay),
                                Interaction::None,
                                MaterialButtonBuilder::new(label)
                                    .variant(if selected {
                                        ButtonVariant::FilledTonal
                                    } else {
                                        ButtonVariant::Outlined
                                    })
                                    .build(&theme),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    ButtonLabel,
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
            });

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(32.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|row| {
                let demo_label = "Hover Me";
                let demo_button =
                    MaterialButton::new(demo_label).with_variant(ButtonVariant::Filled);
                let demo_text_color = demo_button.text_color(&theme);

                row.spawn((
                    TooltipDemoButton,
                    TooltipTrigger::new("Hover to see tooltip!").bottom(),
                    Interaction::None,
                    MaterialButtonBuilder::new(demo_label)
                        .filled()
                        .build(&theme),
                ))
                .insert_test_id("tooltip_demo/button", &telemetry)
                .with_children(|btn| {
                    btn.spawn((
                        ButtonLabel,
                        Text::new(demo_label),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(demo_text_color),
                    ));
                });

                row.spawn((
                    Text::new("← Hover to test tooltip with selected options"),
                    TextFont {
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));
            });
        });
}

#[derive(Resource)]
struct TooltipDemoOptions {
    position: TooltipPosition,
    delay: f32,
}

impl Default for TooltipDemoOptions {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Bottom,
            delay: 0.5,
        }
    }
}

#[derive(Component, Copy, Clone)]
struct TooltipPositionOption(pub TooltipPosition);

#[derive(Component, Copy, Clone)]
struct TooltipDelayOption(pub f32);

#[derive(Component)]
struct TooltipDemoButton;

fn tooltip_demo_options_system(
    mut options: ResMut<TooltipDemoOptions>,
    position_buttons: Query<(&TooltipPositionOption, &Interaction), Changed<Interaction>>,
    delay_buttons: Query<(&TooltipDelayOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }

    for (opt, interaction) in delay_buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.delay = opt.0;
        }
    }
}

fn tooltip_demo_apply_system(
    options: Res<TooltipDemoOptions>,
    mut triggers: Query<&mut TooltipTrigger, With<TooltipDemoButton>>,
    mut tooltips: Query<&mut Tooltip>,
) {
    if !options.is_changed() {
        return;
    }

    for mut trigger in triggers.iter_mut() {
        trigger.position = options.position;
        trigger.delay = options.delay;

        if let Some(tooltip_entity) = trigger.tooltip_entity {
            if let Ok(mut tooltip) = tooltips.get_mut(tooltip_entity) {
                tooltip.position = options.position;
            }
        }
    }
}

fn tooltip_demo_style_system(
    theme: Res<MaterialTheme>,
    options: Res<TooltipDemoOptions>,
    mut position_buttons: Query<
        (
            Entity,
            &TooltipPositionOption,
            &mut MaterialButton,
            &Children,
        ),
        Without<TooltipDelayOption>,
    >,
    mut delay_buttons: Query<
        (Entity, &TooltipDelayOption, &mut MaterialButton, &Children),
        Without<TooltipPositionOption>,
    >,
    mut label_colors: Query<&mut TextColor, With<ButtonLabel>>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (_entity, opt, mut button, children) in position_buttons.iter_mut() {
        let selected = opt.0 == options.position;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }

    for (_entity, opt, mut button, children) in delay_buttons.iter_mut() {
        let selected = (opt.0 - options.delay).abs() < 0.01;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }
}
