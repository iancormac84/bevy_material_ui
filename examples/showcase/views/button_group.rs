//! Button Group view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the button group section content
pub fn spawn_button_group_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.button_group.title",
                "Button Groups",
                "showcase.section.button_group.description",
                "Segmented buttons for single or multi-selection toggle groups",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    row_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // Horizontal single selection
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new("Horizontal (single)"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        col.spawn((
                            MaterialButtonGroup::new()
                                .single_selection(true)
                                .selection_required(true)
                                .horizontal(),
                            Node::default(),
                        ))
                        .with_children(|group| {
                            spawn_toggle_button(group, theme, "Day", true);
                            spawn_toggle_button(group, theme, "Week", false);
                            spawn_toggle_button(group, theme, "Month", false);
                        });
                    });

                    // Vertical single selection
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new("Vertical (single)"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        col.spawn((
                            MaterialButtonGroup::new()
                                .single_selection(true)
                                .selection_required(true)
                                .vertical(),
                            Node::default(),
                        ))
                        .with_children(|group| {
                            spawn_toggle_button(group, theme, "Low", false);
                            spawn_toggle_button(group, theme, "Med", true);
                            spawn_toggle_button(group, theme, "High", false);
                        });
                    });

                    // Horizontal multi selection
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn((
                            Text::new("Multi-select"),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));

                        col.spawn((
                            MaterialButtonGroup::new()
                                .single_selection(false)
                                .horizontal(),
                            Node::default(),
                        ))
                        .with_children(|group| {
                            spawn_toggle_button(group, theme, "A", true);
                            spawn_toggle_button(group, theme, "B", true);
                            spawn_toggle_button(group, theme, "C", false);
                        });
                    });
                });

            spawn_code_block(section, theme, include_str!("../../button_group_demo.rs"));
        });
}

fn spawn_toggle_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    selected: bool,
) {
    let button = MaterialButton::new(label)
        .with_variant(ButtonVariant::Outlined)
        .checkable(true)
        .checked(selected);

    let text_color = button.text_color(theme);
    let bg_color = button.background_color(theme);
    let border_color = button.border_color(theme);

    parent
        .spawn((
            button,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                min_width: Val::Px(64.0),
                height: Val::Px(40.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}
