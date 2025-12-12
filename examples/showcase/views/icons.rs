//! Icons view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the icons section content
pub fn spawn_icons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Material Icons",
                "Google Material Symbols with variable font support"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Show several icons using Unicode codepoints
                    let icons = [
                        ("\u{e5ca}", "check"),      // check
                        ("\u{e88a}", "home"),       // home
                        ("\u{e8b8}", "settings"),   // settings
                        ("\u{e87d}", "favorite"),   // favorite
                        ("\u{e8b6}", "search"),     // search
                    ];
                    
                    for (icon_char, _name) in icons {
                        row.spawn((
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme.surface_container),
                            BorderRadius::all(Val::Px(8.0)),
                        )).with_children(|container| {
                            container.spawn((
                                Text::new(icon_char),
                                TextFont { 
                                    font: icon_font.clone(),
                                    font_size: 24.0, 
                                    ..default() 
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                    }
                });

            spawn_code_block(section, theme,
r#"// Using Material Symbols icons
use bevy_material_ui::icons::{ICON_CHECK, icon_by_name};

// By constant
commands.spawn((
    Text::new(ICON_CHECK),
    TextFont { font: icon_font.0.clone(), font_size: 24.0, ..default() },
));

// By name lookup
if let Some(codepoint) = icon_by_name("home") {
    // Use codepoint...
}"#);
        });
}
