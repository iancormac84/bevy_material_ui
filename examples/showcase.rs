//! Interactive Material Design 3 UI Components Showcase
//!
//! This example demonstrates interactive Material Design 3 UI components
//! with proper event handling and visual feedback, overlaid on a 3D scene
//! with a spinning D10 dice.
//!
//! Run with: `cargo run --example showcase_interactive`

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use bevy_material_ui::prelude::*;
use bevy_material_ui::checkbox::{CheckboxChangeEvent, CheckboxBox, CheckboxIcon};
use bevy_material_ui::switch::SwitchChangeEvent;
use bevy_material_ui::radio::RadioChangeEvent;
use bevy_material_ui::list::{ListBuilder, ListItemBuilder, ScrollableList};
use bevy_material_ui::icons::ICON_CHECK;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Material Design 3 Interactive Showcase".into(),
                resolution: bevy::window::WindowResolution::new(1000, 700),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaterialUiPlugin)
        .init_resource::<ScrollbarDragState>()
        .init_resource::<ListScrollDragState>()
        .add_systems(Startup, (setup_3d_scene, setup_ui))
        .add_systems(Update, (
            handle_button_clicks,
            handle_checkbox_changes,
            handle_switch_changes,
            handle_radio_changes,
            update_checkbox_visuals,
            update_switch_visuals,
            update_radio_visuals,
            mouse_scroll_system,
            clamp_scroll_positions, // Clamp after any scroll changes
            update_scrollbar_thumb,
            scrollbar_thumb_drag_system,
            update_list_scroll_thumb,
            list_scroll_thumb_drag_system,
            rotate_dice,
        ))
        .run();
}

/// Marker component for the spinning dice
#[derive(Component)]
struct SpinningDice;

/// Setup the 3D scene with a spinning D10 dice
fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            order: 0, // Render first (behind UI)
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.08)),
            ..default()
        },
    ));
    
    // Ambient light
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.0,
            affects_lightmapped_meshes: true,
        },
    ));
    
    // Point light
    commands.spawn((
        PointLight {
            color: Color::srgb(0.8, 0.6, 1.0),
            intensity: 800_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    
    // Secondary light for fill
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.8, 1.0),
            intensity: 400_000.0,
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(-4.0, -2.0, 6.0),
    ));
    
    // Create D10 dice mesh (pentagonal trapezohedron approximation)
    let d10_mesh = create_d10_mesh();
    
    // Spawn the spinning D10
    commands.spawn((
        SpinningDice,
        Mesh3d(meshes.add(d10_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.2, 0.8), // Purple dice
            metallic: 0.3,
            perceptual_roughness: 0.4,
            emissive: LinearRgba::new(0.1, 0.02, 0.15, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.5)),
    ));
}

/// Create a D10 mesh (pentagonal trapezohedron)
fn create_d10_mesh() -> Mesh {
    // D10 is a pentagonal trapezohedron - 10 kite-shaped faces
    // We'll create a simplified version using triangles
    
    let n = 5; // Pentagon base
    let top_radius = 0.9;
    let bottom_radius = 0.9;
    let top_height = 1.2;
    let bottom_height = -1.2;
    let mid_top = 0.3;
    let mid_bottom = -0.3;
    
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    
    // Generate vertices for the D10 shape
    // Top point
    let top_point = [0.0, top_height, 0.0];
    // Bottom point
    let bottom_point = [0.0, bottom_height, 0.0];
    
    // Upper ring of vertices
    let mut upper_ring: Vec<[f32; 3]> = Vec::new();
    for i in 0..n {
        let angle = (i as f32) * 2.0 * PI / (n as f32);
        upper_ring.push([
            top_radius * angle.cos(),
            mid_top,
            top_radius * angle.sin(),
        ]);
    }
    
    // Lower ring of vertices (offset by half step)
    let mut lower_ring: Vec<[f32; 3]> = Vec::new();
    for i in 0..n {
        let angle = ((i as f32) + 0.5) * 2.0 * PI / (n as f32);
        lower_ring.push([
            bottom_radius * angle.cos(),
            mid_bottom,
            bottom_radius * angle.sin(),
        ]);
    }
    
    // Create faces
    // Upper kites (top point + two adjacent upper ring vertices + one lower ring vertex)
    for i in 0..n {
        let next_i = (i + 1) % n;
        
        // Upper face (kite shape as 2 triangles)
        // Triangle 1: top -> upper[i] -> lower[i]
        add_triangle(&mut positions, &mut normals, &mut indices,
            top_point, upper_ring[i], lower_ring[i]);
        // Triangle 2: top -> lower[i] -> upper[next]
        add_triangle(&mut positions, &mut normals, &mut indices,
            top_point, lower_ring[i], upper_ring[next_i]);
        
        // Lower face (kite shape as 2 triangles)
        // Triangle 1: bottom -> lower[i] -> upper[next]
        add_triangle(&mut positions, &mut normals, &mut indices,
            bottom_point, lower_ring[i], upper_ring[i]);
        // Triangle 2: bottom -> upper[i] -> lower[prev]
        let prev_i = (i + n - 1) % n;
        add_triangle(&mut positions, &mut normals, &mut indices,
            bottom_point, upper_ring[i], lower_ring[prev_i]);
    }
    
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

fn add_triangle(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
) {
    let base_idx = positions.len() as u32;
    
    // Calculate face normal
    let a = Vec3::from(v1) - Vec3::from(v0);
    let b = Vec3::from(v2) - Vec3::from(v0);
    let normal = a.cross(b).normalize();
    let n = normal.to_array();
    
    positions.push(v0);
    positions.push(v1);
    positions.push(v2);
    
    normals.push(n);
    normals.push(n);
    normals.push(n);
    
    indices.push(base_idx);
    indices.push(base_idx + 1);
    indices.push(base_idx + 2);
}

/// System to rotate the dice
fn rotate_dice(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SpinningDice>>,
) {
    for mut transform in query.iter_mut() {
        // Rotate around Y axis (vertical spin)
        transform.rotate_y(time.delta_secs() * 0.5);
        // Add a slight wobble on X axis
        let wobble = (time.elapsed_secs() * 0.3).sin() * 0.1;
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.5) 
            * Quat::from_rotation_x(wobble);
    }
}

fn setup_ui(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    icon_font: Res<MaterialIconFont>,
) {
    // UI Camera (renders on top of 3D)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // Render after 3D camera
            clear_color: ClearColorConfig::None, // Don't clear - show 3D behind
            ..default()
        },
    ));

    // Clone font handle for use in closures
    let icon_font_handle = icon_font.0.clone();
    
    // Semi-transparent background color for UI
    let ui_bg = theme.surface.with_alpha(0.85);
    let scrollbar_bg = theme.surface_container_highest.with_alpha(0.5);

    // Root container using grid layout to hold scroll area + scrollbar
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: vec![
                    bevy::ui::RepeatedGridTrack::flex(1, 1.0),
                    bevy::ui::RepeatedGridTrack::px(1, 12.0), // Scrollbar width
                ],
                grid_template_rows: vec![bevy::ui::RepeatedGridTrack::flex(1, 1.0)],
                ..default()
            },
            BackgroundColor(Color::NONE), // Transparent root to see 3D
        ))
        .with_children(|grid| {
            let font_for_content = icon_font_handle.clone();
            let scroll_area_theme = theme.clone();
            
            // Scrollable area (left column) - semi-transparent
            // Using scroll_y() for native scrolling - we clamp ScrollPosition every frame
            let scroll_area_id = grid
                .spawn((
                    ScrollableRoot,
                    ScrollPosition::default(),
                    Node {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        grid_column: bevy::ui::GridPlacement::start(1),
                        grid_row: bevy::ui::GridPlacement::start(1),
                        ..default()
                    },
                    BackgroundColor(ui_bg),
                ))
                .with_children(|scroll_area| {
                    // Content container
                    scroll_area.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(32.0),
                            padding: UiRect::all(Val::Px(32.0)),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                    )).with_children(|content| {
                        // Title
                        content.spawn((
                            Text::new("Material Design 3 Component Library"),
                            TextFont { font_size: 28.0, ..default() },
                            TextColor(scroll_area_theme.on_surface),
                        ));
                        
                        content.spawn((
                            Text::new("Interactive showcase with code examples for each component"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(scroll_area_theme.on_surface_variant),
                            Node { margin: UiRect::bottom(Val::Px(16.0)), ..default() },
                        ));

                        // Buttons Section
                        spawn_buttons_section(content, &scroll_area_theme);

                        // Checkboxes Section
                        spawn_checkboxes_section(content, &scroll_area_theme, Some(font_for_content.clone()));

                        // Switches Section
                        spawn_switches_section(content, &scroll_area_theme);

                        // Radio Buttons Section
                        spawn_radios_section(content, &scroll_area_theme);
                        
                        // Chips Section
                        spawn_chips_section(content, &scroll_area_theme);
                        
                        // FAB Section
                        spawn_fab_section(content, &scroll_area_theme);
                        
                        // Badges Section
                        spawn_badges_section(content, &scroll_area_theme);
                        
                        // Progress Section
                        spawn_progress_section(content, &scroll_area_theme);
                        
                        // Cards Section
                        spawn_cards_section(content, &scroll_area_theme);
                        
                        // Dividers Section
                        spawn_dividers_section(content, &scroll_area_theme);
                        
                        // List Section (with scroll)
                        spawn_list_section(content, &scroll_area_theme);
                        
                        // Icons Section
                        spawn_icons_section(content, &scroll_area_theme, font_for_content.clone());
                        
                        // Theme Colors Section
                        spawn_theme_section(content, &scroll_area_theme);
                    });
                })
                .id();
            
            // MD3 Styled Scrollbar Track (right column)
            grid.spawn((
                ScrollbarTrack,
                Node {
                    grid_column: bevy::ui::GridPlacement::start(2),
                    grid_row: bevy::ui::GridPlacement::start(1),
                    width: Val::Px(12.0),
                    height: Val::Percent(100.0),
                    // No padding - we'll handle positioning manually
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(scrollbar_bg),
            ))
            .with_children(|track| {
                // MD3 Scrollbar Thumb
                track.spawn((
                    ScrollbarThumb { target: scroll_area_id },
                    Button,
                    Interaction::None,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(8.0),
                        height: Val::Px(100.0), // Will be updated dynamically
                        top: Val::Px(2.0),      // Initial offset from top
                        left: Val::Px(2.0),
                        ..default()
                    },
                    BackgroundColor(theme.primary.with_alpha(0.6)),
                    BorderRadius::all(Val::Px(4.0)),
                ));
            });
        });
}

/// Marker component for the scrollable root container
#[derive(Component)]
struct ScrollableRoot;

/// Marker component for scrollbar track
#[derive(Component)]
struct ScrollbarTrack;

/// Scrollbar thumb component with reference to its scroll target
#[derive(Component)]
struct ScrollbarThumb {
    target: Entity,
}

/// Resource to track scrollbar drag state
#[derive(Resource, Default)]
struct ScrollbarDragState {
    /// The thumb being dragged
    dragging_thumb: Option<Entity>,
    /// Y position when drag started
    start_cursor_y: f32,
    /// Scroll position when drag started
    start_scroll_y: f32,
}

/// System to update scrollbar thumb size and position based on scroll state
fn update_scrollbar_thumb(
    scroll_query: Query<(&ScrollPosition, &ComputedNode), With<ScrollableRoot>>,
    track_query: Query<&ComputedNode, (With<ScrollbarTrack>, Without<ScrollbarThumb>)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut Node), Without<ScrollbarTrack>>,
) {
    // Fixed inset from track edges
    const TRACK_INSET: f32 = 2.0;
    
    for (thumb, mut thumb_node) in thumb_query.iter_mut() {
        // Get the scroll area info
        if let Ok((scroll_pos, scroll_computed)) = scroll_query.get(thumb.target) {
            // Get track height
            if let Ok(track_computed) = track_query.single() {
                let content_height = scroll_computed.content_size().y;
                let container_height = scroll_computed.size().y;
                let track_total_height = track_computed.size().y;
                
                // Inner area where thumb can move (with inset on both ends)
                let track_inner_height = track_total_height - (TRACK_INSET * 2.0);
                
                // Calculate max scroll - this is how much the content can scroll
                let max_scroll = (content_height - container_height).max(0.0);
                
                // Calculate thumb size as percentage of visible content
                let visible_ratio = if content_height > 0.0 {
                    (container_height / content_height).min(1.0)
                } else {
                    1.0
                };
                let thumb_height = (track_inner_height * visible_ratio).max(30.0); // Min thumb size
                
                // Calculate thumb position
                let scroll_ratio = if max_scroll > 0.0 {
                    (scroll_pos.y / max_scroll).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                // Max offset is track inner height minus thumb height
                let max_thumb_offset = (track_inner_height - thumb_height).max(0.0);
                let thumb_top = (scroll_ratio * max_thumb_offset).min(max_thumb_offset);
                
                // Update thumb node
                thumb_node.height = Val::Px(thumb_height);
                // Position: TRACK_INSET + thumb_top, clamped so thumb never exceeds track bottom
                thumb_node.top = Val::Px(TRACK_INSET + thumb_top);
            }
        }
    }
}

/// System to handle scrollbar thumb dragging
fn scrollbar_thumb_drag_system(
    mut drag_state: ResMut<ScrollbarDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    thumb_query: Query<(Entity, &ScrollbarThumb, &Interaction)>,
    track_query: Query<&ComputedNode, With<ScrollbarTrack>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode), With<ScrollableRoot>>,
) {
    const TRACK_INSET: f32 = 2.0;
    
    let window = windows.single().expect("No window found");
    let cursor_pos = window.cursor_position();
    
    // Check for drag start
    for (thumb_entity, thumb, interaction) in thumb_query.iter() {
        if *interaction == Interaction::Pressed && drag_state.dragging_thumb.is_none() {
            if let Some(pos) = cursor_pos {
                if let Ok((scroll_pos, _)) = scroll_query.get(thumb.target) {
                    drag_state.dragging_thumb = Some(thumb_entity);
                    drag_state.start_cursor_y = pos.y;
                    drag_state.start_scroll_y = scroll_pos.y;
                }
            }
        }
    }
    
    // Handle active drag
    if let Some(dragging_thumb) = drag_state.dragging_thumb {
        if !mouse_button.pressed(MouseButton::Left) {
            // Drag ended
            drag_state.dragging_thumb = None;
        } else if let Some(pos) = cursor_pos {
            // Get the thumb's target
            if let Ok((thumb_entity_check, thumb, _)) = thumb_query.get(dragging_thumb) {
                if thumb_entity_check == dragging_thumb {
                    if let Ok(track_computed) = track_query.single() {
                        if let Ok((mut scroll_pos, scroll_computed)) = scroll_query.get_mut(thumb.target) {
                            let content_size = scroll_computed.content_size();
                            let container_size = scroll_computed.size();
                            
                            // Inner track area (with inset on both ends)
                            let track_inner_height = track_computed.size().y - (TRACK_INSET * 2.0);
                            
                            // Calculate max scroll (content that extends beyond container)
                            let max_scroll = (content_size.y - container_size.y).max(0.0);
                            
                            // Calculate visible ratio for thumb size
                            let visible_ratio = if content_size.y > 0.0 {
                                (container_size.y / content_size.y).min(1.0)
                            } else {
                                1.0
                            };
                            let thumb_height = (track_inner_height * visible_ratio).max(30.0);
                            let max_thumb_offset = (track_inner_height - thumb_height).max(0.0);
                            
                            if max_thumb_offset > 0.0 && max_scroll > 0.0 {
                                // Calculate cursor delta in screen space
                                // Bevy's window cursor Y=0 is at TOP, positive Y is downward
                                let cursor_delta = pos.y - drag_state.start_cursor_y;
                                
                                // Convert cursor delta to scroll delta (proportional mapping)
                                let scroll_delta = (cursor_delta / max_thumb_offset) * max_scroll;
                                
                                // Update scroll position with clamping
                                scroll_pos.y = (drag_state.start_scroll_y + scroll_delta).clamp(0.0, max_scroll);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Mouse wheel scroll system that respects hover context
/// Scrolls the list if hovering over it, otherwise scrolls main area
#[allow(deprecated)]
fn mouse_scroll_system(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut list_query: Query<(&Interaction, &mut ScrollPosition, &ComputedNode), (With<ScrollableList>, Without<ScrollableRoot>)>,
    mut main_query: Query<(&mut ScrollPosition, &ComputedNode), (With<ScrollableRoot>, Without<ScrollableList>)>,
) {
    for event in mouse_wheel.read() {
        // Calculate scroll delta
        let delta_y = match event.unit {
            MouseScrollUnit::Line => -event.y * 21.0, // Line height
            MouseScrollUnit::Pixel => -event.y,
        };
        
        // Check if hovering over any scrollable list
        let mut scrolled_list = false;
        for (interaction, mut scroll_pos, computed) in list_query.iter_mut() {
            if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                let content_height = computed.content_size().y;
                let container_height = computed.size().y;
                let max_scroll = (content_height - container_height).max(0.0);
                
                scroll_pos.y = (scroll_pos.y + delta_y).clamp(0.0, max_scroll);
                scrolled_list = true;
                break;
            }
        }
        
        // If not hovering over a list, scroll the main area
        if !scrolled_list {
            for (mut scroll_pos, computed) in main_query.iter_mut() {
                let content_height = computed.content_size().y;
                let container_height = computed.size().y;
                let max_scroll = (content_height - container_height).max(0.0);
                
                scroll_pos.y = (scroll_pos.y + delta_y).clamp(0.0, max_scroll);
            }
        }
    }
}

/// System to clamp scroll positions to valid bounds every frame
/// This prevents Bevy's native scroll handling from exceeding content bounds
fn clamp_scroll_positions(
    mut main_query: Query<(&mut ScrollPosition, &ComputedNode), (With<ScrollableRoot>, Without<ScrollableList>)>,
    mut list_query: Query<(&mut ScrollPosition, &ComputedNode), (With<ScrollableList>, Without<ScrollableRoot>)>,
) {
    // Clamp main scroll area
    for (mut scroll_pos, computed) in main_query.iter_mut() {
        let content_height = computed.content_size().y;
        let container_height = computed.size().y;
        let max_scroll = (content_height - container_height).max(0.0);
        scroll_pos.y = scroll_pos.y.clamp(0.0, max_scroll);
    }
    
    // Clamp list scroll areas  
    for (mut scroll_pos, computed) in list_query.iter_mut() {
        let content_height = computed.content_size().y;
        let container_height = computed.size().y;
        let max_scroll = (content_height - container_height).max(0.0);
        scroll_pos.y = scroll_pos.y.clamp(0.0, max_scroll);
    }
}

// ============================================================================
// Helper: Spawn a code block
// ============================================================================

fn spawn_code_block(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, code: &str) {
    parent
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(16.0)),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(theme.surface_container.with_alpha(0.8)),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|block| {
            block.spawn((
                Text::new(code),
                TextFont { 
                    font_size: 12.0, 
                    ..default() 
                },
                TextColor(theme.on_surface.with_alpha(0.87)),
            ));
        });
}

fn spawn_section_header(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, title: &str, description: &str) {
    parent.spawn((
        Text::new(title),
        TextFont { font_size: 22.0, ..default() },
        TextColor(theme.primary),
    ));
    
    if !description.is_empty() {
        parent.spawn((
            Text::new(description),
            TextFont { font_size: 14.0, ..default() },
            TextColor(theme.on_surface_variant),
            Node {
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
        ));
    }
}

// ============================================================================
// Buttons Section
// ============================================================================

fn spawn_buttons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Buttons",
                "MD3 buttons with 5 variants: Filled, Outlined, Text, Elevated, and Tonal"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_interactive_button(row, theme, "Filled", ButtonVariant::Filled);
                    spawn_interactive_button(row, theme, "Outlined", ButtonVariant::Outlined);
                    spawn_interactive_button(row, theme, "Text", ButtonVariant::Text);
                    spawn_interactive_button(row, theme, "Elevated", ButtonVariant::Elevated);
                    spawn_interactive_button(row, theme, "Tonal", ButtonVariant::FilledTonal);
                });

            spawn_code_block(section, theme, 
r#"// Create a filled button
let button = MaterialButton::new("Click Me")
    .with_variant(ButtonVariant::Filled);

commands.spawn((
    button,
    Button,  // Required for interaction
    RippleHost::new(),
    Node { padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)), ..default() },
    BackgroundColor(theme.primary),
    BorderRadius::all(Val::Px(20.0)),
));"#);
        });
}

fn spawn_interactive_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    variant: ButtonVariant,
) {
    let button = MaterialButton::new(label).with_variant(variant);
    let text_color = button.text_color(theme);
    let bg_color = button.background_color(theme);
    let border_color = button.border_color(theme);
    let has_border = variant == ButtonVariant::Outlined;

    parent
        .spawn((
            button,
            Button, // This is key - Bevy's Button component enables interaction
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(text_color),
            ));
        });
}

// ============================================================================
// Checkboxes Section
// ============================================================================

fn spawn_checkboxes_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Option<Handle<Font>>,
) {
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
                "Checkboxes",
                "Toggle selection with visual checkmark feedback"
            );

            let font = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_interactive_checkbox(col, theme, "Option 1", true, font.clone());
                    spawn_interactive_checkbox(col, theme, "Option 2", false, font.clone());
                    spawn_interactive_checkbox(col, theme, "Option 3", false, font.clone());
                });

            spawn_code_block(section, theme,
r#"// Create a checkbox (unchecked by default)
let checkbox = MaterialCheckbox::new();

// Create a pre-checked checkbox
let checkbox = MaterialCheckbox::new().checked();

// Listen for changes
fn handle_checkbox_changes(
    mut events: MessageReader<CheckboxChangeEvent>,
) {
    for event in events.read() {
        info!("Checkbox {:?} -> {}", event.entity, event.checked);
    }
}"#);
        });
}

fn spawn_interactive_checkbox(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    checked: bool,
    icon_font: Option<Handle<Font>>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            let checkbox = if checked {
                MaterialCheckbox::new().checked()
            } else {
                MaterialCheckbox::new()
            };

            let is_checked = checkbox.state.is_checked();
            let bg_color = if is_checked { theme.primary } else { Color::NONE };
            let border_color = if is_checked { theme.primary } else { theme.on_surface_variant };
            let icon_text = if is_checked { ICON_CHECK.to_string() } else { String::new() };

            // Spawn the checkbox touch target with the component
            row.spawn((
                checkbox,
                Button, // Enables Bevy interaction
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|touch_target| {
                // State layer for hover/press effects
                touch_target.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(20.0)),
                ))
                .with_children(|state_layer| {
                    // The visual checkbox box
                    state_layer.spawn((
                        CheckboxBox,
                        Node {
                            width: Val::Px(18.0),
                            height: Val::Px(18.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        BorderColor::all(border_color),
                        BorderRadius::all(Val::Px(2.0)),
                    ))
                    .with_children(|box_node| {
                        // Checkmark icon using Material Symbols font
                        let text_font = if let Some(ref font) = icon_font {
                            TextFont { 
                                font: font.clone(),
                                font_size: 14.0, 
                                ..default() 
                            }
                        } else {
                            TextFont { font_size: 14.0, ..default() }
                        };
                        
                        box_node.spawn((
                            CheckboxIcon,
                            Text::new(icon_text),
                            text_font,
                            TextColor(theme.on_primary),
                        ));
                    });
                });
            });

            // Label
            row.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(theme.on_surface),
            ));
        });
}

// ============================================================================
// Switches Section
// ============================================================================

fn spawn_switches_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Switches",
                "Toggle on/off with sliding thumb animation"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_interactive_switch(col, theme, "Wi-Fi", true);
                    spawn_interactive_switch(col, theme, "Bluetooth", false);
                    spawn_interactive_switch(col, theme, "Dark Mode", false);
                });

            spawn_code_block(section, theme,
r#"// Create a switch (off by default)
let switch = MaterialSwitch::new();

// Create an on switch
let switch = MaterialSwitch::new().selected(true);

// Listen for changes
fn handle_switch_changes(
    mut events: MessageReader<SwitchChangeEvent>,
) {
    for event in events.read() {
        info!("Switch {:?} -> {}", event.entity, event.on);
    }
}"#);
        });
}

fn spawn_interactive_switch(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    on: bool,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            let switch = MaterialSwitch::new().selected(on);

            let is_on = switch.selected;
            let track_color = if is_on { theme.primary } else { theme.surface_container_highest };
            let handle_color = if is_on { theme.on_primary } else { theme.outline };
            let border_color = if is_on { theme.primary } else { theme.outline };

            // Switch container with component
            row.spawn((
                switch,
                Button,
                Node {
                    width: Val::Px(52.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(2.0)),
                    justify_content: if is_on { JustifyContent::FlexEnd } else { JustifyContent::FlexStart },
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(track_color),
                BorderColor::all(border_color),
                BorderRadius::all(Val::Px(16.0)),
            ))
            .with_children(|track| {
                // Handle
                track.spawn((
                    SwitchHandle,
                    Node {
                        width: Val::Px(if is_on { 24.0 } else { 16.0 }),
                        height: Val::Px(if is_on { 24.0 } else { 16.0 }),
                        ..default()
                    },
                    BackgroundColor(handle_color),
                    BorderRadius::all(Val::Px(12.0)),
                ));
            });

            // Label
            row.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(theme.on_surface),
            ));
        });
}

// ============================================================================
// Radio Buttons Section
// ============================================================================

fn spawn_radios_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Radio Buttons",
                "Single selection within a group - only one can be selected"
            );

            section
                .spawn((
                    RadioGroup::new("example_group"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    },
                ))
                .with_children(|col| {
                    spawn_interactive_radio(col, theme, "Choice A", "choice_a", true);
                    spawn_interactive_radio(col, theme, "Choice B", "choice_b", false);
                    spawn_interactive_radio(col, theme, "Choice C", "choice_c", false);
                });

            spawn_code_block(section, theme,
r#"// Create radios in a group
commands.spawn((
    RadioGroup::new("my_group"),
    Node { flex_direction: FlexDirection::Column, ..default() },
)).with_children(|group| {
    // Each radio must reference the group name
    let radio = MaterialRadio::new()
        .selected(true)  // First one selected
        .group("my_group");
    
    group.spawn((radio, Button, ..));
});"#);
        });
}

fn spawn_interactive_radio(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    _value: &str,
    selected: bool,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Create radio with group set
            let radio = MaterialRadio::new()
                .selected(selected)
                .group("example_group"); // Must match RadioGroup name

            let is_selected = radio.selected;
            let border_color = if is_selected { theme.primary } else { theme.on_surface_variant };

            // Radio touch target
            row.spawn((
                radio,
                Button,
                Interaction::None,
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|touch| {
                // Outer circle
                touch
                    .spawn((
                        RadioOuter,
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(border_color),
                        BorderRadius::all(Val::Px(10.0)),
                    ))
                    .with_children(|outer| {
                        // Inner dot (visibility controlled by update system)
                        outer.spawn((
                            RadioInner,
                            Node {
                                width: Val::Px(10.0),
                                height: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(if is_selected { theme.primary } else { Color::NONE }),
                            BorderRadius::all(Val::Px(5.0)),
                        ));
                    });
            });

            // Label
            row.spawn((
                Text::new(label),
                TextFont { font_size: 14.0, ..default() },
                TextColor(theme.on_surface),
            ));
        });
}

/// Marker for radio outer circle
#[derive(Component)]
struct RadioOuter;

/// Marker for radio inner dot
#[derive(Component)]
struct RadioInner;

// ============================================================================
// Event Handlers
// ============================================================================

fn handle_button_clicks(mut events: MessageReader<ButtonClickEvent>) {
    for event in events.read() {
        info!("🔘 Button clicked: {:?}", event.entity);
    }
}

fn handle_checkbox_changes(mut events: MessageReader<CheckboxChangeEvent>) {
    for event in events.read() {
        info!("☑️ Checkbox changed: {:?} -> {:?}", event.entity, event.state);
    }
}

fn handle_switch_changes(mut events: MessageReader<SwitchChangeEvent>) {
    for event in events.read() {
        info!("🔀 Switch changed: {:?} -> {}", event.entity, event.selected);
    }
}

fn handle_radio_changes(mut events: MessageReader<RadioChangeEvent>) {
    for event in events.read() {
        info!("🔘 Radio changed: {:?} -> {}", event.entity, event.selected);
    }
}

// ============================================================================
// Visual Update Systems
// ============================================================================

fn update_checkbox_visuals(
    theme: Res<MaterialTheme>,
    icon_font: Option<Res<MaterialIconFont>>,
    checkboxes: Query<(&MaterialCheckbox, &Children), Changed<MaterialCheckbox>>,
    mut boxes: Query<(&mut BackgroundColor, &mut BorderColor), With<CheckboxBox>>,
    mut icons: Query<(&mut Text, &mut TextFont, &mut TextColor), With<CheckboxIcon>>,
    children_query: Query<&Children>,
) {
    for (checkbox, children) in checkboxes.iter() {
        let is_checked = checkbox.state.is_checked();
        let is_indeterminate = checkbox.state.is_indeterminate();
        let bg = if is_checked || is_indeterminate { theme.primary } else { Color::NONE };
        let border = if is_checked || is_indeterminate { theme.primary } else { theme.on_surface_variant };
        let icon_color = theme.on_primary;

        // Get the icon character
        let icon_text = checkbox.state.icon()
            .map(|c| c.to_string())
            .unwrap_or_default();

        // Navigate through children to find CheckboxBox and CheckboxIcon
        for child in children.iter() {
            // Check state layer children
            if let Ok(state_layer_children) = children_query.get(child) {
                for slc in state_layer_children.iter() {
                    // Update the checkbox box
                    if let Ok((mut bg_color, mut border_color)) = boxes.get_mut(slc) {
                        bg_color.0 = bg;
                        *border_color = BorderColor::all(border);
                    }
                    // Find icon in box children
                    if let Ok(box_children) = children_query.get(slc) {
                        for bc in box_children.iter() {
                            if let Ok((mut text, mut text_font, mut color)) = icons.get_mut(bc) {
                                **text = icon_text.clone();
                                color.0 = icon_color;
                                // Use Material Symbols font if available
                                if let Some(ref font) = icon_font {
                                    text_font.font = font.0.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_switch_visuals(
    theme: Res<MaterialTheme>,
    switches: Query<(Entity, &MaterialSwitch, &Children), Changed<MaterialSwitch>>,
    mut nodes: Query<&mut Node>,
    mut backgrounds: Query<&mut BackgroundColor>,
    mut borders: Query<&mut BorderColor>,
) {
    for (entity, switch, children) in switches.iter() {
        let is_on = switch.selected;
        let track_color = if is_on { theme.primary } else { theme.surface_container_highest };
        let handle_color = if is_on { theme.on_primary } else { theme.outline };
        let border = if is_on { theme.primary } else { theme.outline };

        // Update track (the switch entity itself)
        if let Ok(mut bg) = backgrounds.get_mut(entity) {
            bg.0 = track_color;
        }
        if let Ok(mut bc) = borders.get_mut(entity) {
            *bc = BorderColor::all(border);
        }
        if let Ok(mut node) = nodes.get_mut(entity) {
            node.justify_content = if is_on { JustifyContent::FlexEnd } else { JustifyContent::FlexStart };
        }

        // Update handle (first child)
        for child in children.iter() {
            if let Ok(mut bg) = backgrounds.get_mut(child) {
                bg.0 = handle_color;
            }
            if let Ok(mut node) = nodes.get_mut(child) {
                let size = if is_on { 24.0 } else { 16.0 };
                node.width = Val::Px(size);
                node.height = Val::Px(size);
            }
        }
    }
}

fn update_radio_visuals(
    theme: Res<MaterialTheme>,
    radios: Query<(&MaterialRadio, &Children)>,
    mut outer_query: Query<(&mut BorderColor, &Children), With<RadioOuter>>,
    mut inner_query: Query<&mut BackgroundColor, With<RadioInner>>,
) {
    for (radio, children) in radios.iter() {
        let is_selected = radio.selected;
        let border_color = if is_selected { theme.primary } else { theme.on_surface_variant };
        let inner_color = if is_selected { theme.primary } else { Color::NONE };

        // RadioOuter is a direct child of the radio button entity
        for child in children.iter() {
            // Update outer circle border
            if let Ok((mut border, outer_children)) = outer_query.get_mut(child) {
                *border = BorderColor::all(border_color);
                
                // Update inner dot
                for inner_entity in outer_children.iter() {
                    if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                        bg.0 = inner_color;
                    }
                }
            }
        }
    }
}

// ============================================================================
// Chips Section
// ============================================================================

fn spawn_chips_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Chips",
                "Compact elements for filters, selections, and actions"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_chip(row, theme, "Filter", false);
                    spawn_chip(row, theme, "Selected", true);
                    spawn_chip(row, theme, "Tag", false);
                    spawn_chip(row, theme, "Action", false);
                });

            spawn_code_block(section, theme,
r#"// Create an assist chip
let chip = MaterialChip::assist("Label");

// Create a filter chip (toggleable)
let chip = MaterialChip::filter("Category")
    .selected(true);

// Create an input chip (with close button)
let chip = MaterialChip::input("User Input");"#);
        });
}

fn spawn_chip(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, selected: bool) {
    let bg_color = if selected { theme.secondary_container } else { Color::NONE };
    let border_color = if selected { theme.secondary_container } else { theme.outline };
    let text_color = if selected { theme.on_secondary_container } else { theme.on_surface_variant };
    
    parent.spawn((
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(border_color),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|chip| {
        chip.spawn((
            Text::new(label),
            TextFont { font_size: 14.0, ..default() },
            TextColor(text_color),
        ));
    });
}

// ============================================================================
// FAB Section
// ============================================================================

fn spawn_fab_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Floating Action Buttons",
                "Primary actions with prominent visual treatment"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Small FAB
                    spawn_fab(row, theme, 40.0, "+", "Small");
                    // Regular FAB
                    spawn_fab(row, theme, 56.0, "+", "Regular");
                    // Large FAB
                    spawn_fab(row, theme, 96.0, "+", "Large");
                });

            spawn_code_block(section, theme,
r#"// Create a FAB
let fab = MaterialFab::new()
    .icon(ICON_ADD)
    .size(FabSize::Regular);

// Extended FAB with label
let fab = MaterialFab::new()
    .icon(ICON_ADD)
    .label("Create")
    .extended(true);"#);
        });
}

fn spawn_fab(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, size: f32, icon: &str, _label: &str) {
    parent.spawn((
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.primary_container),
        BorderRadius::all(Val::Px(size / 3.5)),
    )).with_children(|fab| {
        fab.spawn((
            Text::new(icon),
            TextFont { font_size: size * 0.4, ..default() },
            TextColor(theme.on_primary_container),
        ));
    });
}

// ============================================================================
// Badges Section
// ============================================================================

fn spawn_badges_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Badges",
                "Notification indicators for counts and status"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Dot badge
                    spawn_badge_example(row, theme, None);
                    // Small count
                    spawn_badge_example(row, theme, Some("3"));
                    // Large count
                    spawn_badge_example(row, theme, Some("99+"));
                });

            spawn_code_block(section, theme,
r#"// Dot badge (no text)
let badge = MaterialBadge::dot();

// Count badge
let badge = MaterialBadge::count(5);

// Count badge with max
let badge = MaterialBadge::count(150).max(99); // Shows "99+""#);
        });
}

fn spawn_badge_example(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, count: Option<&str>) {
    parent.spawn((
        Node {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.surface_container),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|container| {
        // Icon placeholder
        container.spawn((
            Text::new("🔔"),
            TextFont { font_size: 20.0, ..default() },
        ));
        
        // Badge
        let (width, text) = match count {
            None => (Val::Px(8.0), String::new()),
            Some(c) => (Val::Auto, c.to_string()),
        };
        
        container.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(2.0),
                right: Val::Px(2.0),
                width,
                min_width: Val::Px(if count.is_some() { 16.0 } else { 8.0 }),
                height: Val::Px(if count.is_some() { 16.0 } else { 8.0 }),
                padding: UiRect::axes(Val::Px(4.0), Val::Px(0.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.error),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|badge| {
            if !text.is_empty() {
                badge.spawn((
                    Text::new(text),
                    TextFont { font_size: 10.0, ..default() },
                    TextColor(theme.on_error),
                ));
            }
        });
    });
}

// ============================================================================
// Progress Section
// ============================================================================

fn spawn_progress_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Progress Indicators",
                "Visual feedback for loading and progress states"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Linear progress at 0%
                    spawn_linear_progress(col, theme, 0.0, "0%");
                    // Linear progress at 50%
                    spawn_linear_progress(col, theme, 0.5, "50%");
                    // Linear progress at 100%
                    spawn_linear_progress(col, theme, 1.0, "100%");
                });

            spawn_code_block(section, theme,
r#"// Linear progress (determinate)
let progress = LinearProgress::new(0.5); // 50%

// Indeterminate progress
let progress = LinearProgress::indeterminate();

// Circular progress
let progress = CircularProgress::new(0.75);"#);
        });
}

fn spawn_linear_progress(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, value: f32, label: &str) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(12.0),
        ..default()
    }).with_children(|row| {
        // Track
        row.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(theme.surface_container_highest),
            BorderRadius::all(Val::Px(2.0)),
        )).with_children(|track| {
            // Indicator
            track.spawn((
                Node {
                    width: Val::Percent(value * 100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.primary),
                BorderRadius::all(Val::Px(2.0)),
            ));
        });
        
        // Label
        row.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(theme.on_surface_variant),
        ));
    });
}

// ============================================================================
// Cards Section
// ============================================================================

fn spawn_cards_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Cards",
                "Containers for related content and actions"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_card(row, theme, "Elevated", CardType::Elevated);
                    spawn_card(row, theme, "Filled", CardType::Filled);
                    spawn_card(row, theme, "Outlined", CardType::Outlined);
                });

            spawn_code_block(section, theme,
r#"// Create an elevated card
let card = MaterialCard::new()
    .variant(CardVariant::Elevated);

// Filled card
let card = MaterialCard::filled();

// Outlined card  
let card = MaterialCard::outlined();"#);
        });
}

#[derive(Clone, Copy)]
enum CardType { Elevated, Filled, Outlined }

fn spawn_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, title: &str, card_type: CardType) {
    let (bg_color, border_width) = match card_type {
        CardType::Elevated => (theme.surface_container_low, 0.0),
        CardType::Filled => (theme.surface_container_highest, 0.0),
        CardType::Outlined => (theme.surface, 1.0),
    };
    
    parent.spawn((
        Node {
            width: Val::Px(160.0),
            padding: UiRect::all(Val::Px(16.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            border: UiRect::all(Val::Px(border_width)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(theme.outline_variant),
        BorderRadius::all(Val::Px(12.0)),
    )).with_children(|card| {
        card.spawn((
            Text::new(title),
            TextFont { font_size: 16.0, ..default() },
            TextColor(theme.on_surface),
        ));
        card.spawn((
            Text::new("Card content goes here with supporting text."),
            TextFont { font_size: 12.0, ..default() },
            TextColor(theme.on_surface_variant),
        ));
    });
}

// ============================================================================
// Dividers Section
// ============================================================================

fn spawn_dividers_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Dividers",
                "Visual separators between content sections"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Content above divider"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                    
                    // Full-width divider
                    col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            ..default()
                        },
                        BackgroundColor(theme.outline_variant),
                    ));
                    
                    col.spawn((
                        Text::new("Content below divider"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                    
                    // Inset divider
                    col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::left(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(theme.outline_variant),
                    ));
                    
                    col.spawn((
                        Text::new("After inset divider"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                });

            spawn_code_block(section, theme,
r#"// Full-width divider
commands.spawn((
    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
    BackgroundColor(theme.outline_variant),
));

// Inset divider (with left margin)
commands.spawn((
    Node { 
        width: Val::Percent(100.0), 
        height: Val::Px(1.0),
        margin: UiRect::left(Val::Px(16.0)),
        ..default() 
    },
    BackgroundColor(theme.outline_variant),
));"#);
        });
}

// ============================================================================
// List Section (with scrolling)
// ============================================================================

fn spawn_list_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let theme_clone = theme.clone();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section, 
                &theme_clone, 
                "Lists (with Scroll)",
                "Scrollable list showing 4 of 10 items - use mouse wheel when hovering"
            );

            // Container for list with scrollbar
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start, // Track aligns to top of list
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    ..default()
                })
                .with_children(|container| {
                    // Calculate list height - 4 items visible, 2-line variant = 72px per item
                    let list_height = 4.0 * 72.0; // 288px
                    
                    // Scrollable list using the new API
                    let scroll_area_id = container
                        .spawn((
                            ListBuilder::new()
                                .max_visible_items_variant(4, bevy_material_ui::list::ListItemVariant::TwoLine)
                                .build_scrollable(),
                            BackgroundColor(theme_clone.surface_container_low),
                            BorderRadius::all(Val::Px(12.0)),
                            Interaction::None, // Enable hover detection
                        ))
                        .with_children(|list| {
                            // 10 list items
                            let items = [
                                ("Inbox", "Primary inbox for emails"),
                                ("Starred", "Important messages"),
                                ("Sent", "Outgoing messages"),
                                ("Drafts", "Unfinished messages"),
                                ("Spam", "Filtered junk mail"),
                                ("Trash", "Deleted items"),
                                ("Archive", "Stored messages"),
                                ("Labels", "Organized categories"),
                                ("Settings", "Configuration options"),
                                ("Help", "Support and documentation"),
                            ];

                            for (headline, supporting) in items.iter() {
                                list.spawn(
                                    ListItemBuilder::new(*headline)
                                        .two_line()
                                        .supporting_text(*supporting)
                                        .build(&theme_clone)
                                )
                                .with_children(|item| {
                                    // Body with text
                                    item.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        flex_grow: 1.0,
                                        ..default()
                                    })
                                    .with_children(|body| {
                                        body.spawn((
                                            Text::new(*headline),
                                            TextFont { font_size: 16.0, ..default() },
                                            TextColor(theme_clone.on_surface),
                                        ));
                                        body.spawn((
                                            Text::new(*supporting),
                                            TextFont { font_size: 14.0, ..default() },
                                            TextColor(theme_clone.on_surface_variant),
                                        ));
                                    });
                                });
                            }
                        })
                        .id();
                    
                    // Mini scrollbar track for the list - MUST match list height exactly
                    container
                        .spawn((
                            ListScrollTrack,
                            Node {
                                position_type: PositionType::Relative, // Position context for absolute thumb
                                width: Val::Px(12.0),
                                height: Val::Px(list_height), // Same height as list
                                overflow: Overflow::clip(), // Clip thumb to track bounds
                                ..default()
                            },
                            BackgroundColor(theme_clone.surface_container_highest.with_alpha(0.3)),
                            BorderRadius::all(Val::Px(4.0)),
                        ))
                        .with_children(|track| {
                            track.spawn((
                                ListScrollThumb { target: scroll_area_id },
                                Button,
                                Interaction::None,
                                Node {
                                    position_type: PositionType::Absolute,
                                    // left, right, top, height will be set by update_list_scroll_thumb system
                                    ..default()
                                },
                                BackgroundColor(theme_clone.primary.with_alpha(0.6)),
                                BorderRadius::all(Val::Px(4.0)),
                            ));
                        });
                });

            spawn_code_block(section, &theme_clone,
r#"// Scrollable list with 10 items, showing 4 at a time
// Using the new ListBuilder API with built-in scroll support
commands.spawn((
    ListBuilder::new()
        .max_visible_items_variant(4, ListItemVariant::TwoLine)
        .build_scrollable(),
    BackgroundColor(theme.surface_container_low),
)).with_children(|list| {
    for (headline, supporting) in items {
        list.spawn(
            ListItemBuilder::new(headline)
                .two_line()
                .supporting_text(supporting)
                .build(&theme)
        );
    }
});"#);
        });
}

/// Marker for the list's scroll track
#[derive(Component)]
struct ListScrollTrack;

/// Marker for the list's scroll thumb with target reference
#[derive(Component)]
struct ListScrollThumb {
    target: Entity,
}

/// Drag state for list scrollbar
#[derive(Resource, Default)]
struct ListScrollDragState {
    dragging_thumb: Option<Entity>,
    start_cursor_y: f32,
    start_scroll_y: f32,
}

// ============================================================================
// Icons Section
// ============================================================================

fn spawn_icons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Material Icons",
                "Google Material Symbols with variable font support"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Show several icons using Unicode codepoints
                    let icons = [
                        ("\u{e5ca}", "check"),      // check
                        ("\u{e88a}", "home"),       // home
                        ("\u{e8b8}", "settings"),   // settings
                        ("\u{e87d}", "favorite"),   // favorite
                        ("\u{e8b6}", "search"),     // search
                    ];
                    
                    for (icon_char, _name) in icons {
                        row.spawn((
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme.surface_container),
                            BorderRadius::all(Val::Px(8.0)),
                        )).with_children(|container| {
                            container.spawn((
                                Text::new(icon_char),
                                TextFont { 
                                    font: icon_font.clone(),
                                    font_size: 24.0, 
                                    ..default() 
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                    }
                });

            spawn_code_block(section, theme,
r#"// Using Material Symbols icons
use bevy_material_ui::icons::{ICON_CHECK, icon_by_name};

// By constant
commands.spawn((
    Text::new(ICON_CHECK),
    TextFont { font: icon_font.0.clone(), font_size: 24.0, ..default() },
));

// By name lookup
if let Some(codepoint) = icon_by_name("home") {
    // Use codepoint...
}"#);
        });
}

// ============================================================================
// Theme Colors Section
// ============================================================================

fn spawn_theme_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            margin: UiRect::bottom(Val::Px(32.0)), // Extra bottom margin
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section, 
                theme, 
                "Theme Colors",
                "Dynamic color scheme from Material You"
            );

            // Color swatches
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_color_swatch(row, "Primary", theme.primary, theme.on_primary);
                    spawn_color_swatch(row, "Secondary", theme.secondary, theme.on_secondary);
                    spawn_color_swatch(row, "Tertiary", theme.tertiary, theme.on_tertiary);
                    spawn_color_swatch(row, "Error", theme.error, theme.on_error);
                    spawn_color_swatch(row, "Surface", theme.surface, theme.on_surface);
                });

            spawn_code_block(section, theme,
r#"// Access theme colors
fn my_system(theme: Res<MaterialTheme>) {
    let primary = theme.primary;
    let on_primary = theme.on_primary;
    let surface = theme.surface;
    let error = theme.error;
}

// Generate custom scheme
let scheme = MaterialColorScheme::from_seed(
    Color::srgb(0.2, 0.4, 0.8), // Seed color
    false, // dark mode
);"#);
        });
}

fn spawn_color_swatch(parent: &mut ChildSpawnerCommands, name: &str, bg: Color, fg: Color) {
    parent.spawn((
        Node {
            width: Val::Px(80.0),
            height: Val::Px(60.0),
            padding: UiRect::all(Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|swatch| {
        swatch.spawn((
            Text::new(name),
            TextFont { font_size: 11.0, ..default() },
            TextColor(fg),
        ));
    });
}

// ============================================================================
// List Scroll Thumb Update System
// ============================================================================

fn update_list_scroll_thumb(
    list_query: Query<(&ScrollPosition, &ComputedNode), With<ScrollableList>>,
    track_query: Query<&ComputedNode, With<ListScrollTrack>>,
    mut thumb_query: Query<(&ListScrollThumb, &mut Node)>,
) {
    // Following Bevy's official scrollbar implementation from bevy_ui_widgets/src/scrollbar.rs
    const MIN_THUMB_SIZE: f32 = 20.0;
    const TRACK_INSET: f32 = 2.0;
    
    for (thumb, mut thumb_node) in thumb_query.iter_mut() {
        let Ok((scroll_pos, scroll_computed)) = list_query.get(thumb.target) else { 
            continue 
        };
        let Ok(track_computed) = track_query.single() else { 
            continue 
        };
        
        // Get values in logical pixels (matching Bevy's approach)
        let scale = scroll_computed.inverse_scale_factor();
        let visible_size = scroll_computed.size().y * scale;
        let content_size = scroll_computed.content_size().y * scale;
        let track_length = track_computed.size().y * track_computed.inverse_scale_factor();
        
        // Usable track length (minus insets)
        let usable_track = track_length - (TRACK_INSET * 2.0);
        
        // Calculate thumb size (Bevy's formula)
        let thumb_size = if content_size > visible_size {
            (usable_track * visible_size / content_size)
                .max(MIN_THUMB_SIZE)
                .min(usable_track)
        } else {
            usable_track
        };
        
        // Calculate thumb position (Bevy's formula)
        let mut offset = scroll_pos.y;
        let thumb_pos = if content_size > visible_size {
            let max_offset = content_size - visible_size;
            // Clamp offset to prevent thumb from going out of bounds
            offset = offset.clamp(0.0, max_offset);
            offset * (usable_track - thumb_size) / (content_size - visible_size)
        } else {
            0.0
        };
        
        // Apply to node - following Bevy's pattern for vertical scrollbar
        thumb_node.left = Val::Px(TRACK_INSET);
        thumb_node.right = Val::Px(TRACK_INSET);
        thumb_node.top = Val::Px(TRACK_INSET + thumb_pos);
        thumb_node.height = Val::Px(thumb_size);
    }
}

/// System to handle list scroll thumb dragging
fn list_scroll_thumb_drag_system(
    mut drag_state: ResMut<ListScrollDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    thumb_query: Query<(Entity, &ListScrollThumb, &Interaction)>,
    track_query: Query<&ComputedNode, With<ListScrollTrack>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode), With<ScrollableList>>,
) {
    const TRACK_INSET: f32 = 2.0;
    
    let Ok(window) = windows.single() else { return };
    let cursor_pos = window.cursor_position();
    
    // Check for drag start
    for (thumb_entity, thumb, interaction) in thumb_query.iter() {
        if *interaction == Interaction::Pressed && drag_state.dragging_thumb.is_none() {
            if let Some(pos) = cursor_pos {
                if let Ok((scroll_pos, _)) = scroll_query.get(thumb.target) {
                    drag_state.dragging_thumb = Some(thumb_entity);
                    drag_state.start_cursor_y = pos.y;
                    drag_state.start_scroll_y = scroll_pos.y;
                }
            }
        }
    }
    
    // Handle active drag
    if let Some(dragging_thumb) = drag_state.dragging_thumb {
        if !mouse_button.pressed(MouseButton::Left) {
            // Drag ended
            drag_state.dragging_thumb = None;
        } else if let Some(pos) = cursor_pos {
            // Get the thumb's target
            if let Ok((_, thumb, _)) = thumb_query.get(dragging_thumb) {
                if let Ok(track_computed) = track_query.single() {
                    if let Ok((mut scroll_pos, scroll_computed)) = scroll_query.get_mut(thumb.target) {
                        let content_height = scroll_computed.content_size().y;
                        let container_height = scroll_computed.size().y;
                        let track_inner_height = track_computed.size().y - (TRACK_INSET * 2.0);
                        
                        let max_scroll = (content_height - container_height).max(0.0);
                        
                        // Calculate thumb size
                        let visible_ratio = if content_height > 0.0 {
                            (container_height / content_height).min(1.0)
                        } else {
                            1.0
                        };
                        let thumb_height = (track_inner_height * visible_ratio).max(20.0);
                        let max_thumb_offset = (track_inner_height - thumb_height).max(0.0);
                        
                        if max_thumb_offset > 0.0 && max_scroll > 0.0 {
                            // Calculate cursor delta (positive Y is down in Bevy window coords)
                            let cursor_delta = pos.y - drag_state.start_cursor_y;
                            
                            // Convert cursor delta to scroll delta
                            let scroll_delta = (cursor_delta / max_thumb_offset) * max_scroll;
                            
                            // Update scroll position with clamping
                            scroll_pos.y = (drag_state.start_scroll_y + scroll_delta).clamp(0.0, max_scroll);
                        }
                    }
                }
            }
        }
    }
}
