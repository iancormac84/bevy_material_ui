//! App Bar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_MORE_VERT, ICON_SEARCH, ICON_CHECK, ICON_CLOSE, ICON_ADD};

use crate::showcase::common::*;

/// Spawn the app bar section content
pub fn spawn_app_bar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section, 
                theme, 
                "App Bars",
                "Top and Bottom app bars for navigation and actions"
            );

            let icon_font_clone = icon_font.clone();
            // Top App Bar preview
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Top App Bar (Small)"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                let icon_font_top = icon_font_clone.clone();
                // Top app bar
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(64.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                )).with_children(|bar| {
                    // Menu icon
                    bar.spawn((
                        AppBarIconButton,
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(ICON_MENU.to_string()),
                            TextFont { font: icon_font_top.clone(), font_size: 24.0, ..default() },
                            TextColor(theme.on_surface),
                        ));
                    });
                    
                    // Title
                    bar.spawn((
                        Text::new("Page Title"),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(theme.on_surface),
                        Node { flex_grow: 1.0, ..default() },
                    ));
                    
                    // Actions
                    bar.spawn((
                        AppBarIconButton,
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(ICON_MORE_VERT.to_string()),
                            TextFont { font: icon_font_top.clone(), font_size: 24.0, ..default() },
                            TextColor(theme.on_surface),
                        ));
                    });
                });
                
                col.spawn((
                    Text::new("Bottom App Bar"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                    Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ));
                
                let icon_font_bottom = icon_font_clone.clone();
                // Bottom app bar
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                )).with_children(|bar| {
                    // Left actions
                    bar.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..default()
                    }).with_children(|actions| {
                        let icons = [(ICON_MENU, "menu"), (ICON_SEARCH, "search"), (ICON_CHECK, "check"), (ICON_CLOSE, "close")];
                        for (icon, _name) in icons {
                            let icon_f = icon_font_bottom.clone();
                            actions.spawn((
                                AppBarIconButton,
                                Button,
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(20.0)),
                            )).with_children(|btn| {
                                btn.spawn((
                                    Text::new(icon.to_string()),
                                    TextFont { font: icon_f, font_size: 20.0, ..default() },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                        }
                    });
                    
                    // FAB with proper icon
                    let icon_f_fab = icon_font_bottom.clone();
                    bar.spawn((
                        AppBarIconButton,
                        Button,
                        Node {
                            width: Val::Px(56.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme.primary_container),
                        BorderRadius::all(Val::Px(16.0)),
                    )).with_children(|fab| {
                        fab.spawn((
                            Text::new(ICON_ADD.to_string()),
                            TextFont { font: icon_f_fab, font_size: 28.0, ..default() },
                            TextColor(theme.on_primary_container),
                        ));
                    });
                });
            });

            spawn_code_block(section, theme,
r#"// Create a top app bar
let app_bar = TopAppBar::new()
    .with_variant(TopAppBarVariant::Small)
    .title("My App")
    .navigation_icon("menu");

commands.spawn((
    app_bar,
    Node { 
        width: Val::Percent(100.0), 
        height: Val::Px(64.0),
        ..default() 
    },
    BackgroundColor(theme.surface),
));

// Create a bottom app bar
let bottom_bar = BottomAppBar::new()
    .actions(vec!["search", "share", "delete"]);"#);
        });
}
