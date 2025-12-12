//! FAB (Floating Action Button) view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::ICON_ADD;

use crate::showcase::common::*;

/// Spawn the FAB section content
pub fn spawn_fab_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Floating Action Buttons",
                "Primary actions with prominent visual treatment"
            );

            let font = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Small FAB
                    spawn_fab(row, theme, 40.0, font.clone());
                    // Regular FAB
                    spawn_fab(row, theme, 56.0, font.clone());
                    // Large FAB
                    spawn_fab(row, theme, 96.0, font.clone());
                });

            spawn_code_block(section, theme,
r#"// Create a FAB
let fab = MaterialFab::new()
    .icon(ICON_ADD)
    .size(FabSize::Regular);

// Extended FAB with label
let fab = MaterialFab::new()
    .icon(ICON_ADD)
    .label("Create")
    .extended(true);"#);
        });
}

fn spawn_fab(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, size: f32, icon_font: Handle<Font>) {
    parent.spawn((
        FabButton,
        Button,
        Interaction::None,
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.primary_container),
        BorderRadius::all(Val::Px(size / 3.5)),
        Elevation::Level3.to_box_shadow(),
    )).with_children(|fab| {
        // Use proper icon character with icon font
        fab.spawn((
            Text::new(ICON_ADD.to_string()),
            TextFont { font: icon_font, font_size: size * 0.45, ..default() },
            TextColor(theme.on_primary_container),
        ));
    });
}
