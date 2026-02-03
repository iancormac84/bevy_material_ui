//! Time Picker Demo
//!
//! Demonstrates the Material Design 3 time picker component.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct OpenPickerButton(Entity);

#[derive(Component)]
struct ResultText(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, time_picker_demo_system)
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
        .insert_test_id("time_picker_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(560.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|section| {
                // Picker overlay (hidden until opened)
                let picker_entity = section.spawn_time_picker(
                    &theme,
                    TimePickerBuilder::new()
                        .title("Select Time")
                        .initial_time(13, 30)
                        .format(TimeFormat::H24)
                        .width(Val::Px(360.0)),
                );

                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|row| {
                        let label = "Open Time Picker";
                        let btn = MaterialButton::new(label).with_variant(ButtonVariant::Filled);
                        let label_color = btn.text_color(&theme);

                        row.spawn((
                            OpenPickerButton(picker_entity),
                            Interaction::None,
                            MaterialButtonBuilder::new(label).filled().build(&theme),
                        ))
                        .insert_test_id("time_picker_demo/open", &telemetry)
                        .with_children(|b| {
                            b.spawn((
                                ButtonLabel,
                                Text::new(label),
                                TextFont {
                                    font_size: FontSize::Px(14.0),
                                    ..default()
                                },
                                TextColor(label_color),
                            ));
                        });

                        row.spawn((
                            ResultText(picker_entity),
                            Text::new("Result: 13:30"),
                            TextFont {
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ))
                        .insert_test_id("time_picker_demo/result", &telemetry);
                    });
            });
        });
}

#[allow(clippy::type_complexity)]
fn time_picker_demo_system(
    mut open_buttons: Query<(&Interaction, &OpenPickerButton), Changed<Interaction>>,
    mut pickers: ParamSet<(Query<&mut MaterialTimePicker>, Query<&MaterialTimePicker>)>,
    mut result_texts: Query<(&ResultText, &mut Text)>,
) {
    let prefix = "Result:";
    let canceled = "Canceled";

    // Open picker when button is pressed.
    for (interaction, open_button) in open_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.p0().get_mut(open_button.0) {
            picker.open = true;
        }
    }

    // Render the current time for each result display.
    for (display, mut text) in result_texts.iter_mut() {
        let label = if let Ok(picker) = pickers.p1().get(display.0) {
            format!("{prefix} {:02}:{:02}", picker.hour, picker.minute)
        } else {
            format!("{prefix} {canceled}")
        };

        text.0 = label;
    }
}
