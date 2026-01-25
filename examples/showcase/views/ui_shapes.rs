//! UI Shapes view for the showcase application.
//!
//! Note: The full UI shapes demo requires mesh/material assets.
//! Run `cargo run --example ui_shapes_demo` for the full interactive demo.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the UI shapes section content
pub fn spawn_ui_shapes_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.ui_shapes.title",
                "UI Shapes",
                "showcase.section.ui_shapes.description",
                "2D shape primitives rendered as UI elements",
            );

            section.spawn((
                Text::new("Run `cargo run --example ui_shapes_demo` for the full interactive demo with shape rendering."),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(16.0),
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_shape_placeholder(row, theme, "Rounded Rect");
                    spawn_shape_placeholder(row, theme, "Star");
                    spawn_shape_placeholder(row, theme, "Hexagon");
                    spawn_shape_placeholder(row, theme, "Ellipse");
                    spawn_shape_placeholder(row, theme, "Triangle");
                });

            spawn_code_block(section, theme, include_str!("../../ui_shapes_demo.rs"));
        });
}

fn spawn_shape_placeholder(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(120.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(theme.surface_container),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(theme.primary),
            ));

            card.spawn((
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
