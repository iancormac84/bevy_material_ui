//! Radio Demo
//!
//! Demonstrates Material Design 3 radio buttons and group exclusivity.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Resource, Default)]
struct RadioDemoRows(Vec<(Entity, String)>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .insert_resource(RadioDemoRows::default())
        .add_systems(Startup, setup)
        .add_systems(Update, attach_radio_child_test_ids)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    mut rows: ResMut<RadioDemoRows>,
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
        .insert_test_id("radio_demo/root", &telemetry)
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

    // Match the showcase: one radio group with three choices.
    let group_entity = commands
        .spawn((
            RadioGroup::new("example_group"),
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            ChildOf(section_entity),
        ))
        .insert_test_id("radio_demo/group", &telemetry)
        .id();

    for (key, selected, label) in [
        ("choice_a", true, "Choice A"),
        ("choice_b", false, "Choice B"),
        ("choice_c", false, "Choice C"),
    ] {
        let row = commands.spawn_radio(&theme, selected, "example_group", label);

        if telemetry.enabled {
            commands
                .entity(row)
                .insert(TestId::new(format!("radio_demo/row/{key}")));
        }

        commands.entity(group_entity).add_child(row);
        rows.0.push((row, format!("radio_demo/radio/{key}")));
    }
}

fn attach_radio_child_test_ids(
    mut commands: Commands,
    telemetry: Res<TelemetryConfig>,
    rows: Res<RadioDemoRows>,
    children_query: Query<&Children>,
    is_radio: Query<(), With<MaterialRadio>>,
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
            if is_radio.get(child).is_ok() {
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
