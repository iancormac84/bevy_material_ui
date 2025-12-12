//! Interactive Material Design 3 UI Components Showcase
//!
//! This example demonstrates interactive Material Design 3 UI components
//! with proper event handling and visual feedback, overlaid on a 3D scene
//! with a spinning D10 dice.
//!
//! Run with: `cargo run --example showcase`

use bevy::prelude::*;
use bevy::window::WindowPosition;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use bevy_material_ui::prelude::*;
use bevy_material_ui::theme::blend_state_layer;
use bevy_material_ui::checkbox::{CheckboxChangeEvent, CheckboxBox, CheckboxIcon};
use bevy_material_ui::switch::SwitchChangeEvent;
use bevy_material_ui::radio::RadioChangeEvent;
use bevy_material_ui::list::{ListBuilder, ListItemBuilder, ScrollableList, MaterialListItem, ListItemClickEvent, ListItemHeadline};
use bevy_material_ui::icons::{
    ICON_CHECK, ICON_ADD, ICON_EDIT, ICON_DELETE, ICON_FAVORITE, ICON_STAR,
    ICON_MORE_VERT, ICON_EMAIL, ICON_NOTIFICATIONS, ICON_EXPAND_MORE,
    ICON_MENU, ICON_SEARCH, ICON_CLOSE,
};
use bevy_material_ui::text_field::TextFieldVariant;
use bevy_material_ui::select::SelectVariant;
use bevy_material_ui::snackbar::{ShowSnackbar, SnackbarHostBuilder};
use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

// ============================================================================
// TELEMETRY SYSTEM - Reports component state for automated testing
// ============================================================================

/// Global telemetry state that can be written to file for test tooling
#[derive(Resource, Default)]
pub struct ComponentTelemetry {
    /// Component states as key-value pairs
    pub states: HashMap<String, String>,
    /// Recent events log
    pub events: Vec<String>,
    /// Element bounds for test automation (test_id -> bounds)
    pub elements: HashMap<String, ElementBounds>,
    /// Whether to write telemetry to file
    pub enabled: bool,
}

impl ComponentTelemetry {
    pub fn log_event(&mut self, event: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        self.events.push(format!("[{}] {}", timestamp, event));
        // Keep only last 100 events
        if self.events.len() > 100 {
            self.events.remove(0);
        }
    }
    
    pub fn set_state(&mut self, key: &str, value: &str) {
        self.states.insert(key.to_string(), value.to_string());
    }
    
    pub fn set_element(&mut self, bounds: ElementBounds) {
        self.elements.insert(bounds.test_id.clone(), bounds);
    }
    
    pub fn write_to_file(&self) {
        if !self.enabled {
            return;
        }
        
        // Convert elements to serializable format
        let elements_json: Vec<_> = self.elements.values().map(|e| {
            serde_json::json!({
                "test_id": e.test_id,
                "x": e.x,
                "y": e.y,
                "width": e.width,
                "height": e.height,
                "parent": e.parent,
            })
        }).collect();
        
        let json = serde_json::json!({
            "states": self.states,
            "events": self.events,
            "elements": elements_json,
        });
        if let Ok(mut file) = File::create("telemetry.json") {
            let _ = file.write_all(json.to_string().as_bytes());
        }
    }
}

/// Enum representing all available component sections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentSection {
    #[default]
    Buttons,
    Checkboxes,
    Switches,
    RadioButtons,
    Chips,
    FAB,
    Badges,
    Progress,
    Cards,
    Dividers,
    Lists,
    Icons,
    IconButtons,
    Sliders,
    TextFields,
    Dialogs,
    Menus,
    Tabs,
    Select,
    Snackbar,
    Tooltips,
    AppBar,
    ThemeColors,
}

impl ComponentSection {
    /// Get display name for the component
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Checkboxes => "Checkboxes",
            Self::Switches => "Switches",
            Self::RadioButtons => "Radio Buttons",
            Self::Chips => "Chips",
            Self::FAB => "FAB",
            Self::Badges => "Badges",
            Self::Progress => "Progress",
            Self::Cards => "Cards",
            Self::Dividers => "Dividers",
            Self::Lists => "Lists",
            Self::Icons => "Icons",
            Self::IconButtons => "Icon Buttons",
            Self::Sliders => "Sliders",
            Self::TextFields => "Text Fields",
            Self::Dialogs => "Dialogs",
            Self::Menus => "Menus",
            Self::Tabs => "Tabs",
            Self::Select => "Select",
            Self::Snackbar => "Snackbar",
            Self::Tooltips => "Tooltips",
            Self::AppBar => "App Bar",
            Self::ThemeColors => "Theme Colors",
        }
    }
    
    /// Get all component sections in order
    pub fn all() -> &'static [ComponentSection] {
        &[
            Self::Buttons,
            Self::Checkboxes,
            Self::Switches,
            Self::RadioButtons,
            Self::Chips,
            Self::FAB,
            Self::Badges,
            Self::Progress,
            Self::Cards,
            Self::Dividers,
            Self::Lists,
            Self::Icons,
            Self::IconButtons,
            Self::Sliders,
            Self::TextFields,
            Self::Dialogs,
            Self::Menus,
            Self::Tabs,
            Self::Select,
            Self::Snackbar,
            Self::Tooltips,
            Self::AppBar,
            Self::ThemeColors,
        ]
    }
}

/// Resource tracking the currently selected component section
#[derive(Resource, Default)]
pub struct SelectedSection {
    pub current: ComponentSection,
}

/// Marker for navigation list items - stores the ComponentSection this item represents
#[derive(Component)]
struct NavItem(ComponentSection);

/// Marker for the detail content area
#[derive(Component)]
struct DetailContent;

/// Test ID for automated testing - allows tests to find elements by name
#[derive(Component, Debug)]
pub struct TestId(pub String);

impl TestId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Stores element bounds for test automation
#[derive(Debug, Clone)]
pub struct ElementBounds {
    pub test_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub parent: Option<String>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Material Design 3 Interactive Showcase".into(),
                resolution: bevy::window::WindowResolution::new(1400, 800),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MaterialUiPlugin)
        .init_resource::<ScrollbarDragState>()
        .init_resource::<SidebarScrollbarDragState>()
        .init_resource::<ListScrollDragState>()
        .init_resource::<SliderDragState>()
        .init_resource::<DialogState>()
        .init_resource::<MenuState>()
        .init_resource::<SelectState>()
        .init_resource::<SelectedListItem>()
        .init_resource::<TooltipDemoOptions>()
        .init_resource::<SnackbarDemoOptions>()
        .init_resource::<CursorBlinkTimer>()
        .init_resource::<TextFieldDemoOptions>()
        .init_resource::<SelectedSection>()
        .init_resource::<TabState>()
        .init_resource::<ListSelectionState>()
        .init_resource::<ComponentTelemetry>()
        .add_systems(Startup, (setup_3d_scene, setup_ui, setup_telemetry))
        // Split systems into multiple groups to avoid Bevy's tuple size limit
        .add_systems(Update, (
            handle_button_clicks,
            handle_checkbox_changes,
            handle_switch_changes,
            handle_radio_changes,
            handle_fab_clicks,
            handle_chip_clicks,
            handle_list_item_clicks,
            handle_slider_drag,
        ))
        .add_systems(Update, (
            handle_dialog_buttons,
            handle_menu_toggle,
            handle_menu_item_clicks,
            handle_icon_button_clicks,
            handle_app_bar_button_clicks,
            handle_snackbar_trigger,
            handle_text_field_focus,
            handle_text_field_input,
            handle_text_field_unfocus,
            update_checkbox_visuals,
            update_switch_visuals,
            update_radio_visuals,
            update_slider_visuals,
            update_dialog_visibility,
        ))
        .add_systems(Update, (
            update_menu_visibility,
            handle_select_toggle,
            update_select_visibility,
            handle_select_option_clicks,
            handle_menu_keyboard_shortcuts,
            mouse_scroll_system,
            clamp_scroll_positions,
            update_scrollbar_thumb,
            scrollbar_thumb_drag_system,
            update_list_scroll_thumb,
            list_scroll_thumb_drag_system,
            rotate_dice,
            update_sidebar_scrollbar,
            sidebar_scrollbar_thumb_drag_system,
        ))
        .add_systems(Update, (
            handle_tooltip_position_options,
            handle_tooltip_delay_options,
            update_tooltip_demo_button,
            handle_snackbar_duration_options,
            handle_snackbar_action_toggle,
            update_snackbar_duration_button_visuals,
            update_snackbar_action_button_visuals,
            update_cursor_blink,
            handle_text_field_blink_speed_options,
            handle_text_field_cursor_toggle,
            update_text_field_option_buttons,
            handle_nav_clicks,
            update_nav_highlights,
            update_detail_content,
        ))
        .add_systems(Update, (
            handle_tab_clicks,
            update_tab_visuals,
            update_tab_content_visibility,
            handle_list_selection,
            update_list_item_visuals,
            handle_dialog_position_options,
            update_dialog_position_button_visuals,
            handle_list_selection_mode_options,
            update_list_mode_button_visuals,
            handle_list_mode_button_feedback,
            update_telemetry,
        ))
        .run();
}

// ============================================================================
// TELEMETRY SYSTEMS
// ============================================================================

/// Initialize telemetry - check for environment variable to enable
fn setup_telemetry(mut telemetry: ResMut<ComponentTelemetry>) {
    // Enable telemetry if BEVY_TELEMETRY env var is set
    telemetry.enabled = std::env::var("BEVY_TELEMETRY").is_ok();
    if telemetry.enabled {
        info!("📊 Telemetry enabled - writing to telemetry.json");
        telemetry.log_event("Showcase started");
    }
}

/// Update telemetry state and write to file periodically
fn update_telemetry(
    mut telemetry: ResMut<ComponentTelemetry>,
    selected_section: Res<SelectedSection>,
    dialog_state: Res<DialogState>,
    menu_state: Res<MenuState>,
    select_state: Res<SelectState>,
    tab_state: Res<TabState>,
    slider_drag: Res<SliderDragState>,
    list_selection: Res<ListSelectionState>,
    sliders: Query<&SliderThumb>,
    nav_items: Query<(&NavItem, &MaterialListItem, &BackgroundColor)>,
    tabs: Query<(&TabButton, &Node, &BorderColor)>,
    list_items: Query<(Entity, &TestId), With<SelectableListItem>>,
    // Query elements with TestId for position reporting - use UiGlobalTransform for computed global position
    test_elements: Query<(Entity, &TestId, Option<&ComputedNode>, Option<&UiGlobalTransform>)>,
    // Query scroll positions for sidebar and main content
    sidebar_scroll: Query<(&ScrollPosition, &ComputedNode), With<SidebarScrollArea>>,
    main_scroll: Query<&ScrollPosition, With<ScrollableRoot>>,
    window: Single<&Window>,
) {
    if !telemetry.enabled {
        return;
    }
    
    // Debug: count test elements
    let total_test_ids = test_elements.iter().count();
    telemetry.set_state("test_id_count", &total_test_ids.to_string());
    
    // Update component states
    telemetry.set_state("selected_section", &format!("{:?}", selected_section.current));
    telemetry.set_state("dialog_open", &dialog_state.is_open.to_string());
    telemetry.set_state("menu_open", &format!("{:?}", menu_state.open_menu));
    telemetry.set_state("select_open", &format!("{:?}", select_state.open_select));
    telemetry.set_state("tab_selected", &tab_state.selected_tab.to_string());
    telemetry.set_state("slider_dragging", &format!("{:?}", slider_drag.dragging));
    
    // List selection state
    telemetry.set_state("list_selection_mode", &format!("{:?}", list_selection.mode));
    telemetry.set_state("list_selected_count", &list_selection.selected.len().to_string());
    // Report which list items are selected by their TestId
    let selected_items: Vec<String> = list_items.iter()
        .filter(|(entity, _)| list_selection.selected.contains(entity))
        .map(|(_, test_id)| test_id.0.clone())
        .collect();
    telemetry.set_state("list_selected_items", &format!("{:?}", selected_items));
    
    // Slider values
    for (i, slider) in sliders.iter().enumerate() {
        telemetry.set_state(&format!("slider_{}_value", i), &format!("{:.1}", slider.value));
        telemetry.set_state(&format!("slider_{}_step", i), &format!("{:?}", slider.step));
    }
    
    // Scroll positions for test automation
    if let Ok((scroll_pos, computed)) = sidebar_scroll.single() {
        telemetry.set_state("sidebar_scroll_y", &format!("{:.1}", scroll_pos.y));
        let container_size = computed.size();
        let content_size = computed.content_size();
        telemetry.set_state("sidebar_container_height", &format!("{:.1}", container_size.y));
        telemetry.set_state("sidebar_content_height", &format!("{:.1}", content_size.y));
        telemetry.set_state("sidebar_max_scroll", &format!("{:.1}", (content_size.y - container_size.y).max(0.0)));
    } else {
        telemetry.set_state("sidebar_scroll_y", "query_failed");
        telemetry.set_state("sidebar_query_count", &sidebar_scroll.iter().count().to_string());
    }
    if let Ok(scroll_pos) = main_scroll.single() {
        telemetry.set_state("main_scroll_y", &format!("{:.1}", scroll_pos.y));
    }
    
    // Navigation item selection and background color
    for (nav_item, list_item, bg) in nav_items.iter() {
        if list_item.selected {
            telemetry.set_state("nav_selected", &format!("{:?}", nav_item.0));
            telemetry.set_state("nav_selected_has_bg", &(bg.0 != Color::NONE).to_string());
        }
    }
    
    // Tab border check - verify both border width and color
    for (tab_btn, node, border_color) in tabs.iter() {
        let has_border_width = matches!(node.border.bottom, Val::Px(b) if b > 0.0);
        let has_border_color = border_color.bottom != Color::NONE;
        telemetry.set_state(&format!("tab_{}_has_border_width", tab_btn.index), &has_border_width.to_string());
        telemetry.set_state(&format!("tab_{}_has_border_color", tab_btn.index), &has_border_color.to_string());
    }
    
    // Report element bounds for TestId components
    let _window_width = window.width();
    let _window_height = window.height();
    
    // Report window position for test automation
    if let WindowPosition::At(pos) = window.position {
        telemetry.set_state("window_x", &pos.x.to_string());
        telemetry.set_state("window_y", &pos.y.to_string());
    } else {
        telemetry.set_state("window_x", "auto");
        telemetry.set_state("window_y", "auto");
    }
    telemetry.set_state("window_width", &window.width().to_string());
    telemetry.set_state("window_height", &window.height().to_string());
    
    let mut elements_with_bounds = 0;
    let mut elements_missing_computed = 0;
    let mut elements_missing_transform = 0;
    
    for (_entity, test_id, computed_node_opt, global_transform_opt) in test_elements.iter() {
        match (computed_node_opt, global_transform_opt) {
            (Some(computed_node), Some(global_transform)) => {
                // UiGlobalTransform derefs to Affine2, which has a translation field (Vec2)
                // In Bevy 0.17 UI, the translation is in screen coordinates:
                // - Origin at top-left of the window
                // - pos.x is distance from left edge to center of element
                // - pos.y is distance from top edge to center of element
                let pos = global_transform.translation;
                let size = computed_node.size();
                
                // Convert center coordinates to top-left corner for the element's bounding box
                let screen_x = pos.x - (size.x / 2.0);
                let screen_y = pos.y - (size.y / 2.0);
                
                telemetry.set_element(ElementBounds {
                    test_id: test_id.0.clone(),
                    x: screen_x,
                    y: screen_y,
                    width: size.x,
                    height: size.y,
                    parent: None,
                });
                elements_with_bounds += 1;
            },
            (None, _) => elements_missing_computed += 1,
            (_, None) => elements_missing_transform += 1,
        }
    }
    
    telemetry.set_state("elements_with_bounds", &elements_with_bounds.to_string());
    telemetry.set_state("elements_missing_computed", &elements_missing_computed.to_string());
    telemetry.set_state("elements_missing_transform", &elements_missing_transform.to_string());
    
    // Write to file
    telemetry.write_to_file();
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
    selected: Res<SelectedSection>,
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
    let ui_bg = theme.surface.with_alpha(0.92);
    let sidebar_bg = theme.surface_container.with_alpha(0.95);
    let scrollbar_bg = theme.surface_container_highest.with_alpha(0.5);

    // Root container using grid layout: sidebar | main content | scrollbar
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: vec![
                    bevy::ui::RepeatedGridTrack::px(1, 220.0),  // Sidebar
                    bevy::ui::RepeatedGridTrack::flex(1, 1.0),  // Main content
                    bevy::ui::RepeatedGridTrack::px(1, 12.0),   // Scrollbar
                ],
                grid_template_rows: vec![bevy::ui::RepeatedGridTrack::flex(1, 1.0)],
                overflow: Overflow::clip(),  // Clip any content that overflows the grid
                ..default()
            },
            BackgroundColor(Color::NONE), // Transparent root to see 3D
        ))
        .with_children(|grid| {
            let font_for_content = icon_font_handle.clone();
            let scroll_area_theme = theme.clone();
            let current_section = selected.current;
            
            // ========================================
            // SIDEBAR (Left Column)
            // ========================================
            grid.spawn((
                Node {
                    grid_column: bevy::ui::GridPlacement::start(1),
                    grid_row: bevy::ui::GridPlacement::start(1),
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),  // Constrain max height to grid cell
                    overflow: Overflow::clip(),  // Clip content that overflows sidebar column
                    ..default()
                },
                BackgroundColor(sidebar_bg),
            )).with_children(|sidebar| {
                // Sidebar header
                sidebar.spawn((
                    Node {
                        padding: UiRect::all(Val::Px(16.0)),
                        border: UiRect::bottom(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(theme.outline_variant),
                )).with_children(|header| {
                    header.spawn((
                        Text::new("MD3 Components"),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(theme.primary),
                    ));
                });
                
                // Navigation list container with scrollbar
                sidebar.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_grow: 1.0,
                    flex_shrink: 1.0,  // Allow shrinking
                    flex_basis: Val::Px(0.0),  // Start from 0 and grow to fill - prevents overflow
                    min_height: Val::Px(0.0),  // Allow shrinking below content size
                    width: Val::Percent(100.0),
                    overflow: Overflow::clip_y(),  // Clip content to prevent overflow
                    ..default()
                }).with_children(|nav_container| {
                    // Navigation list using MaterialList component
                    let nav_scroll_id = nav_container.spawn((
                        SidebarScrollArea,
                        ScrollableList,
                        ScrollPosition::default(),
                        MaterialList::new(),
                        Node {
                            flex_direction: FlexDirection::Column,
                            flex_grow: 1.0,
                            flex_shrink: 1.0,  // Allow shrinking
                            flex_basis: Val::Px(0.0),  // Start from 0 and grow - prevents overflow
                            min_height: Val::Px(0.0),  // Allow shrinking below content size
                            padding: UiRect::all(Val::Px(8.0)),
                            overflow: Overflow::scroll_y(),
                            ..default()
                        },
                    )).with_children(|list| {
                        for section in ComponentSection::all() {
                            let is_selected = *section == current_section;
                            spawn_nav_item(list, &theme, *section, is_selected);
                        }
                    }).id();
                    
                    // Sidebar scrollbar track
                    nav_container.spawn((
                        SidebarScrollTrack,
                        TestId("sidebar_scroll_track".to_string()),
                        Visibility::Inherited,  // Added for dynamic visibility control
                        Node {
                            width: Val::Px(8.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_highest.with_alpha(0.3)),
                    )).with_children(|track| {
                        track.spawn((
                            SidebarScrollThumb { target: nav_scroll_id },
                            TestId("sidebar_scroll_thumb".to_string()),
                            Visibility::Inherited,  // Added for dynamic visibility control
                            Button,
                            Interaction::None,
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(6.0),
                                height: Val::Px(60.0),
                                left: Val::Px(1.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            BackgroundColor(theme.primary.with_alpha(0.5)),
                            BorderRadius::all(Val::Px(3.0)),
                        ));
                    });
                });
            });
            
            // ========================================
            // MAIN CONTENT AREA (Middle Column)
            // ========================================
            let scroll_area_id = grid
                .spawn((
                    ScrollableRoot,
                    ScrollPosition::default(),
                    Node {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        grid_column: bevy::ui::GridPlacement::start(2),
                        grid_row: bevy::ui::GridPlacement::start(1),
                        ..default()
                    },
                    BackgroundColor(ui_bg),
                ))
                .with_children(|scroll_area| {
                    // Content container with DetailContent marker
                    scroll_area.spawn((
                        DetailContent,
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(24.0),
                            padding: UiRect::all(Val::Px(32.0)),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                    )).with_children(|content| {
                        // Spawn the initially selected section
                        spawn_section_content(content, &scroll_area_theme, font_for_content.clone(), current_section);
                    });
                })
                .id();
            
            // ========================================
            // SCROLLBAR (Right Column)
            // ========================================
            grid.spawn((
                ScrollbarTrack,
                TestId("main_scroll_track".to_string()),
                Visibility::Inherited,  // Added for dynamic visibility control
                Node {
                    grid_column: bevy::ui::GridPlacement::start(3),
                    grid_row: bevy::ui::GridPlacement::start(1),
                    width: Val::Px(12.0),
                    height: Val::Percent(100.0),
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
                    TestId("main_scroll_thumb".to_string()),
                    Visibility::Inherited,  // Added for dynamic visibility control
                    Button,
                    Interaction::None,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(8.0),
                        height: Val::Px(100.0),
                        top: Val::Px(2.0),
                        left: Val::Px(2.0),
                        ..default()
                    },
                    BackgroundColor(theme.primary.with_alpha(0.6)),
                    BorderRadius::all(Val::Px(4.0)),
                ));
            });
        });
    
    // Snackbar Host - positioned at bottom of screen
    commands.spawn(SnackbarHostBuilder::build());
}

/// Spawn a navigation item in the sidebar using MaterialListItem
fn spawn_nav_item(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    is_selected: bool,
) {
    // Create list item with proper selected state
    let item = MaterialListItem::new(section.display_name()).selected(is_selected);
    let text_color = item.headline_color(theme);
    let bg_color = item.background_color(theme);
    
    // Create test ID from section name (e.g., "nav_buttons", "nav_sliders")
    let test_id = format!("nav_{}", section.display_name().to_lowercase().replace(" ", "_"));
    
    // Spawn with MaterialListItem + NavItem marker + TestId
    parent.spawn((
        NavItem(section),
        TestId::new(test_id),
        item,
        Button,
        Interaction::None,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0), // Slightly smaller for navigation
            padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg_color),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|item_container| {
        // Item content - headline text with ListItemHeadline marker for automatic color updates
        item_container.spawn((
            ListItemHeadline,
            Text::new(section.display_name()),
            TextFont { font_size: 14.0, ..default() },
            TextColor(text_color),
        ));
    });
}

/// Spawn the content for a specific section
fn spawn_section_content(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
    section: ComponentSection,
) {
    // Section title header
    parent.spawn((
        Text::new(section.display_name()),
        TextFont { font_size: 32.0, ..default() },
        TextColor(theme.on_surface),
        Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
    ));
    
    // Call the appropriate section spawner
    match section {
        ComponentSection::Buttons => spawn_buttons_section(parent, theme),
        ComponentSection::Checkboxes => spawn_checkboxes_section(parent, theme, Some(icon_font)),
        ComponentSection::Switches => spawn_switches_section(parent, theme),
        ComponentSection::RadioButtons => spawn_radios_section(parent, theme),
        ComponentSection::Chips => spawn_chips_section(parent, theme, icon_font),
        ComponentSection::FAB => spawn_fab_section(parent, theme, icon_font),
        ComponentSection::Badges => spawn_badges_section(parent, theme, icon_font),
        ComponentSection::Progress => spawn_progress_section(parent, theme),
        ComponentSection::Cards => spawn_cards_section(parent, theme),
        ComponentSection::Dividers => spawn_dividers_section(parent, theme),
        ComponentSection::Lists => spawn_list_section(parent, theme, icon_font),
        ComponentSection::Icons => spawn_icons_section(parent, theme, icon_font),
        ComponentSection::IconButtons => spawn_icon_buttons_section(parent, theme, icon_font),
        ComponentSection::Sliders => spawn_sliders_section(parent, theme),
        ComponentSection::TextFields => spawn_text_fields_section(parent, theme),
        ComponentSection::Dialogs => spawn_dialogs_section(parent, theme),
        ComponentSection::Menus => spawn_menus_section(parent, theme, icon_font),
        ComponentSection::Tabs => spawn_tabs_section(parent, theme),
        ComponentSection::Select => spawn_select_section(parent, theme, icon_font),
        ComponentSection::Snackbar => spawn_snackbar_section(parent, theme),
        ComponentSection::Tooltips => spawn_tooltip_section(parent, theme, icon_font),
        ComponentSection::AppBar => spawn_app_bar_section(parent, theme, icon_font),
        ComponentSection::ThemeColors => spawn_theme_section(parent, theme),
    }
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

/// Marker component for sidebar scroll area
#[derive(Component)]
struct SidebarScrollArea;

/// Marker component for sidebar scrollbar track
#[derive(Component)]
struct SidebarScrollTrack;

/// Sidebar scrollbar thumb component with reference to its scroll target
#[derive(Component)]
struct SidebarScrollThumb {
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

/// Resource to track sidebar scrollbar drag state (separate from main scrollbar)
#[derive(Resource, Default)]
struct SidebarScrollbarDragState {
    /// The thumb being dragged
    dragging_thumb: Option<Entity>,
    /// Y position when drag started
    start_cursor_y: f32,
    /// Scroll position when drag started
    start_scroll_y: f32,
}

/// System to update scrollbar thumb size and position based on scroll state
/// Following Bevy's official scrollbar implementation from bevy_ui_widgets/src/scrollbar.rs
/// Also handles visibility - hides scrollbar when content fits (using Display::None to remove from layout)
fn update_scrollbar_thumb(
    scroll_query: Query<(&ScrollPosition, &ComputedNode), With<ScrollableRoot>>,
    mut track_query: Query<(&ComputedNode, &mut Node), (With<ScrollbarTrack>, Without<ScrollbarThumb>)>,
    mut thumb_query: Query<(&ScrollbarThumb, &mut Node), (With<ScrollbarThumb>, Without<ScrollbarTrack>)>,
) {
    const MIN_THUMB_SIZE: f32 = 30.0;
    const TRACK_INSET: f32 = 2.0;
    
    for (thumb, mut thumb_node) in thumb_query.iter_mut() {
        let Ok((scroll_pos, scroll_computed)) = scroll_query.get(thumb.target) else { 
            continue 
        };
        let Ok((track_computed, mut track_node)) = track_query.single_mut() else { 
            continue 
        };
        
        // Get values in logical pixels (matching Bevy's approach)
        let scale = scroll_computed.inverse_scale_factor();
        let visible_size = scroll_computed.size().y * scale;
        let content_size = scroll_computed.content_size().y * scale;
        
        // Hide scrollbar if content fits within visible area
        // Use Display::None to completely remove from layout (not just invisible)
        let needs_scroll = content_size > visible_size + 1.0; // +1 tolerance for rounding
        if needs_scroll {
            track_node.display = Display::Flex;
        } else {
            track_node.display = Display::None;
            continue;
        }
        
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

/// System to handle sidebar scrollbar thumb dragging
fn sidebar_scrollbar_thumb_drag_system(
    mut drag_state: ResMut<SidebarScrollbarDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    thumb_query: Query<(Entity, &SidebarScrollThumb, &Interaction)>,
    track_query: Query<&ComputedNode, With<SidebarScrollTrack>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode), With<SidebarScrollArea>>,
) {
    const TRACK_INSET: f32 = 1.0;
    
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

/// System to update sidebar scrollbar thumb size and position
/// Also handles visibility - hides scrollbar when content fits (using Display::None to remove from layout)
fn update_sidebar_scrollbar(
    scroll_query: Query<(&ScrollPosition, &ComputedNode), With<SidebarScrollArea>>,
    mut track_query: Query<(&ComputedNode, &mut Node), (With<SidebarScrollTrack>, Without<SidebarScrollThumb>)>,
    mut thumb_query: Query<(&SidebarScrollThumb, &mut Node), (With<SidebarScrollThumb>, Without<SidebarScrollTrack>)>,
) {
    const MIN_THUMB_SIZE: f32 = 30.0;
    const TRACK_INSET: f32 = 1.0;
    
    for (thumb, mut thumb_node) in thumb_query.iter_mut() {
        let Ok((scroll_pos, scroll_computed)) = scroll_query.get(thumb.target) else { 
            continue 
        };
        let Ok((track_computed, mut track_node)) = track_query.single_mut() else { 
            continue 
        };
        
        let scale = scroll_computed.inverse_scale_factor();
        let visible_size = scroll_computed.size().y * scale;
        let content_size = scroll_computed.content_size().y * scale;
        
        // Hide scrollbar if content fits within visible area
        // Use Display::None to completely remove from layout (not just invisible)
        let needs_scroll = content_size > visible_size + 1.0; // +1 tolerance for rounding
        if needs_scroll {
            track_node.display = Display::Flex;
        } else {
            track_node.display = Display::None;
            continue;
        }
        
        let track_length = track_computed.size().y * track_computed.inverse_scale_factor();
        
        let usable_track = track_length - (TRACK_INSET * 2.0);
        
        let thumb_size = if content_size > visible_size {
            (usable_track * visible_size / content_size)
                .max(MIN_THUMB_SIZE)
                .min(usable_track)
        } else {
            usable_track
        };
        
        let mut offset = scroll_pos.y;
        let thumb_pos = if content_size > visible_size {
            let max_offset = content_size - visible_size;
            offset = offset.clamp(0.0, max_offset);
            offset * (usable_track - thumb_size) / (content_size - visible_size)
        } else {
            0.0
        };
        
        thumb_node.left = Val::Px(TRACK_INSET);
        thumb_node.right = Val::Px(TRACK_INSET);
        thumb_node.top = Val::Px(TRACK_INSET + thumb_pos);
        thumb_node.height = Val::Px(thumb_size);
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
    let elevation = button.elevation();

    parent
        .spawn((
            button,
            Button, // This is key - Bevy's Button component enables interaction
            Interaction::None, // Ensure interaction is initialized
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
            elevation.to_box_shadow(), // Add shadow for elevated buttons
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
    static CHECKBOX_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let checkbox_index = CHECKBOX_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let test_id = format!("checkbox_{}", checkbox_index);
    
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
                TestId::new(test_id.clone()),
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
    static SWITCH_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let switch_index = SWITCH_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let test_id = format!("switch_{}", switch_index);
    
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
                TestId::new(test_id.clone()),
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
    static RADIO_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let radio_index = RADIO_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let test_id = format!("radio_{}", radio_index);
    
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
                TestId::new(test_id.clone()),
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

fn handle_button_clicks(
    mut events: MessageReader<ButtonClickEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in events.read() {
        info!("🔘 Button clicked: {:?}", event.entity);
        telemetry.log_event(&format!("Button clicked: {:?}", event.entity));
    }
}

fn handle_checkbox_changes(
    mut events: MessageReader<CheckboxChangeEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in events.read() {
        info!("☑️ Checkbox changed: {:?} -> {:?}", event.entity, event.state);
        telemetry.log_event(&format!("Checkbox toggled: {:?}", event.state));
    }
}

fn handle_switch_changes(
    mut events: MessageReader<SwitchChangeEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in events.read() {
        info!("🔀 Switch changed: {:?} -> {}", event.entity, event.selected);
        telemetry.log_event(&format!("Switch toggled: {}", event.selected));
    }
}

fn handle_radio_changes(
    mut events: MessageReader<RadioChangeEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in events.read() {
        info!("🔘 Radio changed: {:?} -> {}", event.entity, event.selected);
        telemetry.log_event(&format!("Radio selected: {}", event.selected));
    }
}

fn handle_fab_clicks(
    mut fab_query: Query<(&Interaction, &mut BackgroundColor, &FabButton), Changed<Interaction>>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, mut bg, _fab) in fab_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                bg.0 = theme.primary_container.with_alpha(0.8);
                info!("🔘 FAB clicked!");
            }
            Interaction::Hovered => {
                bg.0 = theme.primary_container.with_alpha(0.95);
            }
            Interaction::None => {
                bg.0 = theme.primary_container;
            }
        }
    }
}

fn handle_chip_clicks(
    mut chip_query: Query<(Entity, &Interaction, &mut ChipButton, &mut BackgroundColor), Changed<Interaction>>,
    theme: Res<MaterialTheme>,
) {
    for (_entity, interaction, mut chip, mut bg) in chip_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            chip.selected = !chip.selected;
            info!("🏷️ Chip toggled: selected={}", chip.selected);
        }
        
        // Update visual based on selection state
        bg.0 = if chip.selected {
            theme.secondary_container
        } else if *interaction == Interaction::Hovered {
            theme.surface_container_high
        } else {
            theme.surface_container
        };
    }
}

fn handle_list_item_clicks(
    mut list_items: Query<(Entity, &Interaction, &mut BackgroundColor), (With<SelectableListItem>, Changed<Interaction>)>,
    mut selected: ResMut<SelectedListItem>,
    theme: Res<MaterialTheme>,
) {
    for (entity, interaction, mut bg) in list_items.iter_mut() {
        if *interaction == Interaction::Pressed {
            selected.selected = Some(entity);
            info!("📋 List item selected: {:?}", entity);
        }
        
        let is_selected = selected.selected == Some(entity);
        bg.0 = if is_selected {
            theme.secondary_container.with_alpha(0.5)
        } else if *interaction == Interaction::Hovered {
            theme.surface_container_high
        } else {
            Color::NONE
        };
    }
}

fn handle_slider_drag(
    mut drag_state: ResMut<SliderDragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut sliders: Query<(Entity, &Interaction, &mut SliderThumb, &mut Node)>,
    track_query: Query<(Entity, &Interaction, &ComputedNode), With<SliderTrack>>,
    mut fill_query: Query<(&SliderActiveFill, &mut Node), Without<SliderThumb>>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    
    // Handle track clicks - start drag from current thumb position
    // (jumping to click position requires GlobalTransform which UI nodes don't have by default)
    for (track_entity, track_interaction, track_computed) in track_query.iter() {
        if *track_interaction == Interaction::Pressed && drag_state.dragging.is_none() {
            // Find the thumb for this track
            for (thumb_entity, _thumb_interaction, thumb, _node) in sliders.iter() {
                if thumb.track == track_entity {
                    let track_width = track_computed.size().x;
                    if track_width > 0.0 {
                        drag_state.dragging = Some(thumb_entity);
                        drag_state.start_x = cursor_pos.x;
                        drag_state.start_value = thumb.value;
                    }
                    break;
                }
            }
        }
    }
    
    // Start drag from thumb
    for (entity, interaction, thumb, _node) in sliders.iter() {
        if *interaction == Interaction::Pressed && drag_state.dragging.is_none() {
            drag_state.dragging = Some(entity);
            drag_state.start_x = cursor_pos.x;
            drag_state.start_value = thumb.value;
        }
    }
    
    // Continue drag
    if let Some(dragging) = drag_state.dragging {
        if !mouse_button.pressed(MouseButton::Left) {
            drag_state.dragging = None;
        } else if let Ok((_entity, _interaction, mut thumb, mut node)) = sliders.get_mut(dragging) {
            // Find the track for this specific slider
            if let Some((_, _, track_computed)) = track_query.iter().find(|(e, _, _)| *e == thumb.track) {
                let track_width = track_computed.size().x;
                if track_width > 0.0 {
                    // Calculate delta from start position (not incrementally)
                    let delta = cursor_pos.x - drag_state.start_x;
                    let value_delta = (delta / track_width) * (thumb.max - thumb.min);
                    let mut new_value = (drag_state.start_value + value_delta).clamp(thumb.min, thumb.max);
                    
                    // For discrete sliders, snap to nearest step
                    if let Some(step) = thumb.step {
                        let steps = ((new_value - thumb.min) / step).round();
                        new_value = (thumb.min + steps * step).clamp(thumb.min, thumb.max);
                    }
                    
                    thumb.value = new_value;
                    
                    // Update thumb position
                    let percent = (thumb.value - thumb.min) / (thumb.max - thumb.min);
                    node.left = Val::Percent(percent * 100.0 - 5.0); // Center the thumb
                    
                    // Update active fill
                    for (fill, mut fill_node) in fill_query.iter_mut() {
                        if fill.track == thumb.track {
                            fill_node.width = Val::Percent(percent * 100.0);
                        }
                    }
                }
            }
        }
    }
}

fn update_slider_visuals(
    sliders: Query<&SliderThumb, Changed<SliderThumb>>,
    mut displays: Query<(&SliderValueDisplay, &mut Text)>,
) {
    for thumb in sliders.iter() {
        for (display, mut text) in displays.iter_mut() {
            if display.track == thumb.track {
                **text = format!("{:.0}", thumb.value);
            }
        }
    }
}

fn handle_dialog_buttons(
    show_btn: Query<&Interaction, (With<ShowDialogButton>, Changed<Interaction>)>,
    close_btn: Query<&Interaction, (With<DialogCloseButton>, Changed<Interaction>)>,
    confirm_btn: Query<&Interaction, (With<DialogConfirmButton>, Changed<Interaction>)>,
    mut dialog_state: ResMut<DialogState>,
) {
    for interaction in show_btn.iter() {
        if *interaction == Interaction::Pressed {
            dialog_state.is_open = true;
            dialog_state.result = None;
            info!("📋 Dialog opened");
        }
    }
    
    for interaction in close_btn.iter() {
        if *interaction == Interaction::Pressed {
            dialog_state.is_open = false;
            dialog_state.result = Some("Cancelled".to_string());
            info!("📋 Dialog cancelled");
        }
    }
    
    for interaction in confirm_btn.iter() {
        if *interaction == Interaction::Pressed {
            dialog_state.is_open = false;
            dialog_state.result = Some("Confirmed".to_string());
            info!("📋 Dialog confirmed");
        }
    }
}

fn update_dialog_visibility(
    dialog_state: Res<DialogState>,
    mut dialogs: Query<&mut Visibility, With<DialogContainer>>,
    mut result_displays: Query<&mut Text, With<DialogResultDisplay>>,
) {
    if !dialog_state.is_changed() {
        return;
    }
    
    for mut visibility in dialogs.iter_mut() {
        *visibility = if dialog_state.is_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
    if let Some(ref result) = dialog_state.result {
        for mut text in result_displays.iter_mut() {
            **text = format!("Result: {}", result);
        }
    }
}

fn handle_menu_toggle(
    triggers: Query<(Entity, &Interaction), (With<MenuTrigger>, Changed<Interaction>)>,
    mut menu_state: ResMut<MenuState>,
) {
    for (entity, interaction) in triggers.iter() {
        if *interaction == Interaction::Pressed {
            if menu_state.open_menu == Some(entity) {
                menu_state.open_menu = None;
            } else {
                menu_state.open_menu = Some(entity);
            }
            info!("📋 Menu toggled: {:?}", menu_state.open_menu);
        }
    }
}

fn update_menu_visibility(
    menu_state: Res<MenuState>,
    mut menus: Query<(&mut Visibility, &MenuDropdown)>,
) {
    if !menu_state.is_changed() {
        return;
    }
    
    for (mut visibility, _dropdown) in menus.iter_mut() {
        // For simplicity, show/hide all menus based on whether any trigger is active
        *visibility = if menu_state.open_menu.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_menu_item_clicks(
    mut menu_items: Query<(&Interaction, &MenuItemMarker, &mut BackgroundColor), Changed<Interaction>>,
    mut selected_text: Query<&mut Text, With<MenuSelectedText>>,
    mut menu_state: ResMut<MenuState>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, item, mut bg) in menu_items.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Update the selected text display
                for mut text in selected_text.iter_mut() {
                    **text = item.label.clone();
                }
                // Close the menu
                menu_state.open_menu = None;
                info!("📋 Menu item selected: {}", item.label);
            }
            Interaction::Hovered => {
                bg.0 = theme.surface_container_highest;
            }
            Interaction::None => {
                bg.0 = Color::NONE;
            }
        }
    }
}

/// Handle keyboard shortcuts for menu actions (Ctrl+X, Ctrl+C, Ctrl+V)
fn handle_menu_keyboard_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut snackbar_events: MessageWriter<ShowSnackbar>,
) {
    let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    
    if ctrl {
        if keyboard.just_pressed(KeyCode::KeyX) {
            snackbar_events.write(ShowSnackbar::message("✂️ Cut (Ctrl+X)").duration(2.0));
            info!("⌨️ Keyboard shortcut: Cut (Ctrl+X)");
        }
        if keyboard.just_pressed(KeyCode::KeyC) {
            snackbar_events.write(ShowSnackbar::message("📋 Copy (Ctrl+C)").duration(2.0));
            info!("⌨️ Keyboard shortcut: Copy (Ctrl+C)");
        }
        if keyboard.just_pressed(KeyCode::KeyV) {
            snackbar_events.write(ShowSnackbar::message("📄 Paste (Ctrl+V)").duration(2.0));
            info!("⌨️ Keyboard shortcut: Paste (Ctrl+V)");
        }
    }
}

/// Handle select dropdown toggle
fn handle_select_toggle(
    triggers: Query<(Entity, &Interaction), (With<SelectTrigger>, Changed<Interaction>)>,
    mut select_state: ResMut<SelectState>,
) {
    for (entity, interaction) in triggers.iter() {
        if *interaction == Interaction::Pressed {
            if select_state.open_select == Some(entity) {
                select_state.open_select = None;
            } else {
                select_state.open_select = Some(entity);
            }
            info!("🔽 Select toggled: {:?}", select_state.open_select);
        }
    }
}

/// Update select dropdown visibility
fn update_select_visibility(
    select_state: Res<SelectState>,
    triggers: Query<(Entity, &ChildOf), With<SelectTrigger>>,
    mut dropdowns: Query<(&ChildOf, &mut Visibility), With<SelectDropdown>>,
) {
    if !select_state.is_changed() {
        return;
    }
    
    for (dropdown_child_of, mut visibility) in dropdowns.iter_mut() {
        // The dropdown and trigger are siblings - both children of the same container
        let dropdown_parent = dropdown_child_of.parent();
        
        // Find the trigger that shares this parent
        let matching_trigger = triggers.iter()
            .find(|(_, trigger_child_of)| trigger_child_of.parent() == dropdown_parent)
            .map(|(entity, _)| entity);
        
        // Show dropdown if its sibling trigger is the open one
        *visibility = if select_state.open_select == matching_trigger {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Handle select option clicks
fn handle_select_option_clicks(
    mut options: Query<(&Interaction, &SelectOption, &mut BackgroundColor, &ChildOf), Changed<Interaction>>,
    dropdown_query: Query<&ChildOf, With<SelectDropdown>>,
    mut triggers: Query<(Entity, &mut SelectTrigger, &ChildOf, &Children)>,
    mut display_texts: Query<&mut Text, With<SelectDisplayText>>,
    mut select_state: ResMut<SelectState>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, option, mut bg, option_child_of) in options.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Option's parent is the dropdown
                let dropdown_entity = option_child_of.parent();
                
                // Get the dropdown's parent (the container)
                if let Ok(dropdown_child_of) = dropdown_query.get(dropdown_entity) {
                    let container_entity = dropdown_child_of.parent();
                    
                    // Find the trigger that's a sibling (shares the same container parent)
                    for (trigger_entity, mut trigger, trigger_child_of, trigger_children) in triggers.iter_mut() {
                        if trigger_child_of.parent() == container_entity {
                            trigger.selected_index = option.index;
                            
                            // Update display text within THIS trigger's children
                            for child in trigger_children.iter() {
                                if let Ok(mut text) = display_texts.get_mut(child) {
                                    **text = option.label.clone();
                                }
                            }
                            
                            // Close the dropdown
                            if select_state.open_select == Some(trigger_entity) {
                                select_state.open_select = None;
                            }
                            break;
                        }
                    }
                }
                info!("🔽 Select option chosen: {}", option.label);
            }
            Interaction::Hovered => {
                bg.0 = theme.surface_container_highest;
            }
            Interaction::None => {
                bg.0 = Color::NONE;
            }
        }
    }
}

/// Handle tab button clicks
fn handle_tab_clicks(
    tabs: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut tab_state: ResMut<TabState>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (interaction, tab_btn) in tabs.iter() {
        if *interaction == Interaction::Pressed {
            tab_state.selected_tab = tab_btn.index;
            info!("📑 Tab {} selected", tab_btn.index + 1);
            telemetry.log_event(&format!("Tab {} selected", tab_btn.index + 1));
        }
    }
}

/// Update tab button visuals based on selection
fn update_tab_visuals(
    tab_state: Res<TabState>,
    mut tabs: Query<(&TabButton, &mut BorderColor, &mut Node, &Children)>,
    mut tab_texts: Query<&mut TextColor>,
    theme: Res<MaterialTheme>,
) {
    if !tab_state.is_changed() {
        return;
    }
    
    for (tab_btn, mut border, mut node, children) in tabs.iter_mut() {
        let is_selected = tab_btn.index == tab_state.selected_tab;
        *border = BorderColor::all(if is_selected { theme.primary } else { Color::NONE });
        // Update border width for the indicator line
        node.border = UiRect::bottom(Val::Px(if is_selected { 3.0 } else { 0.0 }));
        
        for child in children.iter() {
            if let Ok(mut text_color) = tab_texts.get_mut(child) {
                text_color.0 = if is_selected { theme.primary } else { theme.on_surface_variant };
            }
        }
    }
}

/// Update tab content visibility based on selection
fn update_tab_content_visibility(
    tab_state: Res<TabState>,
    mut contents: Query<(&TabContent, &mut Visibility)>,
) {
    if !tab_state.is_changed() {
        return;
    }
    
    for (content, mut visibility) in contents.iter_mut() {
        *visibility = if content.index == tab_state.selected_tab {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

/// Handle list item selection (single or multi-select)
fn handle_list_selection(
    mut items: Query<(Entity, &Interaction, &mut BackgroundColor, Option<&TestId>), (With<SelectableListItem>, Changed<Interaction>)>,
    mut selection_state: ResMut<ListSelectionState>,
    mut telemetry: ResMut<ComponentTelemetry>,
    theme: Res<MaterialTheme>,
) {
    for (entity, interaction, mut bg, test_id) in items.iter_mut() {
        if *interaction == Interaction::Pressed {
            let item_name = test_id.map(|t| t.0.as_str()).unwrap_or("unknown");
            match selection_state.mode {
                ListSelectionMode::Single => {
                    // Clear previous selection
                    selection_state.selected.clear();
                    selection_state.selected.push(entity);
                    telemetry.log_event(&format!("List item selected: {}", item_name));
                }
                ListSelectionMode::Multi => {
                    // Toggle selection
                    if let Some(pos) = selection_state.selected.iter().position(|e| *e == entity) {
                        selection_state.selected.remove(pos);
                        telemetry.log_event(&format!("List item deselected: {}", item_name));
                    } else {
                        selection_state.selected.push(entity);
                        telemetry.log_event(&format!("List item selected: {}", item_name));
                    }
                }
            }
            info!("📋 List selection: {:?}", selection_state.selected.len());
        }
        
        // Update visual - use correct colors for both selection and interaction state
        let is_selected = selection_state.selected.contains(&entity);
        bg.0 = match (*interaction, is_selected) {
            // Selected items: primary_container with state layer for hover/press
            (Interaction::Pressed, true) => blend_state_layer(theme.primary_container, theme.on_primary_container, 0.12),
            (Interaction::Hovered, true) => blend_state_layer(theme.primary_container, theme.on_primary_container, 0.08),
            (Interaction::None, true) => theme.primary_container,
            // Unselected items: transparent with state layer for hover/press
            (Interaction::Pressed, false) => theme.surface_container_highest,
            (Interaction::Hovered, false) => theme.surface_container_high,
            (Interaction::None, false) => Color::NONE,
        };
    }
}

/// Update all list item visuals when selection state changes
fn update_list_item_visuals(
    mut items: Query<(Entity, &Interaction, &mut BackgroundColor), With<SelectableListItem>>,
    selection_state: Res<ListSelectionState>,
    theme: Res<MaterialTheme>,
) {
    if !selection_state.is_changed() {
        return;
    }
    
    for (entity, interaction, mut bg) in items.iter_mut() {
        let is_selected = selection_state.selected.contains(&entity);
        
        // Apply correct color based on both selection and interaction state
        bg.0 = match (*interaction, is_selected) {
            // Selected items: primary_container with state layer for hover
            (Interaction::Pressed, true) => blend_state_layer(theme.primary_container, theme.on_primary_container, 0.12),
            (Interaction::Hovered, true) => blend_state_layer(theme.primary_container, theme.on_primary_container, 0.08),
            (Interaction::None, true) => theme.primary_container,
            // Unselected items: transparent with state layer for hover
            (Interaction::Pressed, false) => theme.surface_container_highest,
            (Interaction::Hovered, false) => theme.surface_container_high,
            (Interaction::None, false) => Color::NONE,
        };
    }
}

/// Update list mode button visuals when selection mode changes
fn update_list_mode_button_visuals(
    mut options: Query<(&ListSelectionModeOption, &Interaction, &mut BackgroundColor, &mut BorderColor)>,
    selection_state: Res<ListSelectionState>,
    theme: Res<MaterialTheme>,
) {
    if !selection_state.is_changed() {
        return;
    }
    
    for (option, interaction, mut bg, mut border) in options.iter_mut() {
        // Don't override hover state
        if *interaction == Interaction::Hovered {
            continue;
        }
        
        let is_selected = option.0 == selection_state.mode;
        if is_selected {
            bg.0 = theme.secondary_container;
            *border = BorderColor::all(theme.secondary);
        } else {
            bg.0 = theme.surface_container;
            *border = BorderColor::all(theme.outline);
        }
    }
}

/// Handle list mode button hover/press visual feedback
fn handle_list_mode_button_feedback(
    mut options: Query<(&Interaction, &ListSelectionModeOption, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    selection_state: Res<ListSelectionState>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, option, mut bg, mut border) in options.iter_mut() {
        let is_selected = option.0 == selection_state.mode;
        
        match *interaction {
            Interaction::Pressed => {
                // Will be handled by handle_list_selection_mode_options
            }
            Interaction::Hovered => {
                if is_selected {
                    bg.0 = theme.secondary_container;
                } else {
                    bg.0 = theme.surface_container_high;
                }
            }
            Interaction::None => {
                if is_selected {
                    bg.0 = theme.secondary_container;
                    *border = BorderColor::all(theme.secondary);
                } else {
                    bg.0 = theme.surface_container;
                    *border = BorderColor::all(theme.outline);
                }
            }
        }
    }
}

/// Marker for dialog position option buttons
#[derive(Component)]
struct DialogPositionOption(DialogPosition);

/// Handle dialog position option clicks
fn handle_dialog_position_options(
    mut options: Query<(&Interaction, &DialogPositionOption, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
    mut dialog_state: ResMut<DialogState>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, option, mut bg, mut border) in options.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                dialog_state.position = option.0;
                bg.0 = theme.secondary_container;
                *border = BorderColor::all(theme.secondary);
                info!("📋 Dialog position set to: {:?}", option.0);
            }
            Interaction::Hovered => {
                bg.0 = theme.surface_container_high;
            }
            Interaction::None => {
                // Will be updated by visual update system
            }
        }
    }
}

/// Update dialog position button visuals when state changes
fn update_dialog_position_button_visuals(
    mut options: Query<(&DialogPositionOption, &Interaction, &mut BackgroundColor, &mut BorderColor)>,
    dialog_state: Res<DialogState>,
    theme: Res<MaterialTheme>,
) {
    if !dialog_state.is_changed() {
        return;
    }
    
    for (option, interaction, mut bg, mut border) in options.iter_mut() {
        // Don't override hover state
        if *interaction == Interaction::Hovered {
            continue;
        }
        
        let is_selected = option.0 == dialog_state.position;
        if is_selected {
            bg.0 = theme.secondary_container;
            *border = BorderColor::all(theme.secondary);
        } else {
            bg.0 = theme.surface_container;
            *border = BorderColor::all(theme.outline);
        }
    }
}

/// Handle list selection mode option clicks
fn handle_list_selection_mode_options(
    options: Query<(&Interaction, &ListSelectionModeOption), Changed<Interaction>>,
    mut selection_state: ResMut<ListSelectionState>,
) {
    for (interaction, option) in options.iter() {
        if *interaction == Interaction::Pressed {
            selection_state.mode = option.0;
            selection_state.selected.clear(); // Clear selection when mode changes
            info!("📋 List selection mode set to: {:?}", option.0);
        }
    }
}

fn handle_icon_button_clicks(
    mut icon_buttons: Query<(&Interaction, &mut BackgroundColor, &MaterialIconButton), (Changed<Interaction>, With<IconButtonMarker>)>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, mut bg, icon_btn) in icon_buttons.iter_mut() {
        let base_color = icon_btn.background_color(&theme);
        match *interaction {
            Interaction::Pressed => {
                bg.0 = base_color.with_alpha(0.7);
                info!("🔘 Icon button clicked: {}", icon_btn.icon);
            }
            Interaction::Hovered => {
                bg.0 = base_color.with_alpha(0.9);
            }
            Interaction::None => {
                bg.0 = base_color;
            }
        }
    }
}

fn handle_app_bar_button_clicks(
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &AppBarIconButton), Changed<Interaction>>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, mut bg, button) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                bg.0 = theme.on_surface.with_alpha(0.2);
                info!("🔘 App bar button clicked: {}", button.0);
            }
            Interaction::Hovered => {
                bg.0 = theme.on_surface.with_alpha(0.1);
            }
            Interaction::None => {
                // Reset to original (transparent or primary_container for FAB)
                if button.0 == "fab" {
                    bg.0 = theme.primary_container;
                } else {
                    bg.0 = Color::NONE;
                }
            }
        }
    }
}

fn handle_snackbar_trigger(
    triggers: Query<&Interaction, (With<SnackbarTrigger>, Changed<Interaction>)>,
    mut snackbar_events: MessageWriter<ShowSnackbar>,
    options: Res<SnackbarDemoOptions>,
) {
    for interaction in triggers.iter() {
        if *interaction == Interaction::Pressed {
            let mut snackbar = ShowSnackbar::message("This is a snackbar message!")
                .duration(options.duration);
            if options.has_action {
                snackbar = ShowSnackbar::with_action("Message with action", "UNDO")
                    .duration(options.duration);
            }
            snackbar_events.write(snackbar);
            info!("🍫 Snackbar triggered (duration: {}s, action: {})", options.duration, options.has_action);
        }
    }
}

fn handle_text_field_focus(
    mut text_fields: Query<(&Interaction, &mut TextFieldMarker, &mut BorderColor, &Children), Changed<Interaction>>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<TextFieldText>>,
    theme: Res<MaterialTheme>,
) {
    for (interaction, mut marker, mut border, children) in text_fields.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                marker.focused = true;
                *border = BorderColor::all(theme.primary);
                // Update text to show cursor
                for child in children.iter() {
                    if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                        // Show user text with cursor, or just cursor if empty
                        **text = format!("{}|", marker.user_text);
                        text_color.0 = theme.on_surface;
                    }
                }
                info!("📝 Text field focused");
            }
            Interaction::Hovered => {
                if !marker.focused {
                    *border = BorderColor::all(theme.on_surface);
                }
            }
            Interaction::None => {
                if !marker.focused {
                    *border = BorderColor::all(theme.outline);
                }
            }
        }
    }
}

fn handle_text_field_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut text_fields: Query<(&mut TextFieldMarker, &Children)>,
    mut text_query: Query<&mut Text, With<TextFieldText>>,
) {
    // Find focused text field
    let focused_field = text_fields.iter_mut().find(|(marker, _)| marker.focused);
    
    if let Some((mut marker, children)) = focused_field {
        for event in keyboard_events.read() {
            if event.state != ButtonState::Pressed {
                continue;
            }
            
            match &event.logical_key {
                Key::Character(c) => {
                    marker.user_text.push_str(c);
                    info!("⌨️ Text input: {}", marker.user_text);
                }
                Key::Space => {
                    marker.user_text.push(' ');
                }
                Key::Backspace => {
                    marker.user_text.pop();
                }
                Key::Enter => {
                    info!("⏎ Text field submitted: {}", marker.user_text);
                }
                _ => {}
            }
            
            // Update the display text with cursor
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = format!("{}|", marker.user_text);
                }
            }
        }
    } else {
        // Consume events even if no field is focused
        keyboard_events.clear();
    }
}

fn handle_text_field_unfocus(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut text_fields: Query<(&Interaction, &mut TextFieldMarker, &mut BorderColor, &Children)>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<TextFieldText>>,
    theme: Res<MaterialTheme>,
) {
    // When clicking anywhere, unfocus text fields that weren't clicked
    if mouse_button.just_pressed(MouseButton::Left) {
        for (interaction, mut marker, mut border, children) in text_fields.iter_mut() {
            if marker.focused && *interaction != Interaction::Pressed {
                marker.focused = false;
                *border = BorderColor::all(theme.outline);
                
                // Restore placeholder if empty, otherwise show text without cursor
                for child in children.iter() {
                    if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                        if marker.user_text.is_empty() {
                            **text = marker.placeholder.clone();
                            text_color.0 = theme.on_surface_variant;
                        } else {
                            **text = marker.user_text.clone();
                            text_color.0 = theme.on_surface;
                        }
                    }
                }
                
                info!("📝 Text field unfocused");
            }
        }
    }
}

// ============================================================================
// Demo Option Systems
// ============================================================================

fn handle_tooltip_position_options(
    mut options: ResMut<TooltipDemoOptions>,
    buttons: Query<(&Interaction, &TooltipPositionOption), Changed<Interaction>>,
) {
    for (interaction, pos_option) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.position = pos_option.0;
            info!("🔧 Tooltip position changed to: {:?}", pos_option.0);
        }
    }
}

fn handle_tooltip_delay_options(
    mut options: ResMut<TooltipDemoOptions>,
    buttons: Query<(&Interaction, &TooltipDelayOption), Changed<Interaction>>,
) {
    for (interaction, delay_option) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.delay = delay_option.0;
            info!("🔧 Tooltip delay changed to: {}s", delay_option.0);
        }
    }
}

fn update_tooltip_demo_button(
    options: Res<TooltipDemoOptions>,
    mut triggers: Query<&mut TooltipTrigger, With<TooltipDemoButton>>,
) {
    if options.is_changed() {
        for mut trigger in triggers.iter_mut() {
            trigger.position = options.position;
            trigger.delay = options.delay;
        }
    }
}

fn handle_snackbar_duration_options(
    mut options: ResMut<SnackbarDemoOptions>,
    buttons: Query<(&Interaction, &SnackbarDurationOption), Changed<Interaction>>,
) {
    for (interaction, duration_option) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.duration = duration_option.0;
            info!("🔧 Snackbar duration changed to: {}s", duration_option.0);
        }
    }
}

/// Update visual state of snackbar duration buttons based on current selection
fn update_snackbar_duration_button_visuals(
    options: Res<SnackbarDemoOptions>,
    mut buttons: Query<(&SnackbarDurationOption, &mut BackgroundColor, &Children)>,
    mut texts: Query<&mut TextColor>,
    theme: Res<MaterialTheme>,
) {
    if !options.is_changed() {
        return;
    }
    
    for (duration_option, mut bg, children) in buttons.iter_mut() {
        let is_selected = (duration_option.0 - options.duration).abs() < 0.01;
        bg.0 = if is_selected { theme.primary } else { theme.surface_container_high };
        
        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                text_color.0 = if is_selected { theme.on_primary } else { theme.on_surface };
            }
        }
    }
}

/// Update visual state of snackbar action toggle button
fn update_snackbar_action_button_visuals(
    options: Res<SnackbarDemoOptions>,
    mut buttons: Query<(&mut BackgroundColor, &Children), With<SnackbarActionToggle>>,
    mut texts: Query<&mut TextColor>,
    theme: Res<MaterialTheme>,
) {
    if !options.is_changed() {
        return;
    }
    
    for (mut bg, children) in buttons.iter_mut() {
        bg.0 = if options.has_action { theme.primary } else { theme.surface_container_high };
        
        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                text_color.0 = if options.has_action { theme.on_primary } else { theme.on_surface };
            }
        }
    }
}

fn handle_snackbar_action_toggle(
    mut options: ResMut<SnackbarDemoOptions>,
    buttons: Query<&Interaction, (With<SnackbarActionToggle>, Changed<Interaction>)>,
) {
    for interaction in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.has_action = !options.has_action;
            info!("🔧 Snackbar action toggled: {}", options.has_action);
        }
    }
}

fn handle_text_field_blink_speed_options(
    mut options: ResMut<TextFieldDemoOptions>,
    mut blink_timer: ResMut<CursorBlinkTimer>,
    buttons: Query<(&Interaction, &TextFieldBlinkSpeedOption), Changed<Interaction>>,
) {
    for (interaction, speed_option) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.blink_speed = speed_option.0;
            blink_timer.blink_speed = speed_option.0;
            blink_timer.timer = Timer::from_seconds(speed_option.0, TimerMode::Repeating);
            info!("🔧 Text field blink speed changed to: {}s", speed_option.0);
        }
    }
}

fn handle_text_field_cursor_toggle(
    mut options: ResMut<TextFieldDemoOptions>,
    buttons: Query<&Interaction, (With<TextFieldCursorToggle>, Changed<Interaction>)>,
) {
    for interaction in buttons.iter() {
        if *interaction == Interaction::Pressed {
            options.show_cursor = !options.show_cursor;
            info!("🔧 Text field cursor toggled: {}", options.show_cursor);
        }
    }
}

fn update_text_field_option_buttons(
    options: Res<TextFieldDemoOptions>,
    theme: Res<MaterialTheme>,
    mut blink_buttons: Query<(&TextFieldBlinkSpeedOption, &mut BackgroundColor, &Children)>,
    mut toggle_buttons: Query<(&mut BackgroundColor, &Children), (With<TextFieldCursorToggle>, Without<TextFieldBlinkSpeedOption>)>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    if options.is_changed() {
        // Update blink speed button visuals
        for (speed_option, mut bg, children) in blink_buttons.iter_mut() {
            let is_selected = (speed_option.0 - options.blink_speed).abs() < 0.01;
            *bg = BackgroundColor(if is_selected { theme.primary } else { theme.surface_container_high });
            
            for child in children.iter() {
                if let Ok((_, mut text_color)) = texts.get_mut(child) {
                    *text_color = TextColor(if is_selected { theme.on_primary } else { theme.on_surface });
                }
            }
        }
        
        // Update cursor toggle button visuals
        for (mut bg, children) in toggle_buttons.iter_mut() {
            *bg = BackgroundColor(if options.show_cursor { theme.primary } else { theme.surface_container_high });
            
            for child in children.iter() {
                if let Ok((mut text, mut text_color)) = texts.get_mut(child) {
                    **text = if options.show_cursor { "ON".to_string() } else { "OFF".to_string() };
                    *text_color = TextColor(if options.show_cursor { theme.on_primary } else { theme.on_surface });
                }
            }
        }
    }
}

// ============================================================================
// Navigation Systems
// ============================================================================

fn handle_nav_clicks(
    mut selected: ResMut<SelectedSection>,
    mut click_events: MessageReader<ListItemClickEvent>,
    nav_items: Query<&NavItem>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in click_events.read() {
        // Check if this is a navigation item click
        if let Ok(nav_item) = nav_items.get(event.entity) {
            if selected.current != nav_item.0 {
                selected.current = nav_item.0;
                info!("📍 Selected section: {:?}", nav_item.0);
                telemetry.log_event(&format!("Nav selected: {:?}", nav_item.0));
            }
        }
    }
}

fn update_nav_highlights(
    selected: Res<SelectedSection>,
    mut nav_items: Query<(&NavItem, &mut MaterialListItem, &mut BackgroundColor)>,
    theme: Res<MaterialTheme>,
) {
    if selected.is_changed() {
        // Update MaterialListItem selected state and background color
        for (nav_item, mut list_item, mut bg) in nav_items.iter_mut() {
            let is_selected = nav_item.0 == selected.current;
            list_item.selected = is_selected;
            // Update background color based on selection
            bg.0 = if is_selected {
                theme.secondary_container
            } else {
                Color::NONE
            };
        }
    }
}

fn update_detail_content(
    mut commands: Commands,
    selected: Res<SelectedSection>,
    theme: Res<MaterialTheme>,
    icon_font: Res<MaterialIconFont>,
    detail_query: Query<Entity, With<DetailContent>>,
    mut scroll_positions: Query<&mut ScrollPosition, With<ScrollableRoot>>,
) {
    if selected.is_changed() {
        // Despawn existing content and respawn with new section
        for entity in detail_query.iter() {
            // Despawn all children using Bevy 0.17's despawn_children method
            commands.entity(entity).despawn_children();
            
            // Respawn content
            let section = selected.current;
            let theme_clone = theme.clone();
            let icon_font_handle = icon_font.0.clone();
            
            commands.entity(entity).with_children(|parent| {
                // Section title header
                parent.spawn((
                    Text::new(section.display_name()),
                    TextFont { font_size: 32.0, ..default() },
                    TextColor(theme_clone.on_surface),
                    Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                ));
                
                // Call the appropriate section spawner
                match section {
                    ComponentSection::Buttons => spawn_buttons_section(parent, &theme_clone),
                    ComponentSection::Checkboxes => spawn_checkboxes_section(parent, &theme_clone, Some(icon_font_handle.clone())),
                    ComponentSection::Switches => spawn_switches_section(parent, &theme_clone),
                    ComponentSection::RadioButtons => spawn_radios_section(parent, &theme_clone),
                    ComponentSection::Chips => spawn_chips_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::FAB => spawn_fab_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Badges => spawn_badges_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Progress => spawn_progress_section(parent, &theme_clone),
                    ComponentSection::Cards => spawn_cards_section(parent, &theme_clone),
                    ComponentSection::Dividers => spawn_dividers_section(parent, &theme_clone),
                    ComponentSection::Lists => spawn_list_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Icons => spawn_icons_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::IconButtons => spawn_icon_buttons_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Sliders => spawn_sliders_section(parent, &theme_clone),
                    ComponentSection::TextFields => spawn_text_fields_section(parent, &theme_clone),
                    ComponentSection::Dialogs => spawn_dialogs_section(parent, &theme_clone),
                    ComponentSection::Menus => spawn_menus_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Tabs => spawn_tabs_section(parent, &theme_clone),
                    ComponentSection::Select => spawn_select_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::Snackbar => spawn_snackbar_section(parent, &theme_clone),
                    ComponentSection::Tooltips => spawn_tooltip_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::AppBar => spawn_app_bar_section(parent, &theme_clone, icon_font_handle.clone()),
                    ComponentSection::ThemeColors => spawn_theme_section(parent, &theme_clone),
                }
            });
        }
        
        // Reset scroll position when changing sections
        for mut scroll_pos in scroll_positions.iter_mut() {
            *scroll_pos = ScrollPosition::default();
        }
    }
}

fn update_cursor_blink(
    time: Res<Time>,
    mut blink_timer: ResMut<CursorBlinkTimer>,
    options: Res<TextFieldDemoOptions>,
    text_fields: Query<(&TextFieldMarker, &Children)>,
    mut text_query: Query<&mut Text, With<TextFieldText>>,
) {
    blink_timer.timer.tick(time.delta());
    
    if blink_timer.timer.just_finished() {
        blink_timer.visible = !blink_timer.visible;
        
        // Update focused text fields
        for (marker, children) in text_fields.iter() {
            if marker.focused {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        if options.show_cursor && blink_timer.visible {
                            **text = format!("{}|", marker.user_text);
                        } else {
                            **text = format!("{} ", marker.user_text);
                        }
                    }
                }
            }
        }
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

fn spawn_chips_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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

            let font = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_chip(row, theme, "Filter", false, font.clone());
                    spawn_chip(row, theme, "Selected", true, font.clone());
                    spawn_chip(row, theme, "Tag", false, font.clone());
                    spawn_chip(row, theme, "Action", false, font.clone());
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

fn spawn_chip(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, selected: bool, icon_font: Handle<Font>) {
    let bg_color = if selected { theme.secondary_container } else { theme.surface_container };
    let border_color = if selected { theme.secondary_container } else { theme.outline };
    let text_color = if selected { theme.on_secondary_container } else { theme.on_surface_variant };
    
    parent.spawn((
        ChipButton { selected },
        Button,
        Interaction::None,
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(border_color),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|chip| {
        // Show check icon if selected
        if selected {
            chip.spawn((
                Text::new(ICON_CHECK.to_string()),
                TextFont { font: icon_font.clone(), font_size: 18.0, ..default() },
                TextColor(text_color),
            ));
        }
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

fn spawn_fab_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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

            let font = icon_font.clone();
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
                    spawn_fab(row, theme, 40.0, font.clone());
                    // Regular FAB
                    spawn_fab(row, theme, 56.0, font.clone());
                    // Large FAB
                    spawn_fab(row, theme, 96.0, font.clone());
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

fn spawn_fab(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, size: f32, icon_font: Handle<Font>) {
    parent.spawn((
        FabButton,
        Button,
        Interaction::None,
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.primary_container),
        BorderRadius::all(Val::Px(size / 3.5)),
        Elevation::Level3.to_box_shadow(),
    )).with_children(|fab| {
        // Use proper icon character with icon font
        fab.spawn((
            Text::new(ICON_ADD.to_string()),
            TextFont { font: icon_font, font_size: size * 0.45, ..default() },
            TextColor(theme.on_primary_container),
        ));
    });
}

// ============================================================================
// Badges Section
// ============================================================================

fn spawn_badges_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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

            let icon_font_clone = icon_font.clone();
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
                    spawn_badge_example(row, theme, &icon_font_clone, None);
                    // Small count
                    spawn_badge_example(row, theme, &icon_font_clone, Some("3"));
                    // Large count
                    spawn_badge_example(row, theme, &icon_font_clone, Some("99+"));
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

fn spawn_badge_example(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: &Handle<Font>, count: Option<&str>) {
    parent.spawn((
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
        // Notification icon with proper font
        container.spawn((
            Text::new(ICON_NOTIFICATIONS.to_string()),
            TextFont { font: icon_font.clone(), font_size: 24.0, ..default() },
            TextColor(theme.on_surface),
        ));
        
        // Badge
        let (width, text) = match count {
            None => (Val::Px(8.0), String::new()),
            Some(c) => (Val::Auto, c.to_string()),
        };
        
        container.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(4.0),
                right: Val::Px(4.0),
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

/// Marker for list selection mode option buttons
#[derive(Component)]
struct ListSelectionModeOption(ListSelectionMode);

fn spawn_list_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Lists (with Selection)",
                "Scrollable list with single or multi-select - click items to select"
            );

            // Selection mode options
            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                row.spawn((
                    Text::new("Selection Mode:"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme_clone.on_surface),
                    Node { margin: UiRect::right(Val::Px(8.0)), ..default() },
                ));
                spawn_list_mode_option(row, &theme_clone, "Single", ListSelectionMode::Single, true);
                spawn_list_mode_option(row, &theme_clone, "Multi", ListSelectionMode::Multi, false);
            });

            let icon_font_clone = icon_font.clone();
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
                            TestId::new("list_scroll_area"),
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

                            for (i, (headline, supporting)) in items.iter().enumerate() {
                                let icon_for_item = icon_font_clone.clone();
                                list.spawn((
                                    SelectableListItem,
                                    TestId::new(format!("list_item_{}", i)),
                                    ListItemBuilder::new(*headline)
                                        .two_line()
                                        .supporting_text(*supporting)
                                        .build(&theme_clone),
                                ))
                                .with_children(|item| {
                                    // Leading icon with proper font
                                    item.spawn((
                                        Text::new(ICON_EMAIL.to_string()),
                                        TextFont { font: icon_for_item, font_size: 24.0, ..default() },
                                        TextColor(theme_clone.on_surface_variant),
                                        Node { margin: UiRect::right(Val::Px(16.0)), ..default() },
                                    ));
                                    
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
r#"// Scrollable list with selection modes
// Single select clears previous selection
// Multi select allows multiple items to be selected
commands.spawn((
    ListBuilder::new()
        .max_visible_items_variant(4, ListItemVariant::TwoLine)
        .selection_mode(ListSelectionMode::Multi)  // or Single
        .build_scrollable(),
    BackgroundColor(theme.surface_container_low),
)).with_children(|list| {
    for (headline, supporting) in items {
        list.spawn((
            SelectableListItem,
            ListItemBuilder::new(headline)
                .two_line()
                .supporting_text(supporting)
                .build(&theme)
        ));
    }
});"#);
        });
}

fn spawn_list_mode_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    mode: ListSelectionMode,
    is_selected: bool,
) {
    let bg_color = if is_selected { theme.secondary_container } else { theme.surface_container };
    let text_color = if is_selected { theme.on_secondary_container } else { theme.on_surface };
    
    parent.spawn((
        ListSelectionModeOption(mode),
        Button,
        Interaction::None,
        Node {
            padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(if is_selected { theme.secondary } else { theme.outline }),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|btn| {
        btn.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(text_color),
        ));
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
// Additional Interactive State Resources
// ============================================================================

/// Slider drag state
#[derive(Resource, Default)]
struct SliderDragState {
    dragging: Option<Entity>,
    start_x: f32,
    start_value: f32,
    clicked_track: Option<Entity>,
}

/// Tooltip demo options
#[derive(Resource)]
struct TooltipDemoOptions {
    position: TooltipPosition,
    delay: f32,
}

impl Default for TooltipDemoOptions {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Bottom,
            delay: 0.5,
        }
    }
}

/// Snackbar demo options
#[derive(Resource)]
struct SnackbarDemoOptions {
    duration: f32,
    has_action: bool,
}

impl Default for SnackbarDemoOptions {
    fn default() -> Self {
        Self {
            duration: 4.0,
            has_action: false,
        }
    }
}

/// Dialog visibility state
#[derive(Resource, Default)]
struct DialogState {
    is_open: bool,
    result: Option<String>,
    position: DialogPosition,
}

/// Dialog positioning options
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum DialogPosition {
    #[default]
    CenterWindow,
    CenterParent,
    BelowTrigger,
}

/// Tab state for switching tabs
#[derive(Resource)]
struct TabState {
    selected_tab: usize,
}

impl Default for TabState {
    fn default() -> Self {
        Self { selected_tab: 0 }
    }
}

/// Marker for tab buttons with index
#[derive(Component)]
struct TabButton {
    index: usize,
}

/// Marker for tab content panels
#[derive(Component)]
struct TabContent {
    index: usize,
}

/// List selection mode
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
enum ListSelectionMode {
    #[default]
    Single,
    Multi,
}

/// List selection state
#[derive(Resource, Default)]
struct ListSelectionState {
    mode: ListSelectionMode,
    selected: Vec<Entity>,
}

/// Menu visibility state
#[derive(Resource, Default)]
struct MenuState {
    open_menu: Option<Entity>,
}

/// Select dropdown state
#[derive(Resource, Default)]
struct SelectState {
    open_select: Option<Entity>,
}

/// Marker for select container (parent of both trigger and dropdown)
#[derive(Component)]
struct SelectContainer;

/// Marker for select trigger buttons
#[derive(Component)]
struct SelectTrigger {
    #[allow(dead_code)]
    options: Vec<String>,
    selected_index: usize,
}

/// Marker for select dropdown menu
#[derive(Component)]
struct SelectDropdown;

/// Marker for select option items
#[derive(Component)]
struct SelectOption {
    index: usize,
    label: String,
}

/// Marker for select's displayed text
#[derive(Component)]
struct SelectDisplayText;

/// Currently selected list item
#[derive(Resource, Default)]
struct SelectedListItem {
    selected: Option<Entity>,
}

/// Marker for interactive slider thumb
#[derive(Component)]
struct SliderThumb {
    min: f32,
    max: f32,
    value: f32,
    track: Entity, // Link to parent track for proper sizing
    step: Option<f32>, // For discrete sliders - snap to nearest step
}

/// Marker for slider value display linked to a specific slider
#[derive(Component)]
struct SliderValueDisplay {
    track: Entity,
}

/// Marker for slider track
#[derive(Component)]
struct SliderTrack;

/// Marker for slider active fill portion
#[derive(Component)]
struct SliderActiveFill {
    track: Entity,
}

/// Marker for FAB buttons
#[derive(Component)]
struct FabButton;

/// Marker for chip buttons
#[derive(Component)]
struct ChipButton {
    selected: bool,
}

/// Marker for selectable list items
#[derive(Component)]
struct SelectableListItem;

/// Marker for interactive text fields
#[derive(Component)]
struct TextFieldMarker {
    focused: bool,
    /// The actual user input (not placeholder)
    user_text: String,
    /// Placeholder text to show when empty and unfocused
    placeholder: String,
}

impl TextFieldMarker {
    fn new(placeholder: &str) -> Self {
        Self {
            focused: false,
            user_text: String::new(),
            placeholder: placeholder.to_string(),
        }
    }
}

/// Marker for text inside text fields
#[derive(Component)]
struct TextFieldText;

/// Resource for cursor blinking
#[derive(Resource)]
struct CursorBlinkTimer {
    timer: Timer,
    visible: bool,
    blink_speed: f32,
}

impl Default for CursorBlinkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.53, TimerMode::Repeating),
            visible: true,
            blink_speed: 0.53,
        }
    }
}

/// Text field demo options
#[derive(Resource)]
struct TextFieldDemoOptions {
    blink_speed: f32,
    show_cursor: bool,
}

impl Default for TextFieldDemoOptions {
    fn default() -> Self {
        Self {
            blink_speed: 0.53,
            show_cursor: true,
        }
    }
}

/// Marker for text field blink speed option buttons
#[derive(Component)]
struct TextFieldBlinkSpeedOption(f32);

/// Marker for text field cursor toggle
#[derive(Component)]
struct TextFieldCursorToggle;

/// Marker for dialog container
#[derive(Component)]
struct DialogContainer;

/// Marker for dialog show button
#[derive(Component)]
struct ShowDialogButton;

/// Marker for dialog close button
#[derive(Component)]
struct DialogCloseButton;

/// Marker for dialog confirm button
#[derive(Component)]
struct DialogConfirmButton;

/// Marker for dialog result display
#[derive(Component)]
struct DialogResultDisplay;

/// Marker for menu trigger button
#[derive(Component)]
struct MenuTrigger;

/// Marker for menu dropdown
#[derive(Component)]
struct MenuDropdown;

/// Marker for menu item with its label
#[derive(Component)]
struct MenuItemMarker {
    label: String,
}

/// Marker for the text that shows the selected menu item
#[derive(Component)]
struct MenuSelectedText;

/// Marker for snackbar trigger button
#[derive(Component)]
struct SnackbarTrigger;

/// Marker for interactive icon buttons
#[derive(Component)]
struct IconButtonMarker;

/// Marker for app bar icon buttons
#[derive(Component)]
struct AppBarIconButton(String);

/// Marker for tooltip demo button (updates when options change)
#[derive(Component)]
struct TooltipDemoButton;

/// Marker for tooltip position option buttons
#[derive(Component)]
struct TooltipPositionOption(TooltipPosition);

/// Marker for tooltip delay option buttons  
#[derive(Component)]
struct TooltipDelayOption(f32);

/// Marker for snackbar duration option buttons
#[derive(Component)]
struct SnackbarDurationOption(f32);

/// Marker for snackbar action toggle
#[derive(Component)]
struct SnackbarActionToggle;

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
// Icon Buttons Section
// ============================================================================

fn spawn_icon_buttons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Icon Buttons",
                "Icon-only buttons for actions - Standard, Filled, Tonal, and Outlined variants"
            );

            let icon_font_clone = icon_font.clone();
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Standard icon button
                    spawn_icon_button_demo(row, theme, &icon_font_clone, "favorite", IconButtonVariant::Standard, "Standard");
                    // Filled icon button
                    spawn_icon_button_demo(row, theme, &icon_font_clone, "add", IconButtonVariant::Filled, "Filled");
                    // Filled Tonal icon button
                    spawn_icon_button_demo(row, theme, &icon_font_clone, "edit", IconButtonVariant::FilledTonal, "Tonal");
                    // Outlined icon button
                    spawn_icon_button_demo(row, theme, &icon_font_clone, "delete", IconButtonVariant::Outlined, "Outlined");
                });

            spawn_code_block(section, theme,
r#"// Create an icon button
let icon_btn = MaterialIconButton::new("favorite")
    .with_variant(IconButtonVariant::Filled);

commands.spawn((
    icon_btn,
    Button,
    RippleHost::new(),
    Node {
        width: Val::Px(40.0),
        height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(theme.primary),
    BorderRadius::all(Val::Px(20.0)),
));"#);
        });
}

fn spawn_icon_button_demo(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: &Handle<Font>,
    icon_name: &str,
    variant: IconButtonVariant,
    label: &str,
) {
    let icon_btn = MaterialIconButton::new(icon_name).with_variant(variant);
    let bg_color = icon_btn.background_color(theme);
    let icon_color = icon_btn.icon_color(theme);
    let has_border = variant == IconButtonVariant::Outlined;
    
    // Map icon names to actual icon characters
    let icon_char = match icon_name {
        "favorite" => ICON_FAVORITE,
        "add" => ICON_ADD,
        "edit" => ICON_EDIT,
        "delete" => ICON_DELETE,
        _ => ICON_STAR,
    };
    
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(4.0),
        ..default()
    }).with_children(|col| {
        let icon_font_btn = icon_font.clone();
        col.spawn((
            IconButtonMarker,
            icon_btn,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(if has_border { theme.outline } else { Color::NONE }),
            BorderRadius::all(Val::Px(20.0)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new(icon_char.to_string()),
                TextFont { font: icon_font_btn, font_size: 24.0, ..default() },
                TextColor(icon_color),
            ));
        });
        
        col.spawn((
            Text::new(label),
            TextFont { font_size: 11.0, ..default() },
            TextColor(theme.on_surface_variant),
        ));
    });
}

// ============================================================================
// Sliders Section
// ============================================================================

fn spawn_sliders_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Sliders",
                "Select values from a range - Continuous and Discrete with optional ticks"
            );

            // Continuous slider demo
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Continuous Slider"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                // Slider track visualization
                spawn_slider_demo(col, theme, 0.4, false);
                
                col.spawn((
                    Text::new("Discrete Slider with Ticks"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                    Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ));
                
                spawn_slider_demo(col, theme, 0.6, true);
            });

            spawn_code_block(section, theme,
r#"// Create a slider
let slider = MaterialSlider::new(0.0, 100.0)
    .with_value(50.0)
    .with_step(10.0)  // Makes it discrete
    .show_ticks()
    .show_label();

commands.spawn((
    slider,
    Node { width: Val::Px(200.0), height: Val::Px(40.0), ..default() },
));"#);
        });
}

fn spawn_slider_demo(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, value: f32, show_ticks: bool) {
    static SLIDER_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let slider_index = SLIDER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    let initial_value = value * 100.0;
    let theme_clone = theme.clone();
    let show_ticks_val = show_ticks;
    let value_copy = value;
    // Step size for discrete slider (5 ticks = 20% each = step of 20)
    let step = if show_ticks { Some(20.0) } else { None };
    let test_id_thumb = format!("slider_thumb_{}", slider_index);
    let test_id_track = format!("slider_track_{}", slider_index);
    
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(16.0),
        ..default()
    }).with_children(move |row| {
        let test_id_thumb = test_id_thumb.clone();
        let test_id_track = test_id_track.clone();
        // Slider container
        row.spawn(Node {
            width: Val::Px(200.0),
            height: Val::Px(40.0),
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|slider_area| {
            // Track (clickable) - spawn first and get entity
            let track_entity = slider_area.spawn((
                SliderTrack,
                TestId::new(test_id_track.clone()),
                Button,
                Interaction::None,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(theme_clone.surface_container_highest),
                BorderRadius::all(Val::Px(2.0)),
            )).id();
            
            // Active fill portion - OUTSIDE track, positioned absolutely
            slider_area.spawn((
                SliderActiveFill { track: track_entity },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(18.0), // Center vertically in 40px area
                    width: Val::Percent(value_copy * 100.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(theme_clone.primary),
                BorderRadius::all(Val::Px(2.0)),
            ));
            
            // Thumb (draggable) - linked to track with optional step for discrete mode
            slider_area.spawn((
                SliderThumb { min: 0.0, max: 100.0, value: initial_value, track: track_entity, step },
                TestId::new(test_id_thumb.clone()),
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(value_copy * 100.0 - 5.0),
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(theme_clone.primary),
                BorderRadius::all(Val::Px(10.0)),
            ));
            
            // Tick marks for discrete
            if show_ticks_val {
                for i in 0..=5 {
                    let pos = i as f32 / 5.0;
                    slider_area.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(pos * 100.0 - 1.0),
                            top: Val::Px(22.0),
                            width: Val::Px(2.0),
                            height: Val::Px(4.0),
                            ..default()
                        },
                        BackgroundColor(if pos <= value_copy { theme_clone.primary } else { theme_clone.outline }),
                    ));
                }
            }
            
            // Store track_entity for value display - spawn it as sibling
            // Value display with track link
            slider_area.spawn((
                SliderValueDisplay { track: track_entity },
                Text::new(format!("{:.0}", initial_value)),
                TextFont { font_size: 14.0, ..default() },
                TextColor(theme_clone.on_surface),
                Node { 
                    position_type: PositionType::Absolute,
                    right: Val::Px(-50.0),
                    min_width: Val::Px(30.0), 
                    ..default() 
                },
            ));
        });
    });
}

// ============================================================================
// Text Fields Section
// ============================================================================

fn spawn_text_fields_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                        row.spawn((
                            Button,
                            Interaction::None,
                            TextFieldBlinkSpeedOption(speed),
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(if is_default { theme.primary } else { theme.surface_container_high }),
                            BorderRadius::all(Val::Px(4.0)),
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(if is_default { theme.on_primary } else { theme.on_surface }),
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
                    
                    row.spawn((
                        Button,
                        Interaction::None,
                        TextFieldCursorToggle,
                        Node {
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(theme.primary),
                        BorderRadius::all(Val::Px(4.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("ON"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(theme.on_primary),
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
        
        // Input container - now interactive with Button component
        let placeholder_text = if has_error { "Invalid input" } else { "Click to focus..." };
        col.spawn((
            TextFieldMarker::new(placeholder_text),
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
                TextFieldText,
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

// ============================================================================
// Dialogs Section
// ============================================================================

fn spawn_dialogs_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Dialogs",
                "Modal windows with positioning options"
            );

            // Position options
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Dialog Position:"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                }).with_children(|row| {
                    spawn_dialog_position_option(row, theme, "Center Window", DialogPosition::CenterWindow, true);
                    spawn_dialog_position_option(row, theme, "Center Parent", DialogPosition::CenterParent, false);
                    spawn_dialog_position_option(row, theme, "Below Trigger", DialogPosition::BelowTrigger, false);
                });
            });

            // Show Dialog button and result display
            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                // Show Dialog button
                row.spawn((
                    ShowDialogButton,
                    Button,
                    Interaction::None,
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(theme.primary),
                    BorderRadius::all(Val::Px(20.0)),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Show Dialog"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_primary),
                    ));
                });
                
                // Result display
                row.spawn((
                    DialogResultDisplay,
                    Text::new("Result: None"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface_variant),
                ));
            });

            // Dialog container (hidden by default)
            section.spawn((
                DialogContainer,
                Visibility::Hidden,
                Node {
                    width: Val::Px(280.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(16.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_high),
                BorderRadius::all(Val::Px(28.0)),
                BoxShadow::from(ShadowStyle {
                    color: Color::BLACK.with_alpha(0.3),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(8.0),
                    spread_radius: Val::Px(0.0),
                    blur_radius: Val::Px(24.0),
                }),
            )).with_children(|dialog| {
                // Title
                dialog.spawn((
                    Text::new("Confirm Action"),
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                // Content
                dialog.spawn((
                    Text::new("Are you sure you want to proceed? This action cannot be undone."),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface_variant),
                ));
                
                // Actions
                dialog.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|actions| {
                    // Cancel button
                    actions.spawn((
                        DialogCloseButton,
                        Button,
                        Interaction::None,
                        Node {
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Cancel"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(theme.primary),
                        ));
                    });
                    
                    // Confirm button
                    actions.spawn((
                        DialogConfirmButton,
                        Button,
                        Interaction::None,
                        Node {
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(theme.primary),
                        BorderRadius::all(Val::Px(20.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Confirm"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(theme.on_primary),
                        ));
                    });
                });
            });

            spawn_code_block(section, theme,
r#"// Create a dialog with positioning
let dialog = MaterialDialog::new()
    .title("Delete Item?")
    .position(DialogPosition::CenterWindow)  // or CenterParent, BelowTrigger
    .open(true);

// Position can be set relative to:
// - CenterWindow: Centered in the application window
// - CenterParent: Centered within parent container
// - BelowTrigger: Positioned below the trigger button"#);
        });
}

fn spawn_dialog_position_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    position: DialogPosition,
    is_selected: bool,
) {
    let bg_color = if is_selected { theme.secondary_container } else { theme.surface_container };
    let text_color = if is_selected { theme.on_secondary_container } else { theme.on_surface };
    
    parent.spawn((
        DialogPositionOption(position),
        Button,
        Interaction::None,
        Node {
            padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor::all(if is_selected { theme.secondary } else { theme.outline }),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|btn| {
        btn.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(text_color),
        ));
    });
}

// ============================================================================
// Menus Section
// ============================================================================

fn spawn_menus_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Menus",
                "Dropdown menus with selectable items"
            );

            let icon_font_clone = icon_font.clone();
            // Menu trigger and dropdown container
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|container| {
                // Menu trigger button
                container.spawn((
                    MenuTrigger,
                    Button,
                    Interaction::None,
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                    BorderRadius::all(Val::Px(8.0)),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(ICON_MORE_VERT.to_string()),
                        TextFont { font: icon_font_clone.clone(), font_size: 20.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                    btn.spawn((
                        MenuSelectedText,
                        Text::new("Options"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                    btn.spawn((
                        Text::new(ICON_EXPAND_MORE.to_string()),
                        TextFont { font: icon_font_clone.clone(), font_size: 20.0, ..default() },
                        TextColor(theme.on_surface),
                    ));
                });
                
                // Menu dropdown (hidden by default)
                container.spawn((
                    MenuDropdown,
                    Visibility::Hidden,
                    Node {
                        width: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::vertical(Val::Px(8.0)),
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                    BorderRadius::all(Val::Px(4.0)),
                    BoxShadow::from(ShadowStyle {
                        color: Color::BLACK.with_alpha(0.2),
                        x_offset: Val::Px(0.0),
                        y_offset: Val::Px(4.0),
                        spread_radius: Val::Px(0.0),
                        blur_radius: Val::Px(8.0),
                    }),
                )).with_children(|menu| {
                    spawn_menu_item(menu, theme, "Cut", "Ctrl+X", false);
                    spawn_menu_item(menu, theme, "Copy", "Ctrl+C", false);
                    spawn_menu_item(menu, theme, "Paste", "Ctrl+V", false);
                    // Divider
                    menu.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(1.0),
                            margin: UiRect::vertical(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(theme.outline_variant),
                    ));
                    spawn_menu_item(menu, theme, "Delete", "", true);
                });
            });

            spawn_code_block(section, theme,
r#"// Create a menu
let menu = MaterialMenu::new()
    .anchor(MenuAnchor::BottomLeft)
    .open();

commands.spawn((
    menu,
    Node { width: Val::Px(200.0), ..default() },
    BackgroundColor(theme.surface_container),
));

// Add menu items
let item = MenuItem::new("Copy")
    .shortcut("Ctrl+C");"#);
        });
}

fn spawn_menu_item(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, shortcut: &str, is_destructive: bool) {
    let text_color = if is_destructive { theme.error } else { theme.on_surface };
    
    parent.spawn((
        MenuItemMarker { label: label.to_string() },
        Button,
        Interaction::None,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(48.0),
            padding: UiRect::horizontal(Val::Px(16.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::NONE),
    )).with_children(|item| {
        item.spawn((
            Text::new(label),
            TextFont { font_size: 14.0, ..default() },
            TextColor(text_color),
        ));
        
        if !shortcut.is_empty() {
            item.spawn((
                Text::new(shortcut),
                TextFont { font_size: 12.0, ..default() },
                TextColor(theme.on_surface_variant),
            ));
        }
    });
}

// ============================================================================
// Tabs Section
// ============================================================================

fn spawn_tabs_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Tabs",
                "Navigate between related content groups - Click tabs to switch content"
            );

            // Primary tabs demo with content
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(Val::Px(8.0)),
                width: Val::Percent(100.0),
                max_width: Val::Px(500.0),
                ..default()
            }).with_children(|col| {
                // Tabs header bar
                col.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        border: UiRect::bottom(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    BorderColor::all(theme.surface_container_highest),
                )).with_children(|tabs| {
                    spawn_tab_button(tabs, theme, "Home", 0, true);
                    spawn_tab_button(tabs, theme, "Profile", 1, false);
                    spawn_tab_button(tabs, theme, "Settings", 2, false);
                });
                
                // Tab content panels container
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(120.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    BorderRadius::bottom(Val::Px(12.0)),
                )).with_children(|content_area| {
                    // Tab 1 content - Home
                    spawn_tab_content(content_area, theme, 0, "Home", 
                        "Welcome to the home tab! This is the default view with overview content.");
                    
                    // Tab 2 content - Profile
                    spawn_tab_content(content_area, theme, 1, "Profile", 
                        "User profile information and settings would appear here.");
                    
                    // Tab 3 content - Settings
                    spawn_tab_content(content_area, theme, 2, "Settings", 
                        "Application configuration and preferences panel.");
                });
            });

            spawn_code_block(section, theme,
r#"// Create tabs with content panels
let tabs = MaterialTabs::new()
    .with_variant(TabVariant::Primary)
    .selected(0);

commands.spawn((tabs, Node::default()))
    .with_children(|parent| {
        // Tab buttons
        parent.spawn((TabButton { index: 0 }, ..));
        parent.spawn((TabButton { index: 1 }, ..));
        
        // Tab content panels
        parent.spawn((TabContent { index: 0 }, Visibility::Inherited, ..));
        parent.spawn((TabContent { index: 1 }, Visibility::Hidden, ..));
    });"#);
        });
}

fn spawn_tab_button(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, index: usize, is_selected: bool) {
    let text_color = if is_selected { theme.primary } else { theme.on_surface_variant };
    let test_id = format!("tab_{}", index + 1);
    
    parent.spawn((
        TabButton { index },
        TestId::new(test_id),
        Button,
        Interaction::None,
        Node {
            flex_grow: 1.0,
            height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(if is_selected { 2.0 } else { 0.0 })),
            ..default()
        },
        BackgroundColor(Color::NONE),
        BorderColor::all(if is_selected { theme.primary } else { Color::NONE }),
    )).with_children(|tab| {
        tab.spawn((
            Text::new(label),
            TextFont { font_size: 14.0, ..default() },
            TextColor(text_color),
        ));
    });
}

fn spawn_tab_content(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, index: usize, title: &str, description: &str) {
    let visibility = if index == 0 { Visibility::Inherited } else { Visibility::Hidden };
    
    parent.spawn((
        TabContent { index },
        visibility,
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            width: Val::Percent(100.0),
            ..default()
        },
    )).with_children(|content| {
        content.spawn((
            Text::new(format!("📄 {}", title)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(theme.on_surface),
        ));
        content.spawn((
            Text::new(description),
            TextFont { font_size: 14.0, ..default() },
            TextColor(theme.on_surface_variant),
        ));
        // Visual indicator box
        content.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                margin: UiRect::top(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.primary_container),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|box_content| {
            box_content.spawn((
                Text::new(format!("Content Panel {}", index + 1)),
                TextFont { font_size: 12.0, ..default() },
                TextColor(theme.on_primary_container),
            ));
        });
    });
}

// ============================================================================
// Select Section
// ============================================================================

fn spawn_select_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Select / Dropdown",
                "Choose from a list of options - Filled and Outlined variants"
            );

            let icon_font_clone = icon_font.clone();
            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                spawn_select_demo(row, theme, &icon_font_clone, SelectVariant::Filled, "Filled Select");
                spawn_select_demo(row, theme, &icon_font_clone, SelectVariant::Outlined, "Outlined Select");
            });

            spawn_code_block(section, theme,
r#"// Create a select dropdown
let options = vec![
    SelectOption::new("opt1", "Option 1"),
    SelectOption::new("opt2", "Option 2"),
    SelectOption::new("opt3", "Option 3"),
];

let select = MaterialSelect::new(options)
    .with_variant(SelectVariant::Outlined)
    .label("Choose an option")
    .selected(0);

commands.spawn((select, Node::default()));"#);
        });
}

fn spawn_select_demo(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: &Handle<Font>, variant: SelectVariant, label: &str) {
    let is_outlined = variant == SelectVariant::Outlined;
    let options = vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()];
    let theme_clone = theme.clone();
    let icon_font_select = icon_font.clone();
    
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(4.0),
        ..default()
    }).with_children(move |col| {
        // Label
        col.spawn((
            Text::new(label),
            TextFont { font_size: 12.0, ..default() },
            TextColor(theme_clone.primary),
        ));
        
        // Container for select + dropdown
        col.spawn((SelectContainer, Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })).with_children(|container| {
            // Select trigger field
            container.spawn((
                SelectTrigger { options: options.clone(), selected_index: 0 },
                Button,
                Interaction::None,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(56.0),
                    padding: UiRect::horizontal(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(if is_outlined { 1.0 } else { 0.0 })),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(if is_outlined { Color::NONE } else { theme_clone.surface_container_highest }),
                BorderColor::all(if is_outlined { theme_clone.outline } else { Color::NONE }),
                BorderRadius::all(Val::Px(4.0)),
            )).with_children(|field| {
                field.spawn((
                    SelectDisplayText,
                    Text::new("Option 1"),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(theme_clone.on_surface),
                ));
                
                // Dropdown arrow icon
                field.spawn((
                    Text::new(ICON_EXPAND_MORE.to_string()),
                    TextFont { font: icon_font_select.clone(), font_size: 20.0, ..default() },
                    TextColor(theme_clone.on_surface_variant),
                ));
            });
            
            // Dropdown menu (hidden by default)
            container.spawn((
                SelectDropdown,
                Visibility::Hidden,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    width: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme_clone.surface_container),
                BorderRadius::all(Val::Px(4.0)),
                BoxShadow::from(ShadowStyle {
                    color: Color::BLACK.with_alpha(0.2),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(4.0),
                    spread_radius: Val::Px(0.0),
                    blur_radius: Val::Px(8.0),
                }),
                GlobalZIndex(100),
            )).with_children(|dropdown| {
                for (i, opt) in options.iter().enumerate() {
                    dropdown.spawn((
                        SelectOption { index: i, label: opt.clone() },
                        Button,
                        Interaction::None,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(48.0),
                            padding: UiRect::horizontal(Val::Px(16.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    )).with_children(|item| {
                        item.spawn((
                            Text::new(opt.clone()),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(theme_clone.on_surface),
                        ));
                    });
                }
            });
        });
    });
}

// ============================================================================
// Snackbar Section
// ============================================================================

fn spawn_snackbar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Snackbars",
                "Brief messages about app processes - Configure options below"
            );

            // Options panel
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(12.0),
                ..default()
            }).with_children(|options| {
                // Duration options
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Duration:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));
                    
                    for (label, duration) in [
                        ("2s", 2.0_f32),
                        ("4s", 4.0_f32),
                        ("10s", 10.0_f32),
                    ] {
                        let is_default = (duration - 4.0).abs() < 0.01;
                        row.spawn((
                            Button,
                            Interaction::None,
                            SnackbarDurationOption(duration),
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(if is_default { theme.primary } else { theme.surface_container_high }),
                            BorderRadius::all(Val::Px(4.0)),
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(if is_default { theme.on_primary } else { theme.on_surface }),
                            ));
                        });
                    }
                });
                
                // Action toggle
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Show action:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));
                    
                    row.spawn((
                        SnackbarActionToggle,
                        Button,
                        Interaction::None,
                        Node {
                            padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_high),
                        BorderRadius::all(Val::Px(4.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Toggle Action"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(theme.on_surface),
                        ));
                    });
                });
            });

            // Trigger button
            section.spawn(Node {
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                row.spawn((
                    SnackbarTrigger,
                    Button,
                    Interaction::None,
                    Node {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(theme.primary),
                    BorderRadius::all(Val::Px(20.0)),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Show Snackbar"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_primary),
                    ));
                });
            });

            // Snackbar preview (static example)
            section.spawn((
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(48.0),
                    padding: UiRect::horizontal(Val::Px(16.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme.inverse_surface),
                BorderRadius::all(Val::Px(4.0)),
                BoxShadow::from(ShadowStyle {
                    color: Color::BLACK.with_alpha(0.2),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(2.0),
                    spread_radius: Val::Px(0.0),
                    blur_radius: Val::Px(4.0),
                }),
            )).with_children(|snackbar| {
                snackbar.spawn((
                    Text::new("Item deleted"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.inverse_on_surface),
                ));
                
                snackbar.spawn((
                    Button,
                    Interaction::None,
                    Node {
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("UNDO"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.inverse_primary),
                    ));
                });
            });

            spawn_code_block(section, theme,
r#"// Show a snackbar (via event)
commands.write_message(ShowSnackbar::message("File saved"));

// With action button
commands.write_message(
    ShowSnackbar::with_action("Item deleted", "UNDO")
        .duration(5.0)
);

// Handle action clicks
fn handle_snackbar(mut events: MessageReader<SnackbarActionEvent>) {
    for event in events.read() {
        if event.action == "UNDO" {
            // Handle undo
        }
    }
}"#);
        });
}

// ============================================================================
// Tooltip Section
// ============================================================================

fn spawn_tooltip_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, _icon_font: Handle<Font>) {
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
                "Tooltips",
                "Contextual information on hover - Configure options below"
            );

            // Options panel
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(12.0),
                ..default()
            }).with_children(|options| {
                // Position options
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Position:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));
                    
                    // Position buttons
                    for (label, pos) in [
                        ("Top", TooltipPosition::Top),
                        ("Bottom", TooltipPosition::Bottom),
                        ("Left", TooltipPosition::Left),
                        ("Right", TooltipPosition::Right),
                    ] {
                        let is_default = pos == TooltipPosition::Bottom;
                        row.spawn((
                            Button,
                            Interaction::None,
                            TooltipPositionOption(pos),
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(if is_default { theme.primary } else { theme.surface_container_high }),
                            BorderRadius::all(Val::Px(4.0)),
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(if is_default { theme.on_primary } else { theme.on_surface }),
                            ));
                        });
                    }
                });
                
                // Delay options
                options.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("Delay:"),
                        TextFont { font_size: 12.0, ..default() },
                        TextColor(theme.on_surface_variant),
                    ));
                    
                    for (label, delay) in [
                        ("0.15s", 0.15_f32),
                        ("0.5s", 0.5_f32),
                        ("1.0s", 1.0_f32),
                    ] {
                        let is_default = (delay - 0.5).abs() < 0.01;
                        row.spawn((
                            Button,
                            Interaction::None,
                            TooltipDelayOption(delay),
                            Node {
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(if is_default { theme.primary } else { theme.surface_container_high }),
                            BorderRadius::all(Val::Px(4.0)),
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(if is_default { theme.on_primary } else { theme.on_surface }),
                            ));
                        });
                    }
                });
            });

            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(32.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }).with_children(|row| {
                // Interactive tooltip demo button
                row.spawn((
                    TooltipDemoButton,
                    Button,
                    Interaction::None,
                    TooltipTrigger::new("Hover to see tooltip!").bottom(),
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(theme.primary),
                    BorderRadius::all(Val::Px(8.0)),
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Hover Me"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(theme.on_primary),
                    ));
                });
                
                row.spawn((
                    Text::new("← Hover to test tooltip with selected options"),
                    TextFont { font_size: 12.0, ..default() },
                    TextColor(theme.on_surface_variant),
                ));
            });

            spawn_code_block(section, theme,
r#"// Add tooltip to an element
commands.spawn((
    Button,
    TooltipTrigger::new("Add to favorites")
        .with_position(TooltipPosition::Bottom)
        .with_delay(0.5),  // 500ms delay
));

// Rich tooltip with title
let trigger = TooltipTrigger::rich("Title", "Description text")
    .with_position(TooltipPosition::Right);"#);
        });
}

// ============================================================================
// App Bar Section
// ============================================================================

fn spawn_app_bar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "App Bars",
                "Top and Bottom app bars for navigation and actions"
            );

            let icon_font_clone = icon_font.clone();
            // Top App Bar preview
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Top App Bar (Small)"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                let icon_font_top = icon_font_clone.clone();
                // Top app bar
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(64.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                )).with_children(|bar| {
                    // Menu icon
                    bar.spawn((
                        AppBarIconButton("menu".to_string()),
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(ICON_MENU.to_string()),
                            TextFont { font: icon_font_top.clone(), font_size: 24.0, ..default() },
                            TextColor(theme.on_surface),
                        ));
                    });
                    
                    // Title
                    bar.spawn((
                        Text::new("Page Title"),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(theme.on_surface),
                        Node { flex_grow: 1.0, ..default() },
                    ));
                    
                    // Actions
                    bar.spawn((
                        AppBarIconButton("more".to_string()),
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(ICON_MORE_VERT.to_string()),
                            TextFont { font: icon_font_top.clone(), font_size: 24.0, ..default() },
                            TextColor(theme.on_surface),
                        ));
                    });
                });
                
                col.spawn((
                    Text::new("Bottom App Bar"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                    Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ));
                
                let icon_font_bottom = icon_font_clone.clone();
                // Bottom app bar
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                )).with_children(|bar| {
                    // Left actions
                    bar.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..default()
                    }).with_children(|actions| {
                        let icons = [(ICON_MENU, "menu"), (ICON_SEARCH, "search"), (ICON_CHECK, "check"), (ICON_CLOSE, "close")];
                        for (icon, name) in icons {
                            let icon_f = icon_font_bottom.clone();
                            actions.spawn((
                                AppBarIconButton(name.to_string()),
                                Button,
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(20.0)),
                            )).with_children(|btn| {
                                btn.spawn((
                                    Text::new(icon.to_string()),
                                    TextFont { font: icon_f, font_size: 20.0, ..default() },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                        }
                    });
                    
                    // FAB with proper icon
                    let icon_f_fab = icon_font_bottom.clone();
                    bar.spawn((
                        AppBarIconButton("fab".to_string()),
                        Button,
                        Node {
                            width: Val::Px(56.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme.primary_container),
                        BorderRadius::all(Val::Px(16.0)),
                    )).with_children(|fab| {
                        fab.spawn((
                            Text::new(ICON_ADD.to_string()),
                            TextFont { font: icon_f_fab, font_size: 28.0, ..default() },
                            TextColor(theme.on_primary_container),
                        ));
                    });
                });
            });

            spawn_code_block(section, theme,
r#"// Create a top app bar
let app_bar = TopAppBar::new()
    .with_variant(TopAppBarVariant::Small)
    .title("My App")
    .navigation_icon("menu");

commands.spawn((
    app_bar,
    Node { 
        width: Val::Percent(100.0), 
        height: Val::Px(64.0),
        ..default() 
    },
    BackgroundColor(theme.surface),
));

// Create a bottom app bar
let bottom_bar = BottomAppBar::new()
    .actions(vec!["search", "share", "delete"]);"#);
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
