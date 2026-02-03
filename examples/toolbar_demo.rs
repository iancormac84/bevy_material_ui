//! Toolbar Demo
//!
//! Demonstrates the Material toolbar with navigation and actions.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_MORE_VERT, ICON_SEARCH};
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
        .insert_test_id("toolbar_demo/root", &telemetry)
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
                        Node {
                            width: Val::Percent(100.0),
                            max_width: Val::Px(560.0),
                            height: Val::Px(TOOLBAR_HEIGHT),
                            padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(Spacing::MEDIUM),
                            ..default()
                        },
                        BackgroundColor(theme.surface),
                    ))
                    .with_children(|toolbar| {
                        fn spawn_standard_icon_button(
                            parent: &mut ChildSpawnerCommands,
                            theme: &MaterialTheme,
                            icon_name: &str,
                        ) {
                            let icon_btn = MaterialIconButton::new(icon_name.to_string())
                                .with_variant(IconButtonVariant::Standard);
                            let icon_color = icon_btn.icon_color(theme);

                            parent
                                .spawn((IconButtonBuilder::new(icon_name.to_string())
                                    .standard()
                                    .build(theme),))
                                .with_children(|btn| {
                                    if let Some(icon) =
                                        bevy_material_ui::icons::MaterialIcon::from_name(icon_name)
                                    {
                                        btn.spawn(
                                            icon.with_size(TOOLBAR_ICON_SIZE)
                                                .with_color(icon_color),
                                        );
                                    }
                                });
                        }

                        spawn_standard_icon_button(toolbar, &theme, ICON_MENU);

                        toolbar.spawn((
                            Text::new("Inventory"),
                            TextFont {
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                            TextColor(theme.on_surface),
                            Node {
                                flex_grow: 1.0,
                                ..default()
                            },
                        ));

                        spawn_standard_icon_button(toolbar, &theme, ICON_SEARCH);
                        spawn_standard_icon_button(toolbar, &theme, ICON_MORE_VERT);
                    });
            });
        });
}
