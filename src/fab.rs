//! Material Design 3 Floating Action Button (FAB) component
//!
//! FABs represent the primary action on a screen.
//! Reference: <https://m3.material.io/components/floating-action-button/overview>
//!
//! ## Bevy 0.17 Improvements
//! 
//! This module now leverages native `BoxShadow` for elevation shadows.

use bevy::prelude::*;
use bevy::ui::BoxShadow;

use crate::{
    elevation::Elevation,
    ripple::RippleHost,
    theme::{blend_state_layer, MaterialTheme},
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the FAB component
pub struct FabPlugin;

impl Plugin for FabPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FabClickEvent>()
            .add_systems(Update, (
                fab_interaction_system,
                fab_style_system,
                fab_shadow_system,
            ));
    }
}

/// FAB size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FabSize {
    /// Small FAB: 40dp
    Small,
    /// Regular FAB: 56dp
    #[default]
    Regular,
    /// Large FAB: 96dp
    Large,
}

impl FabSize {
    /// Get the size in pixels
    pub fn size(&self) -> f32 {
        match self {
            FabSize::Small => 40.0,
            FabSize::Regular => 56.0,
            FabSize::Large => 96.0,
        }
    }

    /// Get the icon size for this FAB size
    pub fn icon_size(&self) -> f32 {
        match self {
            FabSize::Small => 24.0,
            FabSize::Regular => 24.0,
            FabSize::Large => 36.0,
        }
    }

    /// Get the corner radius for this FAB size
    pub fn corner_radius(&self) -> f32 {
        match self {
            FabSize::Small => CornerRadius::MEDIUM,
            FabSize::Regular => CornerRadius::LARGE,
            FabSize::Large => CornerRadius::EXTRA_LARGE,
        }
    }
}

/// FAB color variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FabColor {
    /// Primary container color (default)
    #[default]
    Primary,
    /// Surface color
    Surface,
    /// Secondary container color
    Secondary,
    /// Tertiary container color
    Tertiary,
}

/// Material FAB component
#[derive(Component)]
pub struct MaterialFab {
    /// FAB size
    pub size: FabSize,
    /// FAB color variant
    pub color: FabColor,
    /// Whether the FAB is lowered (reduced elevation)
    pub lowered: bool,
    /// Icon identifier
    pub icon: String,
    /// Optional label for extended FAB
    pub label: Option<String>,
    /// Interaction state
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialFab {
    /// Create a new FAB
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            size: FabSize::default(),
            color: FabColor::default(),
            lowered: false,
            icon: icon.into(),
            label: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the FAB size
    pub fn with_size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// Set the FAB color
    pub fn with_color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    /// Make this a lowered FAB
    pub fn lowered(mut self) -> Self {
        self.lowered = true;
        self
    }

    /// Make this an extended FAB with a label
    pub fn extended(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        let base = match self.color {
            FabColor::Primary => theme.primary_container,
            FabColor::Surface => theme.surface_container_high,
            FabColor::Secondary => theme.secondary_container,
            FabColor::Tertiary => theme.tertiary_container,
        };
        
        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            let state_color = self.content_color(theme);
            blend_state_layer(base, state_color, state_opacity)
        } else {
            base
        }
    }
    
    /// Get the state layer opacity
    fn state_layer_opacity(&self) -> f32 {
        if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }

    /// Get the icon/content color
    pub fn content_color(&self, theme: &MaterialTheme) -> Color {
        match self.color {
            FabColor::Primary => theme.on_primary_container,
            FabColor::Surface => theme.primary,
            FabColor::Secondary => theme.on_secondary_container,
            FabColor::Tertiary => theme.on_tertiary_container,
        }
    }

    /// Get the elevation
    pub fn elevation(&self) -> Elevation {
        if self.lowered {
            if self.pressed {
                Elevation::Level1
            } else if self.hovered {
                Elevation::Level2
            } else {
                Elevation::Level1
            }
        } else if self.pressed {
            Elevation::Level3
        } else if self.hovered {
            Elevation::Level4
        } else {
            Elevation::Level3
        }
    }

    /// Check if this is an extended FAB
    pub fn is_extended(&self) -> bool {
        self.label.is_some()
    }
}

/// Event when FAB is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct FabClickEvent {
    pub entity: Entity,
}

/// System to handle FAB interactions
fn fab_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialFab),
        (Changed<Interaction>, With<MaterialFab>),
    >,
    mut click_events: MessageWriter<FabClickEvent>,
) {
    for (entity, interaction, mut fab) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                fab.pressed = true;
                fab.hovered = false;
                click_events.write(FabClickEvent { entity });
            }
            Interaction::Hovered => {
                fab.pressed = false;
                fab.hovered = true;
            }
            Interaction::None => {
                fab.pressed = false;
                fab.hovered = false;
            }
        }
    }
}

/// System to update FAB styles
fn fab_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut fabs: Query<(&MaterialFab, &mut BackgroundColor), Changed<MaterialFab>>,
) {
    let Some(theme) = theme else { return };

    for (fab, mut bg_color) in fabs.iter_mut() {
        *bg_color = BackgroundColor(fab.background_color(&theme));
    }
}

/// System to update FAB shadows using native BoxShadow
fn fab_shadow_system(
    mut fabs: Query<(&MaterialFab, &mut BoxShadow), Changed<MaterialFab>>,
) {
    for (fab, mut box_shadow) in fabs.iter_mut() {
        let elevation = fab.elevation();
        *box_shadow = elevation.to_box_shadow();
    }
}

/// Builder for FABs
pub struct FabBuilder {
    fab: MaterialFab,
}

impl FabBuilder {
    /// Create a new FAB builder
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            fab: MaterialFab::new(icon),
        }
    }

    /// Set size
    pub fn size(mut self, size: FabSize) -> Self {
        self.fab.size = size;
        self
    }

    /// Make small FAB
    pub fn small(self) -> Self {
        self.size(FabSize::Small)
    }

    /// Make large FAB
    pub fn large(self) -> Self {
        self.size(FabSize::Large)
    }

    /// Set color
    pub fn color(mut self, color: FabColor) -> Self {
        self.fab.color = color;
        self
    }

    /// Make surface FAB
    pub fn surface(self) -> Self {
        self.color(FabColor::Surface)
    }

    /// Make secondary FAB
    pub fn secondary(self) -> Self {
        self.color(FabColor::Secondary)
    }

    /// Make tertiary FAB
    pub fn tertiary(self) -> Self {
        self.color(FabColor::Tertiary)
    }

    /// Make lowered FAB
    pub fn lowered(mut self) -> Self {
        self.fab.lowered = true;
        self
    }

    /// Make extended FAB
    pub fn extended(mut self, label: impl Into<String>) -> Self {
        self.fab.label = Some(label.into());
        self
    }

    /// Build the FAB bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.fab.background_color(theme);
        let size = self.fab.size.size();
        let corner_radius = self.fab.size.corner_radius();
        let is_extended = self.fab.is_extended();
        let elevation = self.fab.elevation();

        (
            self.fab,
            Button,
            RippleHost::new(),
            Node {
                width: if is_extended { Val::Auto } else { Val::Px(size) },
                height: Val::Px(size),
                min_width: if is_extended { Val::Px(80.0) } else { Val::Auto },
                padding: if is_extended {
                    UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::LARGE))
                } else {
                    UiRect::all(Val::Px(0.0))
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: if is_extended { Val::Px(Spacing::SMALL) } else { Val::Px(0.0) },
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(corner_radius)),
            // Native Bevy 0.17 shadow support
            elevation.to_box_shadow(),
        )
    }
}
