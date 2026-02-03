use bevy::prelude::*;
use bevy_material_ui::{
    loading_indicator::{LoadingIndicatorBuilder, ShapeMorphMaterial, SpawnLoadingIndicatorChild},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialUiPlugin, TelemetryPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(420.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Default
                    col.spawn((
                        Text::new("Loading indicator"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                    col.spawn_loading_indicator(&theme, &mut materials);

                    // Contained
                    col.spawn((
                        Text::new("Loading indicator with container"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                    col.spawn_loading_indicator_with(
                        &theme,
                        &mut materials,
                        LoadingIndicatorBuilder::new().contained(),
                    );

                    // Multi-color
                    col.spawn((
                        Text::new("Loading indicator with multiple colors"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                    col.spawn_loading_indicator_with(
                        &theme,
                        &mut materials,
                        LoadingIndicatorBuilder::new().multi_color(),
                    );

                    // Small
                    col.spawn((
                        Text::new("Small loading indicator"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                    col.spawn_loading_indicator_with(
                        &theme,
                        &mut materials,
                        LoadingIndicatorBuilder::new().size(36.0),
                    );

                    // Large + fast
                    col.spawn((
                        Text::new("Large loading indicator (fast)"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                    col.spawn_loading_indicator_with(
                        &theme,
                        &mut materials,
                        LoadingIndicatorBuilder::new().size(64.0).speed(2.0),
                    );
                });
        });
}
