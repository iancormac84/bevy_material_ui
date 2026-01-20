//! TextField Demo
//!
//! Demonstrates Material Design 3 text fields (Filled / Outlined / Error / Email).

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::{spawn_text_field_control, InputType};

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

    let mut fields: Vec<(&'static str, Entity)> = Vec::new();

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
        .insert_test_id("textfield_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(900.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            })
            .with_children(|row| {
                let filled = spawn_text_field_control(
                    row,
                    &theme,
                    TextFieldBuilder::new()
                        .label("Filled")
                        .placeholder("Type here…")
                        .supporting_text("Click to focus and type")
                        .filled()
                        .width(Val::Px(240.0)),
                );
                fields.push(("filled", filled));

                let outlined = spawn_text_field_control(
                    row,
                    &theme,
                    TextFieldBuilder::new()
                        .label("Outlined")
                        .placeholder("Type here…")
                        .supporting_text("Enter submits")
                        .outlined()
                        .width(Val::Px(240.0)),
                );
                fields.push(("outlined", outlined));

                let error = spawn_text_field_control(
                    row,
                    &theme,
                    TextFieldBuilder::new()
                        .label("With Error")
                        .placeholder("Invalid input")
                        .error_text("This field has an error")
                        .filled()
                        .width(Val::Px(240.0)),
                );
                fields.push(("error", error));

                let email = spawn_text_field_control(
                    row,
                    &theme,
                    TextFieldBuilder::new()
                        .label("Email")
                        .placeholder("name@example.com")
                        .supporting_text("Must look like name@example.com")
                        .label_key("showcase.text_fields.email.label")
                        .placeholder_key("showcase.text_fields.email.placeholder")
                        .supporting_text_key("showcase.text_fields.email.supporting")
                        .input_type(InputType::Email)
                        .outlined()
                        .width(Val::Px(240.0)),
                );
                fields.push(("email", email));
            });
        });

    for (kind, entity) in fields {
        commands
            .entity(entity)
            .insert_test_id(format!("textfield_demo/field/{kind}"), &telemetry);
    }
}
