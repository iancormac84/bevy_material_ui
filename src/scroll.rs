//! Scrollable container component for Material Design 3
//!
//! Provides mouse wheel scrolling and scrollbar visuals using Bevy's native scroll system.
//! Uses Bevy's `ScrollPosition` component and `Overflow::scroll_y()` for actual scrolling.
//!
//! Usage:
//! ```ignore
//! commands.spawn((
//!     ScrollContainer::vertical(),
//!     ScrollPosition::default(),
//!     Node { 
//!         height: Val::Px(400.0), 
//!         overflow: Overflow::scroll_y(), // Use Bevy's native scroll
//!         ..default() 
//!     },
//! )).with_children(|parent| {
//!     parent.spawn((ScrollContent, Node { ..default() }))
//!         .with_children(|content| {
//!             // Your scrollable content here
//!         });
//! });
//! ```

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;

use crate::theme::MaterialTheme;

/// Plugin for scroll container functionality
pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            mouse_wheel_scroll_system,
            sync_scroll_state_system,
            scrollbar_thumb_drag_system,
            update_scrollbars,
        ).chain());
    }
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ScrollDirection {
    /// Vertical scrolling only
    #[default]
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both directions
    Both,
}

/// Scroll container component
#[derive(Component)]
pub struct ScrollContainer {
    /// Scroll direction
    pub direction: ScrollDirection,
    /// Current scroll offset (pixels)
    pub offset: Vec2,
    /// Target scroll offset for smooth scrolling
    pub target_offset: Vec2,
    /// Maximum scroll offset
    pub max_offset: Vec2,
    /// Content size
    pub content_size: Vec2,
    /// Container size
    pub container_size: Vec2,
    /// Scroll sensitivity (pixels per scroll unit)
    pub sensitivity: f32,
    /// Whether smooth scrolling is enabled
    pub smooth: bool,
    /// Smooth scrolling speed (0.0-1.0, higher = faster)
    pub smooth_speed: f32,
    /// Whether the container is currently being dragged
    pub dragging: bool,
    /// Last drag position
    pub last_drag_pos: Option<Vec2>,
    /// Whether to show scrollbars
    pub show_scrollbars: bool,
    /// Scrollbar width
    pub scrollbar_width: f32,
}

impl Default for ScrollContainer {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            offset: Vec2::ZERO,
            target_offset: Vec2::ZERO,
            max_offset: Vec2::ZERO,
            content_size: Vec2::ZERO,
            container_size: Vec2::ZERO,
            sensitivity: 40.0,
            smooth: true,
            smooth_speed: 0.2,
            dragging: false,
            last_drag_pos: None,
            show_scrollbars: true,
            scrollbar_width: 8.0,
        }
    }
}

impl ScrollContainer {
    /// Create a new vertical scroll container
    pub fn vertical() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            ..default()
        }
    }

    /// Create a new horizontal scroll container
    pub fn horizontal() -> Self {
        Self {
            direction: ScrollDirection::Horizontal,
            ..default()
        }
    }

    /// Create a scroll container that scrolls in both directions
    pub fn both() -> Self {
        Self {
            direction: ScrollDirection::Both,
            ..default()
        }
    }

    /// Set scroll sensitivity
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Enable or disable smooth scrolling
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    /// Show or hide scrollbars
    pub fn with_scrollbars(mut self, show: bool) -> Self {
        self.show_scrollbars = show;
        self
    }

    /// Set scrollbar width
    pub fn with_scrollbar_width(mut self, width: f32) -> Self {
        self.scrollbar_width = width;
        self
    }

    /// Scroll by a delta amount
    pub fn scroll_by(&mut self, delta: Vec2) {
        match self.direction {
            ScrollDirection::Vertical => {
                self.target_offset.y += delta.y;
            }
            ScrollDirection::Horizontal => {
                self.target_offset.x += delta.x;
            }
            ScrollDirection::Both => {
                self.target_offset += delta;
            }
        }
    }

    /// Check if scrolling is needed in x direction
    pub fn needs_scroll_x(&self) -> bool {
        self.max_offset.x > 0.0 && matches!(self.direction, ScrollDirection::Horizontal | ScrollDirection::Both)
    }

    /// Check if scrolling is needed in y direction
    pub fn needs_scroll_y(&self) -> bool {
        self.max_offset.y > 0.0 && matches!(self.direction, ScrollDirection::Vertical | ScrollDirection::Both)
    }

    /// Get scrollbar thumb size for vertical scrollbar
    pub fn vertical_thumb_size(&self) -> f32 {
        if self.content_size.y <= 0.0 || self.container_size.y <= 0.0 {
            return 0.0;
        }
        let ratio = self.container_size.y / self.content_size.y;
        (self.container_size.y * ratio).max(30.0).min(self.container_size.y)
    }

    /// Get scrollbar thumb position for vertical scrollbar (0.0 to 1.0)
    pub fn vertical_thumb_position(&self) -> f32 {
        if self.max_offset.y <= 0.0 {
            return 0.0;
        }
        self.offset.y / self.max_offset.y
    }

    /// Get scrollbar thumb size for horizontal scrollbar
    pub fn horizontal_thumb_size(&self) -> f32 {
        if self.content_size.x <= 0.0 || self.container_size.x <= 0.0 {
            return 0.0;
        }
        let ratio = self.container_size.x / self.content_size.x;
        (self.container_size.x * ratio).max(30.0).min(self.container_size.x)
    }

    /// Get scrollbar thumb position for horizontal scrollbar (0.0 to 1.0)
    pub fn horizontal_thumb_position(&self) -> f32 {
        if self.max_offset.x <= 0.0 {
            return 0.0;
        }
        self.offset.x / self.max_offset.x
    }
}

/// Marker component for scroll content (the inner scrollable element)
#[derive(Component, Default)]
pub struct ScrollContent;

/// Marker for vertical scrollbar track
#[derive(Component)]
pub struct ScrollbarTrackVertical;

/// Marker for vertical scrollbar thumb
#[derive(Component)]
pub struct ScrollbarThumbVertical;

/// Marker for horizontal scrollbar track
#[derive(Component)]
pub struct ScrollbarTrackHorizontal;

/// Marker for horizontal scrollbar thumb
#[derive(Component)]
pub struct ScrollbarThumbHorizontal;

/// Component to track scrollbar thumb dragging state
#[derive(Component, Default)]
pub struct ScrollbarDragging {
    /// Whether the thumb is being dragged
    pub is_dragging: bool,
    /// Starting mouse position when drag began
    pub drag_start_pos: Option<Vec2>,
    /// Starting scroll offset when drag began
    pub drag_start_offset: f32,
}

/// Line height for scroll calculations
const LINE_HEIGHT: f32 = 21.0;

/// System to handle mouse wheel scrolling
/// This follows Bevy's pattern: read mouse wheel, find hovered entities, update their ScrollPosition
#[allow(deprecated)] // EventReader renamed to MessageReader in Bevy 0.17
fn mouse_wheel_scroll_system(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scrollable_query: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ScrollContainer>>,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        // Calculate scroll delta (negate for natural scrolling direction)
        let mut delta = Vec2::new(-mouse_wheel.x, -mouse_wheel.y);
        
        // Convert line units to pixels
        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }
        
        // Shift key swaps x/y for horizontal scrolling
        if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }
        
        // Find entities under the cursor and scroll them
        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                // Try to get the scrollable container - walk up parent hierarchy
                if let Ok((mut scroll_position, node, computed)) = scrollable_query.get_mut(entity) {
                    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
                    
                    // Handle vertical scroll
                    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
                        let at_max = if delta.y > 0.0 {
                            scroll_position.y >= max_offset.y
                        } else {
                            scroll_position.y <= 0.0
                        };
                        
                        if !at_max {
                            scroll_position.y = (scroll_position.y + delta.y).clamp(0.0, max_offset.y);
                        }
                    }
                    
                    // Handle horizontal scroll
                    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
                        let at_max = if delta.x > 0.0 {
                            scroll_position.x >= max_offset.x
                        } else {
                            scroll_position.x <= 0.0
                        };
                        
                        if !at_max {
                            scroll_position.x = (scroll_position.x + delta.x).clamp(0.0, max_offset.x);
                        }
                    }
                }
            }
        }
    }
}

/// System to sync ScrollContainer state with Bevy's native ScrollPosition
/// This reads the ScrollPosition (managed by Bevy's scroll system) and updates our ScrollContainer
fn sync_scroll_state_system(
    mut containers: Query<(&mut ScrollContainer, &ScrollPosition, &ComputedNode)>,
) {
    for (mut container, scroll_pos, computed) in containers.iter_mut() {
        let container_size = computed.size();
        let content_size = computed.content_size();
        let scale = computed.inverse_scale_factor();
        
        container.container_size = container_size;
        container.content_size = content_size;
        
        // Calculate max offset
        let max_offset = (content_size - container_size).max(Vec2::ZERO) * scale;
        container.max_offset = max_offset;
        
        // Sync offset from Bevy's ScrollPosition
        container.offset = **scroll_pos;
        container.target_offset = **scroll_pos;
    }
}

/// System to handle scrollbar thumb dragging
fn scrollbar_thumb_drag_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut thumb_v: Query<
        (&Interaction, &mut ScrollbarDragging, &ChildOf, &ComputedNode),
        (With<ScrollbarThumbVertical>, Without<ScrollbarThumbHorizontal>),
    >,
    mut thumb_h: Query<
        (&Interaction, &mut ScrollbarDragging, &ChildOf, &ComputedNode),
        (With<ScrollbarThumbHorizontal>, Without<ScrollbarThumbVertical>),
    >,
    track_v: Query<(&ComputedNode, &ChildOf), With<ScrollbarTrackVertical>>,
    track_h: Query<(&ComputedNode, &ChildOf), With<ScrollbarTrackHorizontal>>,
    mut containers: Query<(&ScrollContainer, &mut ScrollPosition, &ComputedNode)>,
) {
    let Ok(window) = windows.single() else { return };
    let cursor_pos = window.cursor_position();
    
    // Handle vertical scrollbar thumb dragging
    for (interaction, mut drag_state, track_parent, _thumb_node) in thumb_v.iter_mut() {
        // Start dragging on press
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                // Find the container through the track's parent
                if let Ok((_track_node, scroll_parent)) = track_v.get(track_parent.0) {
                    if let Ok((_, scroll_pos, _)) = containers.get(scroll_parent.0) {
                        drag_state.is_dragging = true;
                        drag_state.drag_start_pos = Some(pos);
                        drag_state.drag_start_offset = scroll_pos.y;
                    }
                }
            }
        }
        
        // Stop dragging on release
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.is_dragging = false;
            drag_state.drag_start_pos = None;
        }
        
        // Update scroll position while dragging
        if drag_state.is_dragging {
            if let (Some(start_pos), Some(current_pos)) = (drag_state.drag_start_pos, cursor_pos) {
                if let Ok((track_node, scroll_parent)) = track_v.get(track_parent.0) {
                    if let Ok((container, mut scroll_pos, computed)) = containers.get_mut(scroll_parent.0) {
                        let track_height = track_node.size().y;
                        let thumb_height = container.vertical_thumb_size();
                        let available_track = track_height - thumb_height;
                        
                        if available_track > 0.0 {
                            // Calculate how much the thumb moved in track space
                            let drag_delta = current_pos.y - start_pos.y;
                            // Convert to scroll offset
                            let content_size = computed.content_size();
                            let container_size = computed.size();
                            let scale = computed.inverse_scale_factor();
                            let max_offset_y = (content_size.y - container_size.y).max(0.0) * scale;
                            
                            let scroll_delta = (drag_delta / available_track) * max_offset_y;
                            scroll_pos.y = (drag_state.drag_start_offset + scroll_delta)
                                .clamp(0.0, max_offset_y);
                        }
                    }
                }
            }
        }
    }
    
    // Handle horizontal scrollbar thumb dragging
    for (interaction, mut drag_state, track_parent, _thumb_node) in thumb_h.iter_mut() {
        // Start dragging on press
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            if let Some(pos) = cursor_pos {
                // Find the container through the track's parent
                if let Ok((_track_node, scroll_parent)) = track_h.get(track_parent.0) {
                    if let Ok((_, scroll_pos, _)) = containers.get(scroll_parent.0) {
                        drag_state.is_dragging = true;
                        drag_state.drag_start_pos = Some(pos);
                        drag_state.drag_start_offset = scroll_pos.x;
                    }
                }
            }
        }
        
        // Stop dragging on release
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.is_dragging = false;
            drag_state.drag_start_pos = None;
        }
        
        // Update scroll position while dragging
        if drag_state.is_dragging {
            if let (Some(start_pos), Some(current_pos)) = (drag_state.drag_start_pos, cursor_pos) {
                if let Ok((track_node, scroll_parent)) = track_h.get(track_parent.0) {
                    if let Ok((container, mut scroll_pos, computed)) = containers.get_mut(scroll_parent.0) {
                        let track_width = track_node.size().x;
                        let thumb_width = container.horizontal_thumb_size();
                        let available_track = track_width - thumb_width;
                        
                        if available_track > 0.0 {
                            // Calculate how much the thumb moved in track space
                            let drag_delta = current_pos.x - start_pos.x;
                            // Convert to scroll offset
                            let content_size = computed.content_size();
                            let container_size = computed.size();
                            let scale = computed.inverse_scale_factor();
                            let max_offset_x = (content_size.x - container_size.x).max(0.0) * scale;
                            
                            let scroll_delta = (drag_delta / available_track) * max_offset_x;
                            scroll_pos.x = (drag_state.drag_start_offset + scroll_delta)
                                .clamp(0.0, max_offset_x);
                        }
                    }
                }
            }
        }
    }
}

/// System to update scrollbar visuals
fn update_scrollbars(
    containers: Query<(&ScrollContainer, &Children)>,
    mut thumb_v: Query<&mut Node, (With<ScrollbarThumbVertical>, Without<ScrollbarThumbHorizontal>)>,
    mut thumb_h: Query<&mut Node, (With<ScrollbarThumbHorizontal>, Without<ScrollbarThumbVertical>)>,
    mut track_v_vis: Query<&mut Visibility, (With<ScrollbarTrackVertical>, Without<ScrollbarTrackHorizontal>)>,
    mut track_h_vis: Query<&mut Visibility, (With<ScrollbarTrackHorizontal>, Without<ScrollbarTrackVertical>)>,
    children_query: Query<&Children>,
) {
    for (container, children) in containers.iter() {
        // Find scrollbar elements in children
        for child in children.iter() {
            // Check for vertical track
            if let Ok(mut vis) = track_v_vis.get_mut(child) {
                *vis = if container.needs_scroll_y() && container.show_scrollbars {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            
            // Check for horizontal track
            if let Ok(mut vis) = track_h_vis.get_mut(child) {
                *vis = if container.needs_scroll_x() && container.show_scrollbars {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }

            // Look for thumbs in track children
            if let Ok(track_children) = children_query.get(child) {
                for track_child in track_children.iter() {
                    // Update vertical thumb
                    if let Ok(mut node) = thumb_v.get_mut(track_child) {
                        let thumb_size = container.vertical_thumb_size();
                        let thumb_pos = container.vertical_thumb_position();
                        let track_height = container.container_size.y - container.scrollbar_width;
                        let thumb_travel = (track_height - thumb_size).max(0.0);
                        
                        node.height = Val::Px(thumb_size.max(30.0));
                        node.top = Val::Px(thumb_pos * thumb_travel);
                    }
                    
                    // Update horizontal thumb
                    if let Ok(mut node) = thumb_h.get_mut(track_child) {
                        let thumb_size = container.horizontal_thumb_size();
                        let thumb_pos = container.horizontal_thumb_position();
                        let track_width = container.container_size.x - container.scrollbar_width;
                        let thumb_travel = (track_width - thumb_size).max(0.0);
                        
                        node.width = Val::Px(thumb_size.max(30.0));
                        node.left = Val::Px(thumb_pos * thumb_travel);
                    }
                }
            }
        }
    }
}

/// Spawn scrollbars for a scroll container
/// Call this after spawning ScrollContainer to add visual scrollbars
pub fn spawn_scrollbars(commands: &mut ChildSpawnerCommands, theme: &MaterialTheme, direction: ScrollDirection) {
    let scrollbar_width = 10.0;
    let track_color = theme.surface_container_highest.with_alpha(0.5);
    let thumb_color = theme.primary.with_alpha(0.7);

    // Vertical scrollbar
    if matches!(direction, ScrollDirection::Vertical | ScrollDirection::Both) {
        commands.spawn((
            ScrollbarTrackVertical,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(if matches!(direction, ScrollDirection::Both) { scrollbar_width } else { 0.0 }),
                width: Val::Px(scrollbar_width),
                ..default()
            },
            BackgroundColor(track_color),
            BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
        )).with_children(|track| {
            track.spawn((
                ScrollbarThumbVertical,
                ScrollbarDragging::default(),
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(scrollbar_width),
                    height: Val::Px(50.0), // Will be updated by system
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(thumb_color),
                BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
            ));
        });
    }

    // Horizontal scrollbar
    if matches!(direction, ScrollDirection::Horizontal | ScrollDirection::Both) {
        commands.spawn((
            ScrollbarTrackHorizontal,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(if matches!(direction, ScrollDirection::Both) { scrollbar_width } else { 0.0 }),
                height: Val::Px(scrollbar_width),
                ..default()
            },
            BackgroundColor(track_color),
            BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
        )).with_children(|track| {
            track.spawn((
                ScrollbarThumbHorizontal,
                ScrollbarDragging::default(),
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(scrollbar_width),
                    width: Val::Px(50.0), // Will be updated by system
                    left: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(thumb_color),
                BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
            ));
        });
    }
}

/// Builder for scroll containers
pub struct ScrollContainerBuilder {
    direction: ScrollDirection,
    sensitivity: f32,
    smooth: bool,
    smooth_speed: f32,
    show_scrollbars: bool,
}

impl Default for ScrollContainerBuilder {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            sensitivity: 40.0,
            smooth: true,
            smooth_speed: 0.2,
            show_scrollbars: true,
        }
    }
}

impl ScrollContainerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertical(mut self) -> Self {
        self.direction = ScrollDirection::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.direction = ScrollDirection::Horizontal;
        self
    }

    pub fn both(mut self) -> Self {
        self.direction = ScrollDirection::Both;
        self
    }

    pub fn sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    pub fn with_scrollbars(mut self, show: bool) -> Self {
        self.show_scrollbars = show;
        self
    }

    pub fn build(self) -> ScrollContainer {
        ScrollContainer {
            direction: self.direction,
            sensitivity: self.sensitivity,
            smooth: self.smooth,
            smooth_speed: self.smooth_speed,
            show_scrollbars: self.show_scrollbars,
            ..default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_container_default() {
        let container = ScrollContainer::default();
        assert!(matches!(container.direction, ScrollDirection::Vertical));
        assert_eq!(container.offset, Vec2::ZERO);
        assert!(container.smooth);
        assert!(container.show_scrollbars);
    }

    #[test]
    fn test_scroll_container_vertical() {
        let container = ScrollContainer::vertical();
        assert!(container.needs_scroll_y() == false); // No content yet
    }

    #[test]
    fn test_scroll_by() {
        let mut container = ScrollContainer::vertical();
        container.scroll_by(Vec2::new(10.0, 20.0));
        assert_eq!(container.target_offset.y, 20.0);
        assert_eq!(container.target_offset.x, 0.0);
    }

    #[test]
    fn test_scroll_builder() {
        let container = ScrollContainerBuilder::new()
            .vertical()
            .sensitivity(50.0)
            .smooth(false)
            .with_scrollbars(false)
            .build();
        
        assert_eq!(container.sensitivity, 50.0);
        assert!(!container.smooth);
        assert!(!container.show_scrollbars);
    }

    #[test]
    fn test_thumb_calculations() {
        let mut container = ScrollContainer::vertical();
        container.container_size = Vec2::new(100.0, 400.0);
        container.content_size = Vec2::new(100.0, 1000.0);
        container.max_offset = Vec2::new(0.0, 600.0);
        container.offset = Vec2::new(0.0, 300.0);

        let thumb_size = container.vertical_thumb_size();
        assert!(thumb_size > 0.0);
        assert!(thumb_size < container.container_size.y);

        let thumb_pos = container.vertical_thumb_position();
        assert_eq!(thumb_pos, 0.5); // 300 / 600
    }
}
