//! Typography Demo
//!
//! Demonstrates the Material typography scale.

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

    let typography = Typography::default();

    let styles = [
        ("Display Large", typography.display_large),
        ("Display Medium", typography.display_medium),
        ("Display Small", typography.display_small),
        ("Headline Large", typography.headline_large),
        ("Headline Medium", typography.headline_medium),
        ("Headline Small", typography.headline_small),
        ("Title Large", typography.title_large),
        ("Title Medium", typography.title_medium),
        ("Title Small", typography.title_small),
        ("Label Large", typography.label_large),
        ("Label Medium", typography.label_medium),
        ("Label Small", typography.label_small),
        ("Body Large", typography.body_large),
        ("Body Medium", typography.body_medium),
        ("Body Small", typography.body_small),
    ];

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("typography_demo/root", &telemetry)
        .with_children(|root| {
            for (label, size) in styles {
                root.spawn((
                    Text::new(label),
                    TextFont {
                        font_size: FontSize::Px(size),
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));
            }
        });
}
