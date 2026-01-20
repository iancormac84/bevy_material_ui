//! Icon Button Demo
//!
//! Demonstrates standard/filled/tonal/outlined icon buttons.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn spawn_icon_button_demo(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_name: &str,
    variant: IconButtonVariant,
    label: &str,
    test_id: &str,
    telemetry: &TelemetryConfig,
) {
    let icon_btn = MaterialIconButton::new(icon_name).with_variant(variant);
    let bg_color = icon_btn.background_color(theme);
    let icon_color = icon_btn.icon_color(theme);
    let has_border = variant == IconButtonVariant::Outlined;

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                icon_btn,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(if has_border {
                    theme.outline
                } else {
                    Color::NONE
                }),
                BorderRadius::all(Val::Px(20.0)),
            ))
            .insert_test_id(test_id, telemetry)
            .with_children(|btn| {
                if let Some(icon) =
                    MaterialIcon::from_name(icon_name).or_else(|| MaterialIcon::from_name("star"))
                {
                    btn.spawn(icon.with_size(24.0).with_color(icon_color));
                }
            });

            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}

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
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(16.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("icon_button_demo/root", &telemetry)
        .with_children(|root| {
            spawn_icon_button_demo(
                root,
                &theme,
                "favorite",
                IconButtonVariant::Standard,
                "Standard",
                "icon_button_demo/button/standard",
                &telemetry,
            );
            spawn_icon_button_demo(
                root,
                &theme,
                "add",
                IconButtonVariant::Filled,
                "Filled",
                "icon_button_demo/button/filled",
                &telemetry,
            );
            spawn_icon_button_demo(
                root,
                &theme,
                "edit",
                IconButtonVariant::FilledTonal,
                "Tonal",
                "icon_button_demo/button/filled_tonal",
                &telemetry,
            );
            spawn_icon_button_demo(
                root,
                &theme,
                "delete",
                IconButtonVariant::Outlined,
                "Outlined",
                "icon_button_demo/button/outlined",
                &telemetry,
            );
        });
}
