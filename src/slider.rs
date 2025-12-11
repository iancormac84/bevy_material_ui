//! Material Design 3 Slider component
//!
//! Sliders allow users to select a value from a range.
//! Reference: <https://m3.material.io/components/sliders/overview>

use bevy::prelude::*;

use crate::theme::MaterialTheme;

/// Plugin for the slider component
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SliderChangeEvent>()
            .add_systems(Update, (slider_interaction_system, slider_style_system));
    }
}

/// Slider variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SliderVariant {
    /// Continuous slider - any value in range
    #[default]
    Continuous,
    /// Discrete slider - snaps to steps
    Discrete,
}

/// Material slider component
#[derive(Component)]
pub struct MaterialSlider {
    /// Current value
    pub value: f32,
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
    /// Step size for discrete sliders
    pub step: Option<f32>,
    /// Whether to show tick marks
    pub show_ticks: bool,
    /// Whether to show value label
    pub show_label: bool,
    /// Whether the slider is disabled
    pub disabled: bool,
    /// Interaction states
    pub dragging: bool,
    pub hovered: bool,
}

impl MaterialSlider {
    /// Create a new slider
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            value: min,
            min,
            max,
            step: None,
            show_ticks: false,
            show_label: false,
            disabled: false,
            dragging: false,
            hovered: false,
        }
    }

    /// Set the initial value
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Set the step size (makes it discrete)
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Show tick marks
    pub fn show_ticks(mut self) -> Self {
        self.show_ticks = true;
        self
    }

    /// Show value label
    pub fn show_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Get the normalized value (0.0 to 1.0)
    pub fn normalized_value(&self) -> f32 {
        (self.value - self.min) / (self.max - self.min)
    }

    /// Set value from normalized (0.0 to 1.0)
    pub fn set_from_normalized(&mut self, normalized: f32) {
        let raw_value = self.min + normalized * (self.max - self.min);
        self.value = if let Some(step) = self.step {
            (raw_value / step).round() * step
        } else {
            raw_value
        };
        self.value = self.value.clamp(self.min, self.max);
    }

    /// Get the active track color
    pub fn active_track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.primary
        }
    }

    /// Get the inactive track color
    pub fn inactive_track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.12)
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the handle (thumb) color
    pub fn handle_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.primary
        }
    }

    /// Get the tick mark color for active section
    pub fn active_tick_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_primary
        }
    }

    /// Get the tick mark color for inactive section
    pub fn inactive_tick_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the label background color
    pub fn label_background_color(&self, theme: &MaterialTheme) -> Color {
        theme.primary
    }

    /// Get the label text color
    pub fn label_text_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_primary
    }
}

impl Default for MaterialSlider {
    fn default() -> Self {
        Self::new(0.0, 100.0)
    }
}

/// Event when slider value changes
#[derive(Event, bevy::prelude::Message)]
pub struct SliderChangeEvent {
    pub entity: Entity,
    pub value: f32,
}

/// Slider dimensions
pub const SLIDER_TRACK_HEIGHT: f32 = 4.0;
pub const SLIDER_TRACK_HEIGHT_ACTIVE: f32 = 6.0;
pub const SLIDER_HANDLE_SIZE: f32 = 20.0;
pub const SLIDER_HANDLE_SIZE_PRESSED: f32 = 24.0;
pub const SLIDER_TICK_SIZE: f32 = 4.0;
pub const SLIDER_LABEL_HEIGHT: f32 = 28.0;

/// System to handle slider interactions
fn slider_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialSlider, &ComputedNode, &GlobalTransform),
        With<MaterialSlider>,
    >,
    mut change_events: MessageWriter<SliderChangeEvent>,
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor_position) = window.cursor_position() else { return };

    for (entity, interaction, mut slider, computed_node, transform) in interaction_query.iter_mut() {
        if slider.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                slider.dragging = true;
                slider.hovered = false;
            }
            Interaction::Hovered => {
                if !mouse_button.pressed(MouseButton::Left) {
                    slider.dragging = false;
                }
                slider.hovered = true;
            }
            Interaction::None => {
                if !mouse_button.pressed(MouseButton::Left) {
                    slider.dragging = false;
                }
                slider.hovered = false;
            }
        }

        // Handle dragging
        if slider.dragging {
            let slider_pos = transform.translation().xy();
            let slider_size = computed_node.size();
            
            let relative_x = cursor_position.x - (slider_pos.x - slider_size.x / 2.0);
            let normalized = (relative_x / slider_size.x).clamp(0.0, 1.0);
            
            let old_value = slider.value;
            slider.set_from_normalized(normalized);
            
            if (slider.value - old_value).abs() > f32::EPSILON {
                change_events.write(SliderChangeEvent {
                    entity,
                    value: slider.value,
                });
            }
        }
    }
}

/// System to update slider styles
fn slider_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut sliders: Query<(&MaterialSlider, &mut BackgroundColor), Changed<MaterialSlider>>,
) {
    let Some(theme) = theme else { return };

    for (slider, mut bg_color) in sliders.iter_mut() {
        *bg_color = BackgroundColor(slider.inactive_track_color(&theme));
    }
}

/// Builder for sliders
pub struct SliderBuilder {
    slider: MaterialSlider,
    width: Val,
}

impl SliderBuilder {
    /// Create a new slider builder
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            slider: MaterialSlider::new(min, max),
            width: Val::Px(200.0),
        }
    }

    /// Set initial value
    pub fn value(mut self, value: f32) -> Self {
        self.slider.value = value.clamp(self.slider.min, self.slider.max);
        self
    }

    /// Set step size
    pub fn step(mut self, step: f32) -> Self {
        self.slider.step = Some(step);
        self
    }

    /// Show tick marks
    pub fn ticks(mut self) -> Self {
        self.slider.show_ticks = true;
        self
    }

    /// Show value label
    pub fn label(mut self) -> Self {
        self.slider.show_label = true;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.slider.disabled = disabled;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the slider bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _bg_color = self.slider.inactive_track_color(theme);

        (
            self.slider,
            Button,
            Node {
                width: self.width,
                height: Val::Px(SLIDER_HANDLE_SIZE + 8.0), // Extra space for handle
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
        )
    }
}

/// Marker component for slider track
#[derive(Component)]
pub struct SliderTrack;

/// Marker component for slider active track
#[derive(Component)]
pub struct SliderActiveTrack;

/// Marker component for slider handle
#[derive(Component)]
pub struct SliderHandle;

/// Marker component for slider value label
#[derive(Component)]
pub struct SliderLabel;
