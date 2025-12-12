//! Chips view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::ICON_CHECK;

use crate::showcase::common::*;

/// Spawn the chips section content
pub fn spawn_chips_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Chips",
                "Compact elements for filters, selections, and actions"
            );

            let font = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_chip(row, theme, "Filter", false, font.clone());
                    spawn_chip(row, theme, "Selected", true, font.clone());
                    spawn_chip(row, theme, "Tag", false, font.clone());
                    spawn_chip(row, theme, "Action", false, font.clone());
                });

            spawn_code_block(section, theme,
r#"// Create an assist chip
let chip = MaterialChip::assist("Label");

// Create a filter chip (toggleable)
let chip = MaterialChip::filter("Category")
    .selected(true);

// Create an input chip (with close button)
let chip = MaterialChip::input("User Input");"#);
        });
}

fn spawn_chip(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, selected: bool, icon_font: Handle<Font>) {
    let bg_color = if selected { theme.secondary_container } else { theme.surface_container };
    let border_color = if selected { theme.secondary_container } else { theme.outline };
    let text_color = if selected { theme.on_secondary_container } else { theme.on_surface_variant };
    
    parent.spawn((
        Button,
        Interaction::None,
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(border_color),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|chip| {
        // Show check icon if selected
        if selected {
            chip.spawn((
                Text::new(ICON_CHECK.to_string()),
                TextFont { font: icon_font.clone(), font_size: 18.0, ..default() },
                TextColor(text_color),
            ));
        }
        chip.spawn((
            Text::new(label),
            TextFont { font_size: 14.0, ..default() },
            TextColor(text_color),
        ));
    });
}
