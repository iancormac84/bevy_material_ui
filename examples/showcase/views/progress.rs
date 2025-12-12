//! Progress indicators view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the progress section content
pub fn spawn_progress_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Progress Indicators",
                "Visual feedback for loading and progress states"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Linear progress at 0%
                    spawn_linear_progress(col, theme, 0.0, "0%");
                    // Linear progress at 50%
                    spawn_linear_progress(col, theme, 0.5, "50%");
                    // Linear progress at 100%
                    spawn_linear_progress(col, theme, 1.0, "100%");
                });

            spawn_code_block(section, theme,
r#"// Linear progress (determinate)
let progress = LinearProgress::new(0.5); // 50%

// Indeterminate progress
let progress = LinearProgress::indeterminate();

// Circular progress
let progress = CircularProgress::new(0.75);"#);
        });
}

fn spawn_linear_progress(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, value: f32, label: &str) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(12.0),
        ..default()
    }).with_children(|row| {
        // Track
        row.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(theme.surface_container_highest),
            BorderRadius::all(Val::Px(2.0)),
        )).with_children(|track| {
            // Indicator
            track.spawn((
                Node {
                    width: Val::Percent(value * 100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.primary),
                BorderRadius::all(Val::Px(2.0)),
            ));
        });
        
        // Label
        row.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(theme.on_surface_variant),
        ));
    });
}
