//! Material Design 3 Radio Button component
//!
//! Radio buttons let users select one option from a set.
//! Reference: <https://m3.material.io/components/radio-button/overview>

use bevy::prelude::*;

use crate::{
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::CornerRadius,
};

/// Plugin for the radio button component
pub struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RadioChangeEvent>()
            .add_systems(Update, (radio_interaction_system, radio_group_system, radio_style_system));
    }
}

/// Material radio button component
#[derive(Component)]
pub struct MaterialRadio {
    /// Whether this radio is selected
    pub selected: bool,
    /// Whether the radio is disabled
    pub disabled: bool,
    /// The group this radio belongs to
    pub group: Option<String>,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialRadio {
    /// Create a new radio button
    pub fn new() -> Self {
        Self {
            selected: false,
            disabled: false,
            group: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the radio group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Get the outer circle color
    pub fn outer_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.selected {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the inner dot color
    pub fn inner_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        theme.primary
    }

    /// Get the state layer color
    pub fn state_layer_color(&self, theme: &MaterialTheme) -> Color {
        if self.selected {
            theme.primary
        } else {
            theme.on_surface
        }
    }
}

impl Default for MaterialRadio {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to define a radio group
#[derive(Component)]
pub struct RadioGroup {
    /// Group identifier
    pub name: String,
    /// Currently selected value (entity ID)
    pub selected: Option<Entity>,
}

impl RadioGroup {
    /// Create a new radio group
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selected: None,
        }
    }
}

/// Event when radio selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct RadioChangeEvent {
    pub entity: Entity,
    pub group: Option<String>,
    pub selected: bool,
}

/// Radio button size
pub const RADIO_SIZE: f32 = 20.0;
/// Radio inner dot size when selected
pub const RADIO_DOT_SIZE: f32 = 10.0;
/// Radio touch target size
pub const RADIO_TOUCH_TARGET: f32 = 48.0;

/// System to handle radio interactions
fn radio_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialRadio),
        (Changed<Interaction>, With<MaterialRadio>),
    >,
    mut change_events: MessageWriter<RadioChangeEvent>,
) {
    for (entity, interaction, mut radio) in interaction_query.iter_mut() {
        if radio.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                radio.pressed = true;
                radio.hovered = false;
                
                // Only fire event if not already selected
                if !radio.selected {
                    radio.selected = true;
                    change_events.write(RadioChangeEvent {
                        entity,
                        group: radio.group.clone(),
                        selected: true,
                    });
                }
            }
            Interaction::Hovered => {
                radio.pressed = false;
                radio.hovered = true;
            }
            Interaction::None => {
                radio.pressed = false;
                radio.hovered = false;
            }
        }
    }
}

/// System to handle radio group exclusivity
fn radio_group_system(
    mut change_events: MessageReader<RadioChangeEvent>,
    mut radios: Query<(Entity, &mut MaterialRadio)>,
) {
    for event in change_events.read() {
        if let Some(ref group_name) = event.group {
            // Deselect all other radios in the same group
            for (entity, mut radio) in radios.iter_mut() {
                if entity != event.entity {
                    if let Some(ref radio_group) = radio.group {
                        if radio_group == group_name && radio.selected {
                            radio.selected = false;
                        }
                    }
                }
            }
        }
    }
}

/// System to update radio styles
fn radio_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut radios: Query<(&MaterialRadio, &mut BorderColor), Changed<MaterialRadio>>,
) {
    let Some(theme) = theme else { return };

    for (radio, mut border_color) in radios.iter_mut() {
        *border_color = BorderColor::all(radio.outer_color(&theme));
    }
}

/// Builder for radio buttons
pub struct RadioBuilder {
    radio: MaterialRadio,
}

impl RadioBuilder {
    /// Create a new radio builder
    pub fn new() -> Self {
        Self {
            radio: MaterialRadio::new(),
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.radio.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.radio.disabled = disabled;
        self
    }

    /// Set the radio group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.radio.group = Some(group.into());
        self
    }

    /// Build the radio bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _border_color = self.radio.outer_color(theme);

        (
            self.radio,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(RADIO_TOUCH_TARGET),
                height: Val::Px(RADIO_TOUCH_TARGET),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

impl Default for RadioBuilder {
    fn default() -> Self {
        Self::new()
    }
}
