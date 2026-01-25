//! Scroll view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the scroll section content
pub fn spawn_scroll_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.scroll.title",
                "Scroll",
                "showcase.section.scroll.description",
                "Material scroll containers with customizable scrollbars",
            );

            // Vertical scroller
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Vertical scrollbar"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    col.spawn((
                        ScrollContainerBuilder::new().vertical().build(),
                        ScrollPosition::default(),
                        Node {
                            width: Val::Px(360.0),
                            height: Val::Px(140.0),
                            overflow: Overflow::scroll(),
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                    ))
                    .with_children(|content| {
                        for i in 1..=20 {
                            content.spawn((
                                Text::new(format!("Item {i}")),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        }
                    });
                });

            // Horizontal scroller
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Horizontal scrollbar"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    col.spawn((
                        ScrollContainerBuilder::new().horizontal().build(),
                        ScrollPosition::default(),
                        Node {
                            width: Val::Px(360.0),
                            height: Val::Px(100.0),
                            overflow: Overflow::scroll(),
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(12.0),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                    ))
                    .with_children(|content| {
                        for i in 1..=20 {
                            content
                                .spawn((
                                    Node {
                                        width: Val::Px(80.0),
                                        height: Val::Px(60.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        flex_shrink: 0.0,
                                        border_radius: BorderRadius::all(Val::Px(8.0)),
                                        ..default()
                                    },
                                    BackgroundColor(theme.surface_container_high),
                                ))
                                .with_children(|card| {
                                    card.spawn((
                                        Text::new(format!("{i}")),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(theme.on_surface),
                                    ));
                                });
                        }
                    });
                });

            spawn_code_block(section, theme, include_str!("../../scroll_demo.rs"));
        });
}
