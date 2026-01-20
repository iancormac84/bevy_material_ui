//! Checkbox Demo
//!
//! Demonstrates Material Design 3 checkboxes.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Resource, Default)]
struct CheckboxDemoRows(Vec<(Entity, String)>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .insert_resource(CheckboxDemoRows::default())
        .add_systems(Startup, setup)
        .add_systems(Update, attach_checkbox_child_test_ids)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    mut rows: ResMut<CheckboxDemoRows>,
) {
    commands.spawn(Camera2d);

    let root_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("checkbox_demo/root", &telemetry)
        .id();

    let section_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(560.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            ChildOf(root_entity),
        ))
        .id();

    // Spawn checkboxes as separate rows so we can attach stable IDs.
    // Match the showcase: three options, with option 1 checked.
    let checkbox_defs = [
        ("option_1", CheckboxState::Checked, "Option 1"),
        ("option_2", CheckboxState::Unchecked, "Option 2"),
        ("option_3", CheckboxState::Unchecked, "Option 3"),
    ];

    // A simple column for the checkboxes.
    let column = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            ChildOf(section_entity),
        ))
        .insert_test_id("checkbox_demo/column", &telemetry)
        .id();

    for (key, state, label) in checkbox_defs {
        let row = commands.spawn_checkbox(&theme, state, label);

        if telemetry.enabled {
            commands
                .entity(row)
                .insert(TestId::new(format!("checkbox_demo/row/{key}")));
        }

        commands.entity(column).add_child(row);
        rows.0.push((row, format!("checkbox_demo/checkbox/{key}")));
    }
}

fn attach_checkbox_child_test_ids(
    mut commands: Commands,
    telemetry: Res<TelemetryConfig>,
    rows: Res<CheckboxDemoRows>,
    children_query: Query<&Children>,
    is_checkbox: Query<(), With<MaterialCheckbox>>,
    is_text: Query<(), With<Text>>,
) {
    if !telemetry.enabled {
        return;
    }

    for (row_entity, base) in rows.0.iter() {
        let Ok(children) = children_query.get(*row_entity) else {
            continue;
        };

        for child in children.iter() {
            if is_checkbox.get(child).is_ok() {
                commands
                    .entity(child)
                    .insert_test_id(base.to_string(), &telemetry);
            } else if is_text.get(child).is_ok() {
                commands
                    .entity(child)
                    .insert_test_id(format!("{base}/label"), &telemetry);
            }
        }
    }
}
