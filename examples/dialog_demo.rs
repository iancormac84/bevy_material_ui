//! Dialog Demo
//!
//! Demonstrates Material Design 3 dialogs.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::dialog::create_dialog_scrim_for;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct DialogsSectionRoot;

#[derive(Component)]
struct ShowDialogButton;

#[derive(Component)]
struct DialogCloseButton;

#[derive(Resource)]
struct DialogEntities {
    dialog: Entity,
    root: Entity,
}

#[derive(Component)]
struct DialogConfirmButton;

#[derive(Component)]
struct DialogResultDisplay;

#[derive(Component)]
struct DialogPositionOption(pub DemoDialogPosition);

#[derive(Component)]
struct DialogModalOption(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoDialogPosition {
    CenterWindow,
    CenterParent,
    BelowTrigger,
    AboveTrigger,
    RightOfTrigger,
    LeftOfTrigger,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
struct DialogDemoOptions {
    position: DemoDialogPosition,
    modal: bool,
}

impl Default for DialogDemoOptions {
    fn default() -> Self {
        Self {
            position: DemoDialogPosition::CenterWindow,
            modal: true,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .init_resource::<DialogDemoOptions>()
        .add_systems(
            Update,
            (
                dialog_demo_position_options_system,
                dialog_demo_modal_options_system,
                dialog_demo_position_style_system,
                dialog_demo_modal_style_system,
                dialog_demo_apply_options_system,
                dialog_demo_open_close_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    let root_id = commands
        .spawn((
            DialogsSectionRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("dialog_demo/root", &telemetry)
        .id();

    let mut show_button_entity: Option<Entity> = None;
    let mut dialog_entity: Option<Entity> = None;

    commands.entity(root_id).with_children(|section| {
        // Position options
        section
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("Dialog Position:"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                })
                .with_children(|row| {
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Center Window",
                        DemoDialogPosition::CenterWindow,
                        true,
                    );
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Center Parent",
                        DemoDialogPosition::CenterParent,
                        false,
                    );
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Below Trigger",
                        DemoDialogPosition::BelowTrigger,
                        false,
                    );
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Above Trigger",
                        DemoDialogPosition::AboveTrigger,
                        false,
                    );
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Right of Trigger",
                        DemoDialogPosition::RightOfTrigger,
                        false,
                    );
                    spawn_dialog_position_option(
                        row,
                        theme.as_ref(),
                        "Left of Trigger",
                        DemoDialogPosition::LeftOfTrigger,
                        false,
                    );
                });
            });

        // Modal options
        section
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("Dialog Modality:"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                })
                .with_children(|row| {
                    spawn_dialog_modal_option(
                        row,
                        theme.as_ref(),
                        "Modal (blocks clicks)",
                        true,
                        true,
                    );
                    spawn_dialog_modal_option(row, theme.as_ref(), "Click-through", false, false);
                });
            });

        // Show button + result
        section
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|row| {
                let show_label = "Show Dialog";
                let show_button =
                    MaterialButton::new(show_label).with_variant(ButtonVariant::Filled);
                let show_text_color = show_button.text_color(theme.as_ref());

                let mut show_btn = row.spawn((
                    ShowDialogButton,
                    Interaction::None,
                    MaterialButtonBuilder::new(show_label)
                        .filled()
                        .build(theme.as_ref()),
                ));
                show_btn.insert_test_id("dialog_demo/show_button", &telemetry);
                show_btn.with_children(|btn| {
                    btn.spawn((
                        ButtonLabel,
                        Text::new(show_label),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(show_text_color),
                    ));
                });

                show_button_entity = Some(show_btn.id());

                row.spawn((
                    DialogResultDisplay,
                    Text::new("Result: None"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));
            });

        // Dialog (hidden by default by MaterialDialog.open=false)
        let dialog = section
            .spawn((DialogBuilder::new()
                .title("Confirm Action")
                .modal(true)
                .build(theme.as_ref()),))
            .insert_test_id("dialog_demo/dialog", &telemetry)
            .with_children(|dialog| {
                dialog.spawn((
                    Text::new("Are you sure you want to proceed? This action cannot be undone."),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                    Node {
                        margin: UiRect::bottom(Val::Px(16.0)),
                        ..default()
                    },
                ));

                dialog
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|actions| {
                        let cancel_label = "Cancel";
                        actions
                            .spawn((
                                DialogCloseButton,
                                Interaction::None,
                                MaterialButtonBuilder::new(cancel_label)
                                    .text()
                                    .build(theme.as_ref()),
                            ))
                            .insert_test_id("dialog_demo/dialog/cancel", &telemetry)
                            .with_children(|btn| {
                                btn.spawn((
                                    ButtonLabel,
                                    Text::new(cancel_label),
                                    TextFont {
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(theme.primary),
                                ));
                            });

                        let confirm_label = "Confirm";
                        let confirm_button =
                            MaterialButton::new(confirm_label).with_variant(ButtonVariant::Filled);
                        let confirm_text_color = confirm_button.text_color(theme.as_ref());

                        actions
                            .spawn((
                                DialogConfirmButton,
                                Interaction::None,
                                MaterialButtonBuilder::new(confirm_label)
                                    .filled()
                                    .build(theme.as_ref()),
                            ))
                            .insert_test_id("dialog_demo/dialog/confirm", &telemetry)
                            .with_children(|btn| {
                                btn.spawn((
                                    ButtonLabel,
                                    Text::new(confirm_label),
                                    TextFont {
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(confirm_text_color),
                                ));
                            });
                    });
            })
            .id();

        dialog_entity = Some(dialog);
    });

    let dialog_entity = dialog_entity.expect("Dialog should exist");

    // Scrim follows dialog open state and modality.
    commands
        .spawn(create_dialog_scrim_for(&theme, dialog_entity, true))
        .insert_test_id("dialog_demo/dialog/scrim", &telemetry);

    // Apply initial placement once.
    commands.insert_resource(DialogEntities {
        dialog: dialog_entity,
        root: root_id,
    });
}

fn spawn_dialog_position_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    position: DemoDialogPosition,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            DialogPositionOption(position),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
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

fn spawn_dialog_modal_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    modal: bool,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            DialogModalOption(modal),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
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

fn dialog_demo_position_options_system(
    mut options: ResMut<DialogDemoOptions>,
    mut position_buttons: Query<(&DialogPositionOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }
}

fn dialog_demo_modal_options_system(
    mut options: ResMut<DialogDemoOptions>,
    mut modal_buttons: Query<(&DialogModalOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in modal_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.modal = opt.0;
        }
    }
}

fn dialog_demo_position_style_system(
    theme: Res<MaterialTheme>,
    options: Res<DialogDemoOptions>,
    mut position_chips: Query<(&DialogPositionOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in position_chips.iter_mut() {
        chip.selected = opt.0 == options.position;
    }
}

fn dialog_demo_modal_style_system(
    theme: Res<MaterialTheme>,
    options: Res<DialogDemoOptions>,
    mut modal_chips: Query<(&DialogModalOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in modal_chips.iter_mut() {
        chip.selected = opt.0 == options.modal;
    }
}

fn dialog_demo_apply_options_system(
    mut commands: Commands,
    options: Res<DialogDemoOptions>,
    entities: Res<DialogEntities>,
    show_buttons: Query<Entity, With<ShowDialogButton>>,
    mut dialogs: Query<&mut MaterialDialog>,
) {
    if !options.is_changed() {
        return;
    }

    let show_button = show_buttons.iter().next();

    let (anchor, placement) = match options.position {
        DemoDialogPosition::CenterWindow => (None, MaterialDialogPlacement::center_in_viewport()),
        DemoDialogPosition::CenterParent => {
            (Some(entities.root), MaterialDialogPlacement::CenterInAnchor)
        }
        DemoDialogPosition::BelowTrigger => {
            (show_button, MaterialDialogPlacement::below_anchor(12.0))
        }
        DemoDialogPosition::AboveTrigger => {
            (show_button, MaterialDialogPlacement::above_anchor(12.0))
        }
        DemoDialogPosition::RightOfTrigger => {
            (show_button, MaterialDialogPlacement::right_of_anchor(12.0))
        }
        DemoDialogPosition::LeftOfTrigger => {
            (show_button, MaterialDialogPlacement::left_of_anchor(12.0))
        }
    };

    if let Some(anchor) = anchor {
        commands
            .entity(entities.dialog)
            .insert(MaterialDialogAnchor(anchor));
    } else {
        commands
            .entity(entities.dialog)
            .remove::<MaterialDialogAnchor>();
    }
    commands.entity(entities.dialog).insert(placement);

    if let Ok(mut dialog) = dialogs.get_mut(entities.dialog) {
        dialog.modal = options.modal;
    }
}

fn dialog_demo_open_close_system(
    mut show_buttons: Query<&Interaction, (Changed<Interaction>, With<ShowDialogButton>)>,
    mut close_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogCloseButton>)>,
    mut confirm_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogConfirmButton>)>,
    entities: Res<DialogEntities>,
    mut dialogs: Query<&mut MaterialDialog>,
    mut result_text: Query<&mut Text, With<DialogResultDisplay>>,
) {
    let Ok(mut dialog) = dialogs.get_mut(entities.dialog) else {
        return;
    };

    let mut open = false;
    let mut close_reason: Option<&'static str> = None;

    for interaction in show_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            open = true;
        }
    }

    for interaction in close_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some("Cancelled");
        }
    }

    for interaction in confirm_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some("Confirmed");
        }
    }

    if open {
        dialog.open = true;
    }

    if let Some(reason) = close_reason {
        dialog.open = false;
        for mut text in result_text.iter_mut() {
            text.0 = format!("Result: {reason}");
        }
    }
}
