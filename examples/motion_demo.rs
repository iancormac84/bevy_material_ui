//! Motion Demo
//!
//! Demonstrates animated values and spring motion.

use bevy::prelude::*;
use bevy_material_ui::motion::{AnimatedValue, SpringAnimation, SpringConfig};
use bevy_material_ui::prelude::*;

const TRACK_WIDTH: f32 = 360.0;
const TRACK_HEIGHT: f32 = 80.0;
const BOX_SIZE: f32 = 48.0;
const TRACK_PADDING: f32 = 16.0;

#[derive(Component)]
struct MotionRange {
    start: f32,
    end: f32,
}

#[derive(Component)]
struct AnimatedBox;

#[derive(Component)]
struct SpringBox;

#[derive(Resource, Default)]
struct MotionDemoState {
    toggled: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .init_resource::<MotionDemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (motion_input_system, animate_motion_boxes_system))
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
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
        .insert_test_id("motion_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Press Space to toggle motion"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            spawn_motion_track(
                root,
                &theme,
                "Animated value (standard easing)",
                AnimatedValue::new(0.0)
                    .with_duration(Duration::MEDIUM4)
                    .with_easing(Easing::Standard),
                AnimatedBox,
                &telemetry,
                "motion_demo/animated",
            );

            spawn_motion_track(
                root,
                &theme,
                "Spring animation (smooth)",
                SpringAnimation::new(0.0, 0.0, SpringConfig::smooth()),
                SpringBox,
                &telemetry,
                "motion_demo/spring",
            );
        });
}

fn spawn_motion_track<T: Component>(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    animation: T,
    marker: impl Component,
    telemetry: &TelemetryConfig,
    test_id: &str,
) {
    let start = TRACK_PADDING;
    let end = TRACK_WIDTH - BOX_SIZE - TRACK_PADDING;

    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            ))
        .with_children(|column| {
            column.spawn((
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            column
                .spawn((
                    Node {
                        width: Val::Px(TRACK_WIDTH),
                        height: Val::Px(TRACK_HEIGHT),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                    BorderRadius::all(Val::Px(12.0)),
                ))
                .insert_test_id(test_id, telemetry)
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Px(BOX_SIZE),
                            height: Val::Px(BOX_SIZE),
                            position_type: PositionType::Absolute,
                            left: Val::Px(start),
                            top: Val::Px((TRACK_HEIGHT - BOX_SIZE) / 2.0),
                            ..default()
                        },
                        BackgroundColor(theme.primary),
                        BorderRadius::all(Val::Px(12.0)),
                        animation,
                        marker,
                        MotionRange { start, end },
                    ));
                });
        });
}

fn motion_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<MotionDemoState>,
    mut animated: Query<&mut AnimatedValue>,
    mut springs: Query<&mut SpringAnimation>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    state.toggled = !state.toggled;
    let target = if state.toggled { 1.0 } else { 0.0 };

    for mut value in animated.iter_mut() {
        value.set_target(target);
    }

    for mut spring in springs.iter_mut() {
        spring.set_target(target);
    }
}

fn animate_motion_boxes_system(
    time: Res<Time>,
    mut animated_boxes: Query<(&mut AnimatedValue, &MotionRange, &mut Node), With<AnimatedBox>>,
    mut spring_boxes: Query<(&mut SpringAnimation, &MotionRange, &mut Node), With<SpringBox>>,
) {
    let dt = time.delta_secs();

    for (mut value, range, mut node) in animated_boxes.iter_mut() {
        value.update(dt);
        let progress = value.value();
        node.left = Val::Px(range.start + (range.end - range.start) * progress);
    }

    for (mut spring, range, mut node) in spring_boxes.iter_mut() {
        spring.update(dt);
        let progress = spring.progress();
        node.left = Val::Px(range.start + (range.end - range.start) * progress);
    }
}
