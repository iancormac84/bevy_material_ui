//! Motion view for the showcase application.
//!
//! Note: The full motion demo with interactive animations requires
//! additional runtime state. Run `cargo run --example motion_demo`
//! for the full interactive experience.

use bevy::prelude::*;
use bevy_material_ui::motion::{AnimatedValue, SpringAnimation, SpringConfig};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

const TRACK_WIDTH: f32 = 320.0;
const TRACK_HEIGHT: f32 = 60.0;
const BOX_SIZE: f32 = 40.0;
const TRACK_PADDING: f32 = 12.0;

/// Range for motion animation
#[derive(Component)]
pub struct MotionDemoRange {
    pub start: f32,
    pub end: f32,
}

/// Marker for animated value box
#[derive(Component)]
pub struct MotionDemoAnimatedBox;

/// Marker for spring animation box
#[derive(Component)]
pub struct MotionDemoSpringBox;

/// State for motion demo toggle
#[derive(Resource, Default)]
pub struct MotionDemoState {
    pub toggled: bool,
}

/// Marker for the motion demo toggle button
#[derive(Component)]
pub struct MotionDemoToggleButton;

/// Spawn the motion section content
pub fn spawn_motion_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.motion.title",
                "Motion",
                "showcase.section.motion.description",
                "Animated values with easing and spring physics",
            );

            section.spawn((
                Text::new("Click the toggle button to animate"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            // Toggle button with marker
            section
                .spawn((
                    MotionDemoToggleButton,
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(theme.primary),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Toggle Animation"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(theme.on_primary),
                    ));
                });

            let start = TRACK_PADDING;
            let end = TRACK_WIDTH - BOX_SIZE - TRACK_PADDING;

            // Animated value track
            spawn_motion_track(
                section,
                theme,
                "Animated value (standard easing)",
                AnimatedValue::new(0.0)
                    .with_duration(Duration::MEDIUM4)
                    .with_easing(Easing::Standard),
                MotionDemoAnimatedBox,
                start,
                end,
            );

            // Spring animation track
            spawn_spring_track(
                section,
                theme,
                "Spring animation (smooth)",
                SpringAnimation::new(0.0, 0.0, SpringConfig::smooth()),
                MotionDemoSpringBox,
                start,
                end,
            );

            spawn_code_block(section, theme, include_str!("../../motion_demo.rs"));
        });
}

fn spawn_motion_track(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    animation: AnimatedValue,
    marker: impl Component,
    start: f32,
    end: f32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|column| {
            column.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(12.0),
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
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Px(BOX_SIZE),
                            height: Val::Px(BOX_SIZE),
                            position_type: PositionType::Absolute,
                            left: Val::Px(start),
                            top: Val::Px((TRACK_HEIGHT - BOX_SIZE) / 2.0),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(theme.primary),
                        animation,
                        marker,
                        MotionDemoRange { start, end },
                    ));
                });
        });
}

fn spawn_spring_track(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    animation: SpringAnimation,
    marker: impl Component,
    start: f32,
    end: f32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|column| {
            column.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(12.0),
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
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                ))
                .with_children(|track| {
                    track.spawn((
                        Node {
                            width: Val::Px(BOX_SIZE),
                            height: Val::Px(BOX_SIZE),
                            position_type: PositionType::Absolute,
                            left: Val::Px(start),
                            top: Val::Px((TRACK_HEIGHT - BOX_SIZE) / 2.0),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(theme.tertiary),
                        animation,
                        marker,
                        MotionDemoRange { start, end },
                    ));
                });
        });
}

/// System to toggle motion demo animations
pub fn motion_demo_toggle_system(
    mut state: ResMut<MotionDemoState>,
    mut animated: Query<&mut AnimatedValue, With<MotionDemoAnimatedBox>>,
    mut springs: Query<&mut SpringAnimation, With<MotionDemoSpringBox>>,
    buttons: Query<&Interaction, (Changed<Interaction>, With<MotionDemoToggleButton>)>,
) {
    for interaction in buttons.iter() {
        if *interaction != Interaction::Pressed {
            continue;
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
}

/// System to animate motion demo boxes
pub fn motion_demo_animate_system(
    time: Res<Time>,
    mut animated_boxes: Query<
        (&mut AnimatedValue, &MotionDemoRange, &mut Node),
        (With<MotionDemoAnimatedBox>, Without<MotionDemoSpringBox>),
    >,
    mut spring_boxes: Query<
        (&mut SpringAnimation, &MotionDemoRange, &mut Node),
        (With<MotionDemoSpringBox>, Without<MotionDemoAnimatedBox>),
    >,
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
