//! Switches view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning switches.
//! The switch component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_switch_i18n;

/// Spawn the switches section content
pub fn spawn_switches_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.switches.title",
                "Switches",
                "showcase.section.switches.description",
                "Toggle on/off with sliding thumb animation",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_switch_i18n(col, theme, true, "showcase.switches.wifi", "Wi-Fi");
                    spawn_switch_i18n(
                        col,
                        theme,
                        false,
                        "showcase.switches.bluetooth",
                        "Bluetooth",
                    );
                    spawn_switch_i18n(
                        col,
                        theme,
                        false,
                        "showcase.switches.dark_mode",
                        "Dark Mode",
                    );
                });

            spawn_code_block(section, theme, include_str!("../../switch_demo.rs"));
        });
}
