//! App Bar Demo
//!
//! Demonstrates Material Design 3 top and bottom app bars.

use bevy::prelude::*;
use bevy_material_ui::app_bar::spawn_top_app_bar_with_right_content;
use bevy_material_ui::icons::{ICON_ADD, ICON_CHECK, ICON_CLOSE, ICON_MENU, ICON_SEARCH};
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::{spawn_text_field_control, InputType};

fn spawn_standard_icon_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_name: &str,
) {
    let icon_btn =
        MaterialIconButton::new(icon_name.to_string()).with_variant(IconButtonVariant::Standard);
    let icon_color = icon_btn.icon_color(theme);

    parent
        .spawn(
            IconButtonBuilder::new(icon_name.to_string())
                .standard()
                .build(theme),
        )
        .with_children(|btn| {
            if let Some(icon) = bevy_material_ui::icons::MaterialIcon::from_name(icon_name) {
                btn.spawn(icon.with_size(24.0).with_color(icon_color));
            }
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

    // Root container
    let root_id = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("app_bar_demo/root", &telemetry)
        .id();

    // Header label
    commands.entity(root_id).with_children(|root| {
        root.spawn((
            Text::new("Top App Bar (Small)"),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(theme.on_surface),
            Node {
                margin: UiRect::top(Val::Px(16.0)),
                ..default()
            },
        ));
    });

    // Top app bar with right-side search field slot.
    let top = spawn_top_app_bar_with_right_content(
        &mut commands,
        &theme,
        TopAppBarBuilder::new("Page Title")
            .small()
            .with_navigation("menu")
            .add_action("more_vert", "more"),
        |right| {
            right
                .spawn(Node {
                    width: Val::Px(240.0),
                    ..default()
                })
                .with_children(|slot| {
                    spawn_text_field_control(
                        slot,
                        &theme,
                        TextFieldBuilder::new()
                            .label("Search")
                            .placeholder("Search")
                            .input_type(InputType::Text)
                            .outlined()
                            .width(Val::Percent(100.0)),
                    );
                });
        },
    );

    commands
        .entity(top)
        .insert_test_id("app_bar_demo/top", &telemetry);
    commands.entity(root_id).add_child(top);

    // Bottom section
    commands.entity(root_id).with_children(|root| {
        root.spawn((
            Text::new("Bottom App Bar"),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(theme.on_surface),
            Node {
                margin: UiRect::top(Val::Px(32.0)),
                ..default()
            },
        ));

        // Bottom app bar preview
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                padding: UiRect::horizontal(Val::Px(16.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(theme.surface_container),
        ))
        .insert_test_id("app_bar_demo/bottom", &telemetry)
        .with_children(|bar| {
            // Left actions
            bar.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|actions| {
                for icon_name in [ICON_MENU, ICON_SEARCH, ICON_CHECK, ICON_CLOSE] {
                    spawn_standard_icon_button(actions, &theme, icon_name);
                }
            });

            // FAB preview
            {
                let fab_btn = MaterialFab::new(ICON_ADD.to_string()).with_size(FabSize::Regular);
                let bg_color = fab_btn.background_color(&theme);
                let icon_color = fab_btn.content_color(&theme);

                bar.spawn((
                    fab_btn,
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        width: Val::Px(56.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    BorderRadius::all(Val::Px(16.0)),
                ))
                .with_children(|btn| {
                    if let Some(icon) = bevy_material_ui::icons::MaterialIcon::from_name(ICON_ADD) {
                        btn.spawn(icon.with_size(24.0).with_color(icon_color));
                    }
                });
            }
        });
    });
}
