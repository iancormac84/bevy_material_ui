//! Elevation view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

const CARD_WIDTH: f32 = 140.0;
const CARD_HEIGHT: f32 = 96.0;

/// Spawn the elevation section content
pub fn spawn_elevation_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.elevation.title",
                "Elevation",
                "showcase.section.elevation.description",
                "Material elevation levels with shadow depth",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(24.0),
                    row_gap: Val::Px(24.0),
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                })
                .with_children(|grid| {
                    let levels = [
                        ("Level 0", Elevation::Level0),
                        ("Level 1", Elevation::Level1),
                        ("Level 2", Elevation::Level2),
                        ("Level 3", Elevation::Level3),
                        ("Level 4", Elevation::Level4),
                        ("Level 5", Elevation::Level5),
                    ];

                    for (label, elevation) in levels {
                        spawn_elevation_card(grid, theme, label, elevation);
                    }
                });

            spawn_code_block(section, theme, include_str!("../../elevation_demo.rs"));
        });
}

fn spawn_elevation_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    elevation: Elevation,
) {
    let dp_label = format!("{} ({}dp)", label, elevation.dp());

    parent
        .spawn((
            Node {
                width: Val::Px(CARD_WIDTH),
                height: Val::Px(CARD_HEIGHT),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(theme.surface_container),
            elevation.to_box_shadow(),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new(dp_label),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
        });
}
