//! UI Shapes Demo
//!
//! Demonstrates rendering UI-compatible 2D shapes.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::ui_shapes::{ShapePath, UiShapeBuilder, UiShapePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(UiShapePlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(24.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("ui_shapes_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("UI Shapes"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(24.0),
                row_gap: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|grid| {
                spawn_shape_card(
                    grid,
                    &theme,
                    &mut meshes,
                    &mut materials,
                    "Rounded Rect",
                    ShapePath::rounded_rect(Vec2::new(90.0, 52.0), 12.0, Vec2::ZERO, 8),
                    theme.primary,
                    "ui_shapes_demo/rounded_rect",
                    &telemetry,
                );

                spawn_shape_card(
                    grid,
                    &theme,
                    &mut meshes,
                    &mut materials,
                    "Star",
                    ShapePath::star(5, 34.0, 14.0, Vec2::ZERO),
                    theme.tertiary,
                    "ui_shapes_demo/star",
                    &telemetry,
                );

                spawn_shape_card(
                    grid,
                    &theme,
                    &mut meshes,
                    &mut materials,
                    "Polygon",
                    ShapePath::regular_polygon(6, 32.0, Vec2::ZERO),
                    theme.secondary,
                    "ui_shapes_demo/polygon",
                    &telemetry,
                );

                spawn_shape_card(
                    grid,
                    &theme,
                    &mut meshes,
                    &mut materials,
                    "Ellipse",
                    ShapePath::ellipse(Vec2::new(38.0, 24.0), Vec2::ZERO, 32),
                    theme.primary_container,
                    "ui_shapes_demo/ellipse",
                    &telemetry,
                );
            });
        });
}

fn spawn_shape_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    label: &str,
    shape: ShapePath,
    color: Color,
    test_id: &str,
    telemetry: &TelemetryConfig,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(180.0),
                height: Val::Px(160.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(theme.surface_container),
        ))
        .insert_test_id(test_id, telemetry)
        .with_children(|card| {
            card.spawn(UiShapeBuilder::new(shape)
                .with_color(color)
                .with_offset(Vec2::new(0.0, -18.0))
                .build(meshes, materials));

            card.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
