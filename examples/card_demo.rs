//! Card Demo
//!
//! Demonstrates Material Design 3 cards: elevated, filled, and outlined variants.

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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("card_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .with_children(|row| {
                let cards = [
                    ("elevated", "Elevated", CardBuilder::new().elevated()),
                    ("filled", "Filled", CardBuilder::new().filled()),
                    ("outlined", "Outlined", CardBuilder::new().outlined()),
                ];

                for (id, title, builder) in cards {
                    row.spawn((
                        Interaction::None,
                        builder.width(Val::Px(160.0)).padding(16.0).build(&theme),
                    ))
                    .insert_test_id(format!("card_demo/card/{id}"), &telemetry)
                    .with_children(|card| {
                        card.spawn((
                            Text::new(title),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));
                        card.spawn((
                            Text::new("Card content goes here with supporting text."),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));
                    });
                }
            });
        });
}
