//! Ripple view for the showcase application.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Marker for ripple demo surfaces
#[derive(Component)]
pub struct RippleDemoSurface;

/// Spawn the ripple section content
pub fn spawn_ripple_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.ripple.title",
                "Ripple",
                "showcase.section.ripple.description",
                "Touch feedback ripple effect on interactive surfaces",
            );

            section.spawn((
                Text::new("Click a surface to spawn a ripple"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(24.0),
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_ripple_surface(
                        row,
                        theme,
                        "Primary",
                        theme.primary,
                        theme.on_primary,
                    );
                    spawn_ripple_surface(
                        row,
                        theme,
                        "Secondary",
                        theme.secondary,
                        theme.on_secondary,
                    );
                    spawn_ripple_surface(
                        row,
                        theme,
                        "Tertiary",
                        theme.tertiary,
                        theme.on_tertiary,
                    );
                    spawn_ripple_surface(
                        row,
                        theme,
                        "Surface",
                        theme.surface_container,
                        theme.on_surface,
                    );
                });

            spawn_code_block(section, theme, include_str!("../../ripple_demo.rs"));
        });
}

fn spawn_ripple_surface(
    parent: &mut ChildSpawnerCommands,
    _theme: &MaterialTheme,
    label: &str,
    surface_color: Color,
    text_color: Color,
) {
    parent
        .spawn((
            RippleDemoSurface,
            RippleHost::new().with_color(text_color),
            Interaction::None,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(surface_color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

/// System to handle ripple interactions in the showcase
pub fn ripple_demo_interaction_system(
    mut events: MessageWriter<SpawnRipple>,
    buttons: Query<
        (Entity, &Interaction, &ComputedNode),
        With<RippleDemoSurface>,
    >,
    surfaces: Query<
        (Entity, &ComputedNode, &UiGlobalTransform),
        With<RippleDemoSurface>,
    >,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let scale = windows
        .single()
        .map(|window| window.scale_factor())
        .unwrap_or(1.0);

    for (entity, interaction, node) in buttons.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let size = node.size() / scale;
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }
        let position = Vec2::new(size.x * 0.5, size.y * 0.5);
        events.write(SpawnRipple { host: entity, position });
    }

    let Ok(window) = windows.single() else {
        return;
    };

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_logical) = window.cursor_position() else {
        return;
    };

    let cursor_physical = cursor_logical * scale;

    for (entity, node, transform) in surfaces.iter() {
        let size = node.size();
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }
        let center = transform.translation.trunc();
        let top_left = center - size / 2.0;
        let bottom_right = top_left + size;

        if cursor_physical.x < top_left.x
            || cursor_physical.x > bottom_right.x
            || cursor_physical.y < top_left.y
            || cursor_physical.y > bottom_right.y
        {
            continue;
        }

        let position_physical = Vec2::new(
            cursor_physical.x - top_left.x,
            cursor_physical.y - top_left.y,
        );
        let position = position_physical / scale;

        events.write(SpawnRipple { host: entity, position });
        break;
    }
}
