use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{ScaffoldEntities, ScaffoldTestIds};

/// Configuration for a standard drawer scaffold (drawer + content).
///
/// Standard drawers persist alongside content, but can be shown/hidden.
#[derive(Debug, Clone)]
pub struct StandardDrawerScaffold {
    pub drawer_width_px: f32,
    pub drawer_open: bool,
    pub root_padding_px: f32,
    pub root_gap_px: f32,
    pub drawer_padding_px: f32,
    pub content_padding_px: f32,
    pub test_ids: ScaffoldTestIds,
}

impl Default for StandardDrawerScaffold {
    fn default() -> Self {
        Self {
            drawer_width_px: 280.0,
            drawer_open: true,
            root_padding_px: 0.0,
            root_gap_px: 0.0,
            drawer_padding_px: 12.0,
            content_padding_px: 16.0,
            test_ids: ScaffoldTestIds::default(),
        }
    }
}

/// Spawn a standard drawer scaffold.
///
/// The drawer is part of the layout flow (unlike modal drawers). Toggle the
/// drawer by rebuilding the scaffold or by setting its Node display/width.
pub fn spawn_standard_drawer_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &StandardDrawerScaffold,
    drawer_children: impl FnOnce(&mut ChildSpawnerCommands),
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    let mut navigation = Entity::PLACEHOLDER;
    let mut content = Entity::PLACEHOLDER;

    let drawer_display = if config.drawer_open {
        Display::Flex
    } else {
        Display::None
    };

    let root = parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(config.root_padding_px)),
                column_gap: Val::Px(config.root_gap_px),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
            config.test_ids.root.clone(),
        ))
        .with_children(|root| {
            navigation = root
                .spawn((
                    Node {
                        display: drawer_display,
                        width: Val::Px(config.drawer_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(config.drawer_padding_px)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    config.test_ids.navigation.clone(),
                ))
                .with_children(drawer_children)
                .id();

            content = root
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(config.content_padding_px)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    config.test_ids.content.clone(),
                ))
                .with_children(content_children)
                .id();
        })
        .id();

    ScaffoldEntities {
        root,
        navigation,
        content,
    }
}
