//! Checkboxes view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning checkboxes.
//! The checkbox component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_checkbox_i18n;

/// Spawn the checkboxes section content
pub fn spawn_checkboxes_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Option<Handle<Font>>,
) {
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
                "showcase.section.checkboxes.title",
                "Checkboxes",
                "showcase.section.checkboxes.description",
                "Toggle selection with visual checkmark feedback",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Checked,
                        "showcase.checkboxes.option_1",
                        "Option 1",
                    );
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Unchecked,
                        "showcase.checkboxes.option_2",
                        "Option 2",
                    );
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Unchecked,
                        "showcase.checkboxes.option_3",
                        "Option 3",
                    );
                });

            spawn_code_block(
                section,
                theme,
                include_str!("../../checkbox_demo.rs"),
            );
        });
}
