//! Focus ring system for Material Design 3
//!
//! Focus rings provide visible keyboard focus indicators for accessibility.
//! Reference: <https://m3.material.io/foundations/interaction/states/state-layers>

use bevy::prelude::*;

/// Plugin for the focus ring system
pub struct FocusPlugin;

impl Plugin for FocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_focus_ring_system);
    }
}

/// Component that enables focus ring on an entity
#[derive(Component, Default)]
pub struct Focusable {
    /// Whether the element is currently focused
    pub focused: bool,
    /// Whether focus came from keyboard navigation
    pub focus_visible: bool,
    /// Custom focus ring color
    pub ring_color: Option<Color>,
    /// Focus ring offset from the element
    pub ring_offset: f32,
    /// Focus ring width
    pub ring_width: f32,
}

impl Focusable {
    /// Create a new focusable component
    pub fn new() -> Self {
        Self {
            focused: false,
            focus_visible: false,
            ring_color: None,
            ring_offset: 2.0,
            ring_width: 3.0,
        }
    }

    /// Set the focus ring color
    pub fn with_color(mut self, color: Color) -> Self {
        self.ring_color = Some(color);
        self
    }

    /// Set the focus ring offset
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.ring_offset = offset;
        self
    }

    /// Set the focus ring width
    pub fn with_width(mut self, width: f32) -> Self {
        self.ring_width = width;
        self
    }
}

/// Marker component for focus ring entities
#[derive(Component)]
pub struct FocusRing {
    /// The entity this focus ring belongs to
    pub target: Entity,
}

/// Event when an element gains focus
#[derive(Event, bevy::prelude::Message)]
pub struct FocusGained {
    /// The focused entity
    pub entity: Entity,
    /// Whether focus came from keyboard
    pub from_keyboard: bool,
}

/// Event when an element loses focus
#[derive(Event, bevy::prelude::Message)]
pub struct FocusLost {
    /// The entity that lost focus
    pub entity: Entity,
}

/// System to update focus ring visibility
fn update_focus_ring_system(
    focusables: Query<(&Focusable, &Children), Changed<Focusable>>,
    mut focus_rings: Query<&mut Node, With<FocusRing>>,
) {
    for (focusable, children) in focusables.iter() {
        for child in children.iter() {
            if let Ok(mut node) = focus_rings.get_mut(child) {
                node.display = if focusable.focus_visible {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

/// Create a focus ring node bundle
pub fn create_focus_ring(target: Entity, color: Color, offset: f32, width: f32) -> impl Bundle {
    (
        FocusRing { target },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(-offset - width),
            top: Val::Px(-offset - width),
            right: Val::Px(-offset - width),
            bottom: Val::Px(-offset - width),
            border: UiRect::all(Val::Px(width)),
            display: Display::None,
            ..default()
        },
        BorderColor::all(color),
        BorderRadius::all(Val::Px(4.0 + offset)),
        BackgroundColor(Color::NONE),
    )
}
