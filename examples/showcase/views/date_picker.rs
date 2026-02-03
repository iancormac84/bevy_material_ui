//! Date Picker view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the date picker section content
pub fn spawn_date_picker_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.date_picker.title",
                "Date Picker",
                "showcase.section.date_picker.description",
                "Material Design 3 calendar-based date selection",
            );

            // Picker overlay (hidden until opened)
            let picker_entity = section.spawn_date_picker(
                theme,
                DatePickerBuilder::new()
                    .title_key("showcase.date_picker.title")
                    .single_date(Date::new(2025, 1, 15))
                    .width(Val::Px(360.0)),
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    let btn = MaterialButton::new("").with_variant(ButtonVariant::Filled);
                    let text_color = btn.text_color(theme);

                    row.spawn((
                        DatePickerOpenButton(picker_entity),
                        Interaction::None,
                        MaterialButtonBuilder::new("").filled().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(""),
                            LocalizedText::new("showcase.date_picker.open_button")
                                .with_default("Open Date Picker"),
                            TextFont {
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                            TextColor(text_color),
                        ));
                    });

                    row.spawn((
                        DatePickerResultDisplay(picker_entity),
                        Text::new(""),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                });

            spawn_code_block(section, theme, include_str!("../../date_picker_demo.rs"));
        });
}
