//! Text fields view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::TextFieldVariant;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};

use crate::showcase::common::*;

/// Spawn the text fields section content
pub fn spawn_text_fields_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Text Fields",
                "Text input with Filled and Outlined variants - Configure options below"
            );

            // Options panel
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(12.0),
                ..default()
            }).with_children(|options| {
                // Blink speed options
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Cursor Blink:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));
                    
                    // Blink speed buttons
                    for (label, speed) in [
                        ("0.25s", 0.25_f32),
                        ("0.5s", 0.53_f32),
                        ("1.0s", 1.0_f32),
                    ] {
                        let is_default = (speed - 0.53).abs() < 0.01;
                        let chip_for_color = MaterialChip::filter(label).with_selected(is_default);
                        let label_color = chip_for_color.label_color(theme);

                        row.spawn((
                            TextFieldBlinkSpeedOption(speed),
                            Interaction::None,
                            ChipBuilder::filter(label).selected(is_default).build(theme),
                        ))
                        .with_children(|chip| {
                            chip.spawn((
                                ChipLabel,
                                Text::new(label),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(label_color),
                            ));
                        });
                    }
                });
                
                // Cursor toggle
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Show Cursor:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));

                    let toggle_label = "Cursor";
                    let chip_for_color = MaterialChip::filter(toggle_label).with_selected(true);
                    let label_color = chip_for_color.label_color(theme);

                    row.spawn((
                        TextFieldCursorToggle,
                        Interaction::None,
                        ChipBuilder::filter(toggle_label).selected(true).build(theme),
                    ))
                    .with_children(|chip| {
                        chip.spawn((
                            ChipLabel,
                            Text::new("ON"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(label_color),
                        ));
                    });
                });
            });

            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                spawn_text_field_demo(row, theme, TextFieldVariant::Filled, "Filled", false);
                spawn_text_field_demo(row, theme, TextFieldVariant::Outlined, "Outlined", false);
                spawn_text_field_demo(row, theme, TextFieldVariant::Filled, "With Error", true);
            });

            spawn_code_block(section, theme,
r#"// Create a text field
let text_field = MaterialTextField::new()
    .with_variant(TextFieldVariant::Outlined)
    .label("Email")
    .placeholder("Enter your email")
    .supporting_text("We'll never share your email");

commands.spawn((
    text_field,
    Node { width: Val::Px(280.0), ..default() },
));"#);
        });
}

fn spawn_text_field_demo(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    variant: TextFieldVariant,
    label: &str,
    has_error: bool,
) {
    let is_outlined = variant == TextFieldVariant::Outlined;
    let border_color = if has_error { 
        theme.error 
    } else if is_outlined { 
        theme.outline 
    } else { 
        theme.on_surface_variant
    };
    let bg_color = if is_outlined { 
        Color::NONE 
    } else { 
        theme.surface_container_highest 
    };
    
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(4.0),
        ..default()
    }).with_children(|col| {
        // Label
        col.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(if has_error { theme.error } else { theme.primary }),
        ));
        
        // Input container - interactive demo element
        let placeholder_text = if has_error { "Invalid input" } else { "Click to focus..." };
        col.spawn((
            TextFieldDemoInput,
            Button,
            Interaction::None,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(56.0),
                padding: UiRect::horizontal(Val::Px(16.0)),
                border: UiRect::all(Val::Px(if is_outlined { 1.0 } else { 0.0 })),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(4.0)),
        )).with_children(|input| {
            input.spawn((
                TextFieldDemoText { base: placeholder_text.to_string() },
                Text::new(placeholder_text),
                TextFont { font_size: 16.0, ..default() },
                TextColor(if has_error { theme.error } else { theme.on_surface_variant }),
            ));
        });
        
        // Bottom border for filled variant
        if !is_outlined {
            col.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(2.0),
                    margin: UiRect::top(Val::Px(-2.0)),
                    ..default()
                },
                BackgroundColor(if has_error { theme.error } else { theme.primary }),
            ));
        }
        
        // Supporting text
        if has_error {
            col.spawn((
                Text::new("This field has an error"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(theme.error),
            ));
        } else {
            col.spawn((
                Text::new("Click to focus the field"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(theme.on_surface_variant),
            ));
        }
    });
}
