//! Theme Demo
//!
//! Demonstrates setting a custom Material theme seed and light/dark mode.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::theme::ThemeMode;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Light,
    Dark,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Dark
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .init_resource::<Mode>()
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_theme_mode)
        .run();
}

fn setup(
    mut commands: Commands,
    mut theme: ResMut<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
) {
    // Pick a seed so the demo is visually distinctive.
    *theme = MaterialTheme::from_seed(Color::srgb_u8(0x67, 0x50, 0xA4), ThemeMode::Dark);

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
        .insert_test_id("theme_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Press Space to toggle light/dark"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            root.spawn(
                MaterialButtonBuilder::new("A Themed Button")
                    .filled()
                    .build(&theme),
            )
            .insert_test_id("theme_demo/button", &telemetry)
            .with_children(|b| {
                b.spawn((
                    ButtonLabel,
                    Text::new("A Themed Button"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(theme.on_primary),
                ));
            });
        });
}

fn toggle_theme_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<Mode>,
    mut theme: ResMut<MaterialTheme>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    *mode = match *mode {
        Mode::Light => Mode::Dark,
        Mode::Dark => Mode::Light,
    };

    let theme_mode = match *mode {
        Mode::Light => ThemeMode::Light,
        Mode::Dark => ThemeMode::Dark,
    };

    *theme = MaterialTheme::from_seed(Color::srgb_u8(0x67, 0x50, 0xA4), theme_mode);
}
