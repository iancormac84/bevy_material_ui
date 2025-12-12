//! Badges view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::ICON_NOTIFICATIONS;

use crate::showcase::common::*;

/// Spawn the badges section content
pub fn spawn_badges_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Badges",
                "Notification indicators for counts and status"
            );

            let icon_font_clone = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Dot badge
                    spawn_badge_example(row, theme, &icon_font_clone, None);
                    // Small count
                    spawn_badge_example(row, theme, &icon_font_clone, Some("3"));
                    // Large count
                    spawn_badge_example(row, theme, &icon_font_clone, Some("99+"));
                });

            spawn_code_block(section, theme,
r#"// Dot badge (no text)
let badge = MaterialBadge::dot();

// Count badge
let badge = MaterialBadge::count(5);

// Count badge with max
let badge = MaterialBadge::count(150).max(99); // Shows "99+""#);
        });
}

fn spawn_badge_example(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: &Handle<Font>, count: Option<&str>) {
    parent.spawn((
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
        // Notification icon with proper font
        container.spawn((
            Text::new(ICON_NOTIFICATIONS.to_string()),
            TextFont { font: icon_font.clone(), font_size: 24.0, ..default() },
            TextColor(theme.on_surface),
        ));
        
        // Badge
        let (width, text) = match count {
            None => (Val::Px(8.0), String::new()),
            Some(c) => (Val::Auto, c.to_string()),
        };
        
        container.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(4.0),
                right: Val::Px(4.0),
                width,
                min_width: Val::Px(if count.is_some() { 16.0 } else { 8.0 }),
                height: Val::Px(if count.is_some() { 16.0 } else { 8.0 }),
                padding: UiRect::axes(Val::Px(4.0), Val::Px(0.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.error),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|badge| {
            if !text.is_empty() {
                badge.spawn((
                    Text::new(text),
                    TextFont { font_size: 10.0, ..default() },
                    TextColor(theme.on_error),
                ));
            }
        });
    });
}
