//! List Demo
//!
//! Demonstrates Material Design 3 lists with various item configurations.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    language: Option<Res<MaterialLanguage>>,
    i18n: Option<Res<MaterialI18n>>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("list_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(420.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
            ))
            .insert_test_id("list_demo/panel", &telemetry)
            .with_children(|panel| {
                let mut items: Vec<ListItemBuilder> = Vec::new();
                let language_tag = language.as_ref().map(|l| l.tag.as_str()).unwrap_or("en-US");

                // Use a larger list to highlight virtualization benefits.
                // Note: dividers are not supported in the virtualized path yet.
                for i in 1..=500 {
                    let headline_key = format!("list_demo.item_{}.headline", i);
                    let supporting_key = format!("list_demo.item_{}.supporting", i);

                    let headline = i18n
                        .as_ref()
                        .and_then(|i18n| i18n.translate(language_tag, &headline_key))
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("Item {i}"));

                    let builder = if i % 3 == 0 {
                        let supporting = i18n
                            .as_ref()
                            .and_then(|i18n| i18n.translate(language_tag, &supporting_key))
                            .map(str::to_string)
                            .unwrap_or_else(|| "Supporting text".to_string());

                        ListItemBuilder::new(headline)
                            .two_line()
                            .supporting_text(supporting)
                    } else {
                        ListItemBuilder::new(headline).one_line()
                    };

                    items.push(builder);
                }

                panel
                    .spawn((
                        ListBuilder::new()
                            .max_height(360.0)
                            .selection_mode(ListSelectionMode::Single)
                            .items_from_builders(items)
                            .virtualize(true)
                            .overscan_rows(3)
                            .build_scrollable(),
                        BackgroundColor(theme.surface),
                    ))
                    .insert_test_id("list_demo/list", &telemetry);
            });
        });
}
