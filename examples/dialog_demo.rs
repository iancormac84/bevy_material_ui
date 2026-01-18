//! Dialog Demo
//!
//! Demonstrates Material Design 3 dialogs.

use bevy::prelude::*;
use bevy_material_ui::dialog::create_dialog_scrim_for;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct OpenDialogButton;

#[derive(Component)]
struct CancelDialogButton;

#[derive(Component)]
struct ConfirmDialogButton;

#[derive(Resource)]
struct DialogEntities {
    dialog: Entity,
    trigger: Entity,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
enum DemoPlacement {
    Below,
    Above,
    Right,
    Left,
    CenterOnWindow,
}

impl Default for DemoPlacement {
    fn default() -> Self {
        Self::Below
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .init_resource::<DemoPlacement>()
        .add_systems(
            Update,
            (
                open_dialog_system,
                close_dialog_system,
                placement_hotkeys_system,
                apply_demo_placement_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    let mut trigger_entity: Option<Entity> = None;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("dialog_demo/root", &telemetry)
        .with_children(|root| {
            let open_label = "Open Dialog";
            let open_button = MaterialButton::new(open_label).with_variant(ButtonVariant::Filled);
            let open_text_color = open_button.text_color(&theme);

            let mut open_button_entity = root.spawn((
                OpenDialogButton,
                Interaction::None,
                MaterialButtonBuilder::new(open_label)
                    .filled()
                    .build(&theme),
            ));

            open_button_entity.insert_test_id("dialog_demo/open_button", &telemetry);
            open_button_entity.with_children(|btn| {
                btn.spawn((
                    ButtonLabel,
                    Text::new(open_label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(open_text_color),
                ));
            });

            trigger_entity = Some(open_button_entity.id());

            root.spawn((
                Text::new("Placement hotkeys: 1=Below, 2=Above, 3=Right, 4=Left, 5=Center"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });

    let trigger_entity = trigger_entity.expect("Open dialog button should exist");

    let dialog_entity = commands
        .spawn((
            DialogBuilder::new()
                .title("Confirm Action")
                .modal(true)
                .build(&theme),
            // Place the dialog relative to the trigger button.
            MaterialDialogAnchor(trigger_entity),
            MaterialDialogPlacement::below_anchor(12.0),
        ))
        .insert_test_id("dialog_demo/dialog", &telemetry)
        .with_children(|dialog| {
            dialog.spawn((
                Text::new("Are you sure you want to proceed?"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
                Node {
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));

            dialog
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    DialogActions,
                ))
                .with_children(|actions| {
                    let cancel_label = "Cancel";
                    actions
                        .spawn((
                            CancelDialogButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(cancel_label)
                                .text()
                                .build(&theme),
                        ))
                        .insert_test_id("dialog_demo/dialog/cancel", &telemetry)
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(cancel_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.primary),
                            ));
                        });

                    let confirm_label = "Confirm";
                    let confirm_button =
                        MaterialButton::new(confirm_label).with_variant(ButtonVariant::Filled);
                    let confirm_text_color = confirm_button.text_color(&theme);

                    actions
                        .spawn((
                            ConfirmDialogButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(confirm_label)
                                .filled()
                                .build(&theme),
                        ))
                        .insert_test_id("dialog_demo/dialog/confirm", &telemetry)
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(confirm_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(confirm_text_color),
                            ));
                        });
                });
        })
            .id();

    // Scrim follows dialog open state and modality.
    commands
        .spawn(create_dialog_scrim_for(&theme, dialog_entity, true))
        .insert_test_id("dialog_demo/dialog/scrim", &telemetry);

    commands.insert_resource(DialogEntities {
        dialog: dialog_entity,
        trigger: trigger_entity,
    });
}

fn open_dialog_system(
    entities: Res<DialogEntities>,
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<OpenDialogButton>)>,
    mut dialogs: Query<&mut MaterialDialog>,
) {
    let Ok(mut dialog) = dialogs.get_mut(entities.dialog) else {
        return;
    };

    for interaction in interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            dialog.open = true;
        }
    }
}

fn close_dialog_system(
    entities: Res<DialogEntities>,
    mut dialogs: Query<&mut MaterialDialog>,
    mut cancel: Query<&Interaction, (Changed<Interaction>, With<CancelDialogButton>)>,
    mut confirm: Query<&Interaction, (Changed<Interaction>, With<ConfirmDialogButton>)>,
) {
    let Ok(mut dialog) = dialogs.get_mut(entities.dialog) else {
        return;
    };

    let should_close = cancel.iter_mut().any(|i| *i == Interaction::Pressed)
        || confirm.iter_mut().any(|i| *i == Interaction::Pressed);

    if should_close {
        dialog.open = false;
    }
}

fn placement_hotkeys_system(keys: Res<ButtonInput<KeyCode>>, mut placement: ResMut<DemoPlacement>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *placement = DemoPlacement::Below;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *placement = DemoPlacement::Above;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *placement = DemoPlacement::Right;
    } else if keys.just_pressed(KeyCode::Digit4) {
        *placement = DemoPlacement::Left;
    } else if keys.just_pressed(KeyCode::Digit5) {
        *placement = DemoPlacement::CenterOnWindow;
    }
}

fn apply_demo_placement_system(
    placement: Res<DemoPlacement>,
    entities: Res<DialogEntities>,
    mut commands: Commands,
) {
    if !placement.is_changed() {
        return;
    }

    let (anchor, placement_component) = match *placement {
        DemoPlacement::Below => (
            Some(entities.trigger),
            MaterialDialogPlacement::below_anchor(12.0),
        ),
        DemoPlacement::Above => (
            Some(entities.trigger),
            MaterialDialogPlacement::above_anchor(12.0),
        ),
        DemoPlacement::Right => (
            Some(entities.trigger),
            MaterialDialogPlacement::right_of_anchor(12.0),
        ),
        DemoPlacement::Left => (
            Some(entities.trigger),
            MaterialDialogPlacement::left_of_anchor(12.0),
        ),
        DemoPlacement::CenterOnWindow => (None, MaterialDialogPlacement::center_in_viewport()),
    };

    if let Some(anchor) = anchor {
        commands.entity(entities.dialog).insert(MaterialDialogAnchor(anchor));
    }
    commands.entity(entities.dialog).insert(placement_component);
}
