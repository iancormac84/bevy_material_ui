//! Material Design 3 Checkbox component
//!
//! Checkboxes let users select one or more items from a list.
//! Reference: <https://m3.material.io/components/checkbox/overview>
//!
//! # Example
//! ```ignore
//! commands.spawn_checkbox(&theme, CheckboxState::Unchecked, "Option 1");
//! ```

use bevy::prelude::*;

use crate::{
    icons::{ICON_CHECK, ICON_REMOVE, MaterialIconFont},
    motion::{ease_emphasized_decelerate, StateLayer},
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Duration},
};

/// Plugin for the checkbox component
pub struct CheckboxPlugin;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CheckboxChangeEvent>()
            .add_systems(Update, (
                checkbox_interaction_system,
                checkbox_visual_update_system,
                checkbox_animation_system,
            ).chain());
    }
}

/// Checkbox checked state
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CheckboxState {
    /// Unchecked state
    #[default]
    Unchecked,
    /// Checked state
    Checked,
    /// Indeterminate state (partially selected)
    Indeterminate,
}

impl CheckboxState {
    /// Toggle between checked and unchecked
    pub fn toggle(&self) -> Self {
        match self {
            CheckboxState::Unchecked => CheckboxState::Checked,
            CheckboxState::Checked => CheckboxState::Unchecked,
            CheckboxState::Indeterminate => CheckboxState::Checked,
        }
    }

    /// Check if the checkbox is checked
    pub fn is_checked(&self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    /// Check if the checkbox is indeterminate
    pub fn is_indeterminate(&self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }

    /// Get the icon for this state (Material Symbols codepoints)
    pub fn icon(&self) -> Option<char> {
        match self {
            CheckboxState::Unchecked => None,
            CheckboxState::Checked => Some(ICON_CHECK),
            CheckboxState::Indeterminate => Some(ICON_REMOVE),
        }
    }
}

/// Material checkbox component
#[derive(Component)]
pub struct MaterialCheckbox {
    /// Current state
    pub state: CheckboxState,
    /// Whether the checkbox is disabled
    pub disabled: bool,
    /// Whether there's an error
    pub error: bool,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
    /// Animation progress (0.0 to 1.0)
    pub animation_progress: f32,
    /// Whether animating
    pub animating: bool,
    /// Previous state (for animation)
    pub previous_state: CheckboxState,
}

impl MaterialCheckbox {
    /// Create a new checkbox
    pub fn new() -> Self {
        Self {
            state: CheckboxState::default(),
            disabled: false,
            error: false,
            pressed: false,
            hovered: false,
            animation_progress: 1.0,
            animating: false,
            previous_state: CheckboxState::Unchecked,
        }
    }

    /// Set the initial state
    pub fn with_state(mut self, state: CheckboxState) -> Self {
        self.state = state;
        self.previous_state = state;
        self
    }

    /// Set as checked
    pub fn checked(mut self) -> Self {
        self.state = CheckboxState::Checked;
        self.previous_state = CheckboxState::Checked;
        self
    }

    /// Set as indeterminate
    pub fn indeterminate(mut self) -> Self {
        self.state = CheckboxState::Indeterminate;
        self.previous_state = CheckboxState::Indeterminate;
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Get the container color (when checked/indeterminate)
    pub fn container_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        match self.state {
            CheckboxState::Unchecked => Color::NONE,
            CheckboxState::Checked | CheckboxState::Indeterminate => theme.primary,
        }
    }

    /// Get the outline color
    pub fn outline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        match self.state {
            CheckboxState::Unchecked => theme.on_surface_variant,
            CheckboxState::Checked | CheckboxState::Indeterminate => theme.primary,
        }
    }

    /// Get the checkmark/icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.surface;
        }

        if self.error {
            return theme.on_error;
        }

        theme.on_primary
    }

    /// Get the state layer color for hover/press states
    pub fn state_layer_color(&self, theme: &MaterialTheme) -> Color {
        if self.error {
            return theme.error;
        }

        match self.state {
            CheckboxState::Unchecked => theme.on_surface,
            CheckboxState::Checked | CheckboxState::Indeterminate => theme.primary,
        }
    }

    /// Start animation to new state
    fn start_animation(&mut self, new_state: CheckboxState) {
        if self.state != new_state {
            self.previous_state = self.state;
            self.state = new_state;
            self.animation_progress = 0.0;
            self.animating = true;
        }
    }
}

impl Default for MaterialCheckbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when checkbox state changes
#[derive(Event, bevy::prelude::Message)]
pub struct CheckboxChangeEvent {
    pub entity: Entity,
    pub state: CheckboxState,
}

/// Marker for the checkbox visual box
#[derive(Component)]
pub struct CheckboxBox;

/// Marker for the checkmark icon
#[derive(Component)]
pub struct CheckboxIcon;

/// Marker for the state layer (hover/press overlay)
#[derive(Component)]
pub struct CheckboxStateLayer;

/// Checkbox container size
pub const CHECKBOX_SIZE: f32 = 18.0;
/// Checkbox touch target size  
pub const CHECKBOX_TOUCH_TARGET: f32 = 48.0;
/// Checkbox border width
pub const CHECKBOX_BORDER_WIDTH: f32 = 2.0;
/// Checkbox corner radius
pub const CHECKBOX_CORNER_RADIUS: f32 = 2.0;

/// System to handle checkbox interactions
fn checkbox_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialCheckbox),
        (Changed<Interaction>, With<MaterialCheckbox>),
    >,
    mut change_events: MessageWriter<CheckboxChangeEvent>,
) {
    for (entity, interaction, mut checkbox) in interaction_query.iter_mut() {
        if checkbox.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                checkbox.pressed = true;
                checkbox.hovered = false;
                let new_state = checkbox.state.toggle();
                checkbox.start_animation(new_state);
                change_events.write(CheckboxChangeEvent {
                    entity,
                    state: checkbox.state,
                });
            }
            Interaction::Hovered => {
                checkbox.pressed = false;
                checkbox.hovered = true;
            }
            Interaction::None => {
                checkbox.pressed = false;
                checkbox.hovered = false;
            }
        }
    }
}

/// System to update checkbox visual styles when state changes
fn checkbox_visual_update_system(
    theme: Option<Res<MaterialTheme>>,
    icon_font: Option<Res<MaterialIconFont>>,
    checkboxes: Query<(Entity, &MaterialCheckbox, &Children), Changed<MaterialCheckbox>>,
    mut boxes: Query<(&mut BackgroundColor, &mut BorderColor), With<CheckboxBox>>,
    mut icons: Query<(&mut Text, &mut TextFont, &mut TextColor), With<CheckboxIcon>>,
    mut state_layers: Query<&mut StateLayer, With<CheckboxStateLayer>>,
    children_query: Query<&Children>,
) {
    let Some(theme) = theme else { return };

    for (_entity, checkbox, children) in checkboxes.iter() {
        // Find checkbox box and icon through children
        for child in children.iter() {
            // Check if this child is the state layer
            if let Ok(mut layer) = state_layers.get_mut(child) {
                layer.color = checkbox.state_layer_color(&theme);
                if checkbox.pressed {
                    layer.set_pressed();
                } else if checkbox.hovered {
                    layer.set_hovered();
                } else {
                    layer.clear();
                }
            }

            // Navigate to checkbox box
            if let Ok(grandchildren) = children_query.get(child) {
                for grandchild in grandchildren.iter() {
                    // Update box colors
                    if let Ok((mut bg, mut border)) = boxes.get_mut(grandchild) {
                        bg.0 = checkbox.container_color(&theme);
                        *border = BorderColor::all(checkbox.outline_color(&theme));
                    }

                    // Update icon
                    if let Ok(great_grandchildren) = children_query.get(grandchild) {
                        for ggc in great_grandchildren.iter() {
                            if let Ok((mut text, mut text_font, mut color)) = icons.get_mut(ggc) {
                                if let Some(icon) = checkbox.state.icon() {
                                    **text = icon.to_string();
                                    color.0 = checkbox.icon_color(&theme);
                                    // Set the Material Symbols font if available
                                    if let Some(ref font) = icon_font {
                                        text_font.font = font.0.clone();
                                    }
                                } else {
                                    **text = String::new();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// System to handle checkbox animations
fn checkbox_animation_system(
    time: Res<Time>,
    mut checkboxes: Query<&mut MaterialCheckbox>,
) {
    let dt = time.delta_secs();
    let animation_duration = Duration::MEDIUM2;

    for mut checkbox in checkboxes.iter_mut() {
        if checkbox.animating {
            checkbox.animation_progress += dt / animation_duration;
            
            if checkbox.animation_progress >= 1.0 {
                checkbox.animation_progress = 1.0;
                checkbox.animating = false;
            }
            
            // Apply easing
            let _eased = ease_emphasized_decelerate(checkbox.animation_progress);
        }
    }
}

/// Builder for checkboxes
pub struct CheckboxBuilder {
    checkbox: MaterialCheckbox,
}

impl CheckboxBuilder {
    /// Create a new checkbox builder
    pub fn new() -> Self {
        Self {
            checkbox: MaterialCheckbox::new(),
        }
    }

    /// Set initial state
    pub fn state(mut self, state: CheckboxState) -> Self {
        self.checkbox.state = state;
        self.checkbox.previous_state = state;
        self
    }

    /// Start checked
    pub fn checked(self) -> Self {
        self.state(CheckboxState::Checked)
    }

    /// Start indeterminate
    pub fn indeterminate(self) -> Self {
        self.state(CheckboxState::Indeterminate)
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.checkbox.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.checkbox.error = error;
        self
    }

    /// Build the checkbox component (just the component, not visuals)
    pub fn build(self) -> MaterialCheckbox {
        self.checkbox
    }

    /// Build a complete checkbox with visuals
    /// 
    /// This spawns a fully-styled checkbox entity with:
    /// - Touch target area (48x48)
    /// - State layer for hover/press feedback
    /// - Visual checkbox box (18x18)
    /// - Checkmark icon
    pub fn spawn(self, commands: &mut Commands, theme: &MaterialTheme) -> Entity {
        let checkbox = self.checkbox;
        let bg_color = checkbox.container_color(theme);
        let border_color = checkbox.outline_color(theme);
        let icon_color = checkbox.icon_color(theme);
        let icon_char = checkbox.state.icon();
        let state_layer_color = checkbox.state_layer_color(theme);

        commands.spawn((
            checkbox,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(CHECKBOX_TOUCH_TARGET),
                height: Val::Px(CHECKBOX_TOUCH_TARGET),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )).with_children(|parent| {
            // State layer (for hover/press effects)
            parent.spawn((
                CheckboxStateLayer,
                StateLayer::new(state_layer_color),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(20.0)),
            )).with_children(|state_layer_parent| {
                // Checkbox box (visual element)
                state_layer_parent.spawn((
                    CheckboxBox,
                    Node {
                        width: Val::Px(CHECKBOX_SIZE),
                        height: Val::Px(CHECKBOX_SIZE),
                        border: UiRect::all(Val::Px(CHECKBOX_BORDER_WIDTH)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    BorderColor::all(border_color),
                    BorderRadius::all(Val::Px(CHECKBOX_CORNER_RADIUS)),
                )).with_children(|box_parent| {
                    // Checkmark icon
                    box_parent.spawn((
                        CheckboxIcon,
                        Text::new(icon_char.map(|c| c.to_string()).unwrap_or_default()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(icon_color),
                    ));
                });
            });
        }).id()
    }
}

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait to spawn checkboxes easily
pub trait SpawnCheckbox {
    /// Spawn a checkbox with a label
    fn spawn_checkbox(
        &mut self,
        theme: &MaterialTheme,
        state: CheckboxState,
        label: &str,
    ) -> Entity;
}

impl SpawnCheckbox for Commands<'_, '_> {
    fn spawn_checkbox(
        &mut self,
        theme: &MaterialTheme,
        state: CheckboxState,
        label: &str,
    ) -> Entity {
        let label_color = theme.on_surface;
        let label_text = label.to_string();
        let checkbox = CheckboxBuilder::new().state(state).build();
        let bg_color = checkbox.container_color(theme);
        let border_color = checkbox.outline_color(theme);
        let icon_color = checkbox.icon_color(theme);
        let icon_char = checkbox.state.icon();
        let state_layer_color = checkbox.state_layer_color(theme);

        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        }).with_children(|row| {
            // Checkbox
            row.spawn((
                checkbox,
                Button,
                RippleHost::new(),
                Node {
                    width: Val::Px(CHECKBOX_TOUCH_TARGET),
                    height: Val::Px(CHECKBOX_TOUCH_TARGET),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            )).with_children(|parent| {
                // State layer
                parent.spawn((
                    CheckboxStateLayer,
                    StateLayer::new(state_layer_color),
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
                )).with_children(|state_layer_parent| {
                    // Checkbox box
                    state_layer_parent.spawn((
                        CheckboxBox,
                        Node {
                            width: Val::Px(CHECKBOX_SIZE),
                            height: Val::Px(CHECKBOX_SIZE),
                            border: UiRect::all(Val::Px(CHECKBOX_BORDER_WIDTH)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(bg_color),
                        BorderColor::all(border_color),
                        BorderRadius::all(Val::Px(CHECKBOX_CORNER_RADIUS)),
                    )).with_children(|box_parent| {
                        // Checkmark
                        box_parent.spawn((
                            CheckboxIcon,
                            Text::new(icon_char.map(|c| c.to_string()).unwrap_or_default()),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(icon_color),
                        ));
                    });
                });
            });

            // Label
            row.spawn((
                Text::new(label_text),
                TextFont { font_size: 14.0, ..default() },
                TextColor(label_color),
            ));
        }).id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_state_toggle() {
        assert_eq!(CheckboxState::Unchecked.toggle(), CheckboxState::Checked);
        assert_eq!(CheckboxState::Checked.toggle(), CheckboxState::Unchecked);
        assert_eq!(CheckboxState::Indeterminate.toggle(), CheckboxState::Checked);
    }

    #[test]
    fn test_checkbox_state_icon() {
        assert!(CheckboxState::Unchecked.icon().is_none());
        assert_eq!(CheckboxState::Checked.icon(), Some(ICON_CHECK));
        assert_eq!(CheckboxState::Indeterminate.icon(), Some(ICON_REMOVE));
    }

    #[test]
    fn test_checkbox_builder() {
        let checkbox = CheckboxBuilder::new()
            .checked()
            .disabled(true)
            .error(true)
            .build();
        
        assert!(checkbox.state.is_checked());
        assert!(checkbox.disabled);
        assert!(checkbox.error);
    }
}
