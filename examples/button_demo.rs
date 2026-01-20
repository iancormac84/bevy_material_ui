//! Button Demo
//!
//! Demonstrates Material Design 3 buttons: elevated, filled, filled tonal,
//! outlined, and text buttons.

use bevy::prelude::*;
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
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("button_demo/root", &telemetry)
        .with_children(|root| {
            // Match the showcase layout: 5 button variants + 2 segmented button groups.
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                width: Val::Percent(100.0),
                max_width: Val::Px(560.0),
                ..default()
            })
            .with_children(|col| {
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_interactive_button(
                        row,
                        &theme,
                        &telemetry,
                        "button_demo/variant/filled",
                        "Filled",
                        ButtonVariant::Filled,
                    );
                    spawn_interactive_button(
                        row,
                        &theme,
                        &telemetry,
                        "button_demo/variant/outlined",
                        "Outlined",
                        ButtonVariant::Outlined,
                    );
                    spawn_interactive_button(
                        row,
                        &theme,
                        &telemetry,
                        "button_demo/variant/text",
                        "Text",
                        ButtonVariant::Text,
                    );
                    spawn_interactive_button(
                        row,
                        &theme,
                        &telemetry,
                        "button_demo/variant/elevated",
                        "Elevated",
                        ButtonVariant::Elevated,
                    );
                    spawn_interactive_button(
                        row,
                        &theme,
                        &telemetry,
                        "button_demo/variant/tonal",
                        "Tonal",
                        ButtonVariant::FilledTonal,
                    );
                });

                col.spawn((
                    Text::new("Button Groups"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                    Node {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                ));

                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexStart,
                    column_gap: Val::Px(24.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Horizontal segmented (single selection)
                    row.spawn((
                        MaterialButtonGroup::new()
                            .single_selection(true)
                            .selection_required(true)
                            .horizontal(),
                        Node { ..default() },
                    ))
                    .insert_test_id("button_demo/group/period", &telemetry)
                    .with_children(|group| {
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/period/day",
                            "Day",
                            true,
                        );
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/period/week",
                            "Week",
                            false,
                        );
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/period/month",
                            "Month",
                            false,
                        );
                    });

                    // Vertical segmented (single selection)
                    row.spawn((
                        MaterialButtonGroup::new()
                            .single_selection(true)
                            .selection_required(true)
                            .vertical(),
                        Node { ..default() },
                    ))
                    .insert_test_id("button_demo/group/priority", &telemetry)
                    .with_children(|group| {
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/priority/low",
                            "Low",
                            false,
                        );
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/priority/med",
                            "Med",
                            true,
                        );
                        spawn_toggle_button(
                            group,
                            &theme,
                            &telemetry,
                            "button_demo/group/priority/high",
                            "High",
                            false,
                        );
                    });
                });
            });
        });
}

fn spawn_toggle_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    telemetry: &TelemetryConfig,
    test_id: &str,
    label: &str,
    checked: bool,
) {
    let button = MaterialButton::new(label)
        .with_variant(ButtonVariant::Outlined)
        .checkable(true)
        .checked(checked);

    let text_color = button.text_color(theme);
    let bg_color = button.background_color(theme);
    let border_color = button.border_color(theme);

    parent
        .spawn((
            button,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
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

fn spawn_interactive_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    telemetry: &TelemetryConfig,
    test_id: &str,
    label: &str,
    variant: ButtonVariant,
) {
    let button = MaterialButton::new(label).with_variant(variant);
    let text_color = button.text_color(theme);
    let bg_color = button.background_color(theme);
    let border_color = button.border_color(theme);
    let has_border = variant == ButtonVariant::Outlined;
    let elevation = button.elevation();

    parent
        .spawn((
            button,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
            elevation.to_box_shadow(),
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
