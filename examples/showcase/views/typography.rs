//! Typography view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the typography section content
pub fn spawn_typography_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.typography.title",
                "Typography",
                "showcase.section.typography.description",
                "Material typography scale with display, headline, title, label, and body styles",
            );

            let typography = Typography::default();

            let styles = [
                ("Display Large", typography.display_large),
                ("Display Medium", typography.display_medium),
                ("Display Small", typography.display_small),
                ("Headline Large", typography.headline_large),
                ("Headline Medium", typography.headline_medium),
                ("Headline Small", typography.headline_small),
                ("Title Large", typography.title_large),
                ("Title Medium", typography.title_medium),
                ("Title Small", typography.title_small),
                ("Label Large", typography.label_large),
                ("Label Medium", typography.label_medium),
                ("Label Small", typography.label_small),
                ("Body Large", typography.body_large),
                ("Body Medium", typography.body_medium),
                ("Body Small", typography.body_small),
            ];

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|col| {
                    for (label, size) in styles {
                        col.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: size,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));
                    }
                });

            spawn_code_block(section, theme, include_str!("../../typography_demo.rs"));
        });
}
