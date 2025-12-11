//! Material Design 3 Switch component
//!
//! Switches toggle the state of a single item on or off.
//! Reference: <https://m3.material.io/components/switch/overview>

use bevy::prelude::*;

use crate::{
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::CornerRadius,
};

/// Plugin for the switch component
pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SwitchChangeEvent>()
            .add_systems(Update, (switch_interaction_system, switch_style_system));
    }
}

/// Material switch component
#[derive(Component)]
pub struct MaterialSwitch {
    /// Whether the switch is on
    pub selected: bool,
    /// Whether the switch is disabled
    pub disabled: bool,
    /// Whether the switch has icons
    pub with_icon: bool,
    /// Animation progress (0.0 = off, 1.0 = on)
    pub animation_progress: f32,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialSwitch {
    /// Create a new switch
    pub fn new() -> Self {
        Self {
            selected: false,
            disabled: false,
            with_icon: false,
            animation_progress: 0.0,
            pressed: false,
            hovered: false,
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self.animation_progress = if selected { 1.0 } else { 0.0 };
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable icons in the switch
    pub fn with_icon(mut self) -> Self {
        self.with_icon = true;
        self
    }

    /// Get the track color
    pub fn track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.on_surface.with_alpha(0.12);
            } else {
                return theme.surface_container_highest.with_alpha(0.12);
            }
        }

        if self.selected {
            theme.primary
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the track outline color
    pub fn track_outline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        if self.selected {
            Color::NONE
        } else {
            theme.outline
        }
    }

    /// Get the handle (thumb) color
    pub fn handle_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.surface;
            } else {
                return theme.on_surface.with_alpha(0.38);
            }
        }

        if self.selected {
            theme.on_primary
        } else if self.pressed {
            theme.on_surface_variant
        } else if self.hovered {
            theme.on_surface_variant
        } else {
            theme.outline
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.on_surface.with_alpha(0.38);
            } else {
                return theme.surface_container_highest.with_alpha(0.38);
            }
        }

        if self.selected {
            theme.on_primary_container
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the handle size based on state
    pub fn handle_size(&self) -> f32 {
        if self.pressed {
            SWITCH_HANDLE_SIZE_PRESSED
        } else if self.selected || self.with_icon {
            SWITCH_HANDLE_SIZE_SELECTED
        } else {
            SWITCH_HANDLE_SIZE_UNSELECTED
        }
    }

    /// Get the handle position (0.0 to 1.0)
    pub fn handle_position(&self) -> f32 {
        self.animation_progress
    }
}

impl Default for MaterialSwitch {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when switch state changes
#[derive(Event, bevy::prelude::Message)]
pub struct SwitchChangeEvent {
    pub entity: Entity,
    pub selected: bool,
}

/// Switch dimensions
pub const SWITCH_TRACK_WIDTH: f32 = 52.0;
pub const SWITCH_TRACK_HEIGHT: f32 = 32.0;
pub const SWITCH_HANDLE_SIZE_UNSELECTED: f32 = 16.0;
pub const SWITCH_HANDLE_SIZE_SELECTED: f32 = 24.0;
pub const SWITCH_HANDLE_SIZE_PRESSED: f32 = 28.0;

/// System to handle switch interactions
fn switch_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialSwitch),
        (Changed<Interaction>, With<MaterialSwitch>),
    >,
    mut change_events: MessageWriter<SwitchChangeEvent>,
) {
    for (entity, interaction, mut switch) in interaction_query.iter_mut() {
        if switch.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                switch.pressed = true;
                switch.hovered = false;
                switch.selected = !switch.selected;
                change_events.write(SwitchChangeEvent {
                    entity,
                    selected: switch.selected,
                });
            }
            Interaction::Hovered => {
                switch.pressed = false;
                switch.hovered = true;
            }
            Interaction::None => {
                switch.pressed = false;
                switch.hovered = false;
            }
        }
    }
}

/// System to update switch styles
fn switch_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut switches: Query<(&MaterialSwitch, &mut BackgroundColor, &mut BorderColor), Changed<MaterialSwitch>>,
) {
    let Some(theme) = theme else { return };

    for (switch, mut bg_color, mut border_color) in switches.iter_mut() {
        *bg_color = BackgroundColor(switch.track_color(&theme));
        *border_color = BorderColor::all(switch.track_outline_color(&theme));
    }
}

/// Builder for switches
pub struct SwitchBuilder {
    switch: MaterialSwitch,
}

impl SwitchBuilder {
    /// Create a new switch builder
    pub fn new() -> Self {
        Self {
            switch: MaterialSwitch::new(),
        }
    }

    /// Set initial state
    pub fn selected(mut self, selected: bool) -> Self {
        self.switch.selected = selected;
        self.switch.animation_progress = if selected { 1.0 } else { 0.0 };
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.switch.disabled = disabled;
        self
    }

    /// Enable icon display
    pub fn with_icon(mut self) -> Self {
        self.switch.with_icon = true;
        self
    }

    /// Build the switch bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.switch.track_color(theme);
        let border_color = self.switch.track_outline_color(theme);
        let has_border = !self.switch.selected;

        (
            self.switch,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(SWITCH_TRACK_WIDTH),
                height: Val::Px(SWITCH_TRACK_HEIGHT),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(2.0)),
                border: UiRect::all(Val::Px(if has_border { 2.0 } else { 0.0 })),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

impl Default for SwitchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker component for the switch handle
#[derive(Component)]
pub struct SwitchHandle;
