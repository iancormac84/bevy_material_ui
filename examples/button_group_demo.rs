//! Button Group Demo
//!
//! Demonstrates Material Design 3 segmented buttons / toggle groups.

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
        .insert_test_id("button_group_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(560.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|section| {
                section
                    .spawn((
                        Text::new("Button Groups"),
                        TextFont {
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ))
                    .insert_test_id("button_group_demo/title", &telemetry);

                section
                    .spawn(Node {
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
                        .insert_test_id("button_group_demo/group/horizontal", &telemetry)
                        .with_children(|group| {
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/horizontal/day",
                                "Day",
                                true,
                            );
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/horizontal/week",
                                "Week",
                                false,
                            );
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/horizontal/month",
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
                        .insert_test_id("button_group_demo/group/vertical", &telemetry)
                        .with_children(|group| {
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/vertical/low",
                                "Low",
                                false,
                            );
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/vertical/med",
                                "Med",
                                true,
                            );
                            spawn_toggle_button(
                                group,
                                &theme,
                                &telemetry,
                                "button_group_demo/group/vertical/high",
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
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}
