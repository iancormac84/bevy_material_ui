//! Material Design 3 Tooltip component
//!
//! Tooltips display brief labels or messages on hover or focus.
//! They help identify or add information to elements.
//!
//! Reference: <https://m3.material.io/components/tooltips/overview>

use bevy::prelude::*;

use crate::{
    motion::{ease_standard_decelerate, ease_standard_accelerate},
    theme::MaterialTheme,
    tokens::{CornerRadius, Duration, Spacing},
};

/// Plugin for the tooltip component
pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            tooltip_hover_system,
            tooltip_animation_system,
            tooltip_position_system,
        ));
    }
}

// ============================================================================
// Types
// ============================================================================

/// Tooltip variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TooltipVariant {
    /// Plain tooltip - Simple text label
    #[default]
    Plain,
    /// Rich tooltip - Can contain title, supporting text, and actions
    Rich,
}

/// Tooltip position relative to anchor
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TooltipPosition {
    /// Above the anchor
    #[default]
    Top,
    /// Below the anchor
    Bottom,
    /// Left of the anchor
    Left,
    /// Right of the anchor
    Right,
}

// ============================================================================
// Components
// ============================================================================

/// A tooltip trigger - attach to elements that should show tooltips
#[derive(Component)]
pub struct TooltipTrigger {
    /// The tooltip text (for plain tooltips)
    pub text: String,
    /// Tooltip variant
    pub variant: TooltipVariant,
    /// Preferred position
    pub position: TooltipPosition,
    /// Delay before showing (in seconds)
    pub delay: f32,
    /// Current hover time
    pub hover_time: f32,
    /// Whether currently hovered
    pub hovered: bool,
    /// Associated tooltip entity (if spawned)
    pub tooltip_entity: Option<Entity>,
}

impl TooltipTrigger {
    /// Create a new tooltip trigger
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: TooltipVariant::default(),
            position: TooltipPosition::default(),
            delay: TOOLTIP_DELAY_DEFAULT,
            hover_time: 0.0,
            hovered: false,
            tooltip_entity: None,
        }
    }

    /// Set the position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the delay
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.delay = delay;
        self
    }

    /// Create a rich tooltip trigger
    pub fn rich(mut self) -> Self {
        self.variant = TooltipVariant::Rich;
        self
    }

    /// Position above
    pub fn top(self) -> Self {
        self.with_position(TooltipPosition::Top)
    }

    /// Position below
    pub fn bottom(self) -> Self {
        self.with_position(TooltipPosition::Bottom)
    }

    /// Position left
    pub fn left(self) -> Self {
        self.with_position(TooltipPosition::Left)
    }

    /// Position right
    pub fn right(self) -> Self {
        self.with_position(TooltipPosition::Right)
    }
}

/// The tooltip popup component
#[derive(Component)]
pub struct Tooltip {
    /// Text content
    pub text: String,
    /// Variant
    pub variant: TooltipVariant,
    /// Animation state
    pub animation_state: TooltipAnimationState,
    /// Animation progress (0.0 = hidden, 1.0 = visible)
    pub animation_progress: f32,
    /// The anchor entity this tooltip is for
    pub anchor: Entity,
    /// Position relative to anchor
    pub position: TooltipPosition,
}

/// Tooltip animation state
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TooltipAnimationState {
    /// Fading in
    #[default]
    Entering,
    /// Fully visible
    Visible,
    /// Fading out
    Exiting,
    /// Hidden
    Hidden,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>, anchor: Entity) -> Self {
        Self {
            text: text.into(),
            variant: TooltipVariant::Plain,
            animation_state: TooltipAnimationState::Entering,
            animation_progress: 0.0,
            anchor,
            position: TooltipPosition::Top,
        }
    }

    /// Set variant
    pub fn with_variant(mut self, variant: TooltipVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Start dismissing
    pub fn dismiss(&mut self) {
        if self.animation_state != TooltipAnimationState::Exiting {
            self.animation_state = TooltipAnimationState::Exiting;
        }
    }

    /// Check if hidden
    pub fn is_hidden(&self) -> bool {
        self.animation_state == TooltipAnimationState::Hidden
    }

    /// Get the background color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        match self.variant {
            TooltipVariant::Plain => theme.inverse_surface,
            TooltipVariant::Rich => theme.surface_container,
        }
    }

    /// Get the text color
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        match self.variant {
            TooltipVariant::Plain => theme.inverse_on_surface,
            TooltipVariant::Rich => theme.on_surface_variant,
        }
    }
}

/// Rich tooltip with additional content
#[derive(Component)]
pub struct RichTooltip {
    /// Title text
    pub title: Option<String>,
    /// Supporting text
    pub supporting_text: String,
    /// Action text (optional)
    pub action: Option<String>,
}

impl RichTooltip {
    /// Create a new rich tooltip
    pub fn new(supporting_text: impl Into<String>) -> Self {
        Self {
            title: None,
            supporting_text: supporting_text.into(),
            action: None,
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the action
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

/// Marker for tooltip text
#[derive(Component)]
pub struct TooltipText;

// ============================================================================
// Dimensions
// ============================================================================

/// Plain tooltip height
pub const TOOLTIP_HEIGHT_PLAIN: f32 = 24.0;
/// Rich tooltip minimum height
pub const TOOLTIP_HEIGHT_RICH_MIN: f32 = 40.0;
/// Tooltip horizontal padding (plain)
pub const TOOLTIP_PADDING_PLAIN: f32 = 8.0;
/// Tooltip padding (rich)
pub const TOOLTIP_PADDING_RICH: f32 = 12.0;
/// Maximum tooltip width
pub const TOOLTIP_MAX_WIDTH: f32 = 200.0;
/// Rich tooltip max width
pub const TOOLTIP_MAX_WIDTH_RICH: f32 = 320.0;
/// Offset from anchor
pub const TOOLTIP_OFFSET: f32 = 8.0;
/// Default delay before showing
pub const TOOLTIP_DELAY_DEFAULT: f32 = 0.5;
/// Short delay (for experienced users)
pub const TOOLTIP_DELAY_SHORT: f32 = 0.15;

// ============================================================================
// Builder
// ============================================================================

/// Builder for tooltip triggers
pub struct TooltipTriggerBuilder {
    trigger: TooltipTrigger,
}

impl TooltipTriggerBuilder {
    /// Create a new builder
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            trigger: TooltipTrigger::new(text),
        }
    }

    /// Set position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.trigger.position = position;
        self
    }

    /// Set delay
    pub fn delay(mut self, delay: f32) -> Self {
        self.trigger.delay = delay;
        self
    }

    /// Build the trigger component
    pub fn build(self) -> TooltipTrigger {
        self.trigger
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a plain tooltip
pub fn spawn_tooltip(
    commands: &mut Commands,
    theme: &MaterialTheme,
    tooltip: Tooltip,
) -> Entity {
    let text = tooltip.text.clone();
    let text_color = tooltip.text_color(theme);
    let bg_color = tooltip.background_color(theme);

    commands
        .spawn((
            tooltip,
            Node {
                position_type: PositionType::Absolute,
                min_height: Val::Px(TOOLTIP_HEIGHT_PLAIN),
                max_width: Val::Px(TOOLTIP_MAX_WIDTH),
                padding: UiRect::axes(Val::Px(TOOLTIP_PADDING_PLAIN), Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // Initial position will be set by position system
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
            GlobalZIndex(1000), // Ensure tooltips are on top
        ))
        .with_children(|parent| {
            parent.spawn((
                TooltipText,
                Text::new(text),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        })
        .id()
}

/// Spawn a rich tooltip
pub fn spawn_rich_tooltip(
    commands: &mut Commands,
    theme: &MaterialTheme,
    tooltip: Tooltip,
    rich: RichTooltip,
) -> Entity {
    let text_color = tooltip.text_color(theme);
    let bg_color = tooltip.background_color(theme);

    commands
        .spawn((
            tooltip,
            rich,
            Node {
                position_type: PositionType::Absolute,
                min_height: Val::Px(TOOLTIP_HEIGHT_RICH_MIN),
                max_width: Val::Px(TOOLTIP_MAX_WIDTH_RICH),
                padding: UiRect::all(Val::Px(TOOLTIP_PADDING_RICH)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(Spacing::EXTRA_SMALL),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
            GlobalZIndex(1000),
        ))
        .with_children(|parent| {
            // Title (if present) would go here
            // Supporting text
            parent.spawn((
                TooltipText,
                Text::new(""), // Rich tooltip content handled separately
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        })
        .id()
}

// ============================================================================
// Systems
// ============================================================================

/// System to handle tooltip hover triggers
fn tooltip_hover_system(
    mut commands: Commands,
    time: Res<Time>,
    theme: Option<Res<MaterialTheme>>,
    mut triggers: Query<(Entity, &Interaction, &mut TooltipTrigger)>,
    mut tooltips: Query<&mut Tooltip>,
) {
    let Some(theme) = theme else { return };

    for (entity, interaction, mut trigger) in triggers.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                if !trigger.hovered {
                    trigger.hovered = true;
                    trigger.hover_time = 0.0;
                }

                trigger.hover_time += time.delta_secs();

                // Show tooltip after delay
                if trigger.hover_time >= trigger.delay && trigger.tooltip_entity.is_none() {
                    let tooltip = Tooltip::new(&trigger.text, entity)
                        .with_position(trigger.position);
                    let tooltip_entity = spawn_tooltip(&mut commands, &theme, tooltip);
                    trigger.tooltip_entity = Some(tooltip_entity);
                }
            }
            Interaction::None | Interaction::Pressed => {
                if trigger.hovered {
                    trigger.hovered = false;
                    trigger.hover_time = 0.0;

                    // Dismiss tooltip
                    if let Some(tooltip_entity) = trigger.tooltip_entity {
                        if let Ok(mut tooltip) = tooltips.get_mut(tooltip_entity) {
                            tooltip.dismiss();
                        }
                        trigger.tooltip_entity = None;
                    }
                }
            }
        }
    }
}

/// System to animate tooltips
fn tooltip_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tooltips: Query<(Entity, &mut Tooltip, &mut BackgroundColor)>,
) {
    for (entity, mut tooltip, mut bg_color) in tooltips.iter_mut() {
        let dt = time.delta_secs();

        match tooltip.animation_state {
            TooltipAnimationState::Entering => {
                tooltip.animation_progress += dt / Duration::SHORT3;
                if tooltip.animation_progress >= 1.0 {
                    tooltip.animation_progress = 1.0;
                    tooltip.animation_state = TooltipAnimationState::Visible;
                }

                let alpha = ease_standard_decelerate(tooltip.animation_progress);
                bg_color.0 = bg_color.0.with_alpha(alpha);
            }
            TooltipAnimationState::Visible => {
                // Nothing to do
            }
            TooltipAnimationState::Exiting => {
                tooltip.animation_progress -= dt / Duration::SHORT2;
                if tooltip.animation_progress <= 0.0 {
                    tooltip.animation_progress = 0.0;
                    tooltip.animation_state = TooltipAnimationState::Hidden;
                }

                let alpha = ease_standard_accelerate(tooltip.animation_progress);
                bg_color.0 = bg_color.0.with_alpha(alpha);
            }
            TooltipAnimationState::Hidden => {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// System to position tooltips relative to their anchors
fn tooltip_position_system(
    mut tooltips: Query<(&Tooltip, &mut Node)>,
    anchors: Query<(&GlobalTransform, &ComputedNode)>,
) {
    for (tooltip, mut node) in tooltips.iter_mut() {
        if let Ok((transform, computed)) = anchors.get(tooltip.anchor) {
            let anchor_pos = transform.translation();
            let anchor_size = computed.size();

            let (top, left) = match tooltip.position {
                TooltipPosition::Top => (
                    anchor_pos.y - TOOLTIP_OFFSET - TOOLTIP_HEIGHT_PLAIN,
                    anchor_pos.x + anchor_size.x / 2.0,
                ),
                TooltipPosition::Bottom => (
                    anchor_pos.y + anchor_size.y + TOOLTIP_OFFSET,
                    anchor_pos.x + anchor_size.x / 2.0,
                ),
                TooltipPosition::Left => (
                    anchor_pos.y + anchor_size.y / 2.0 - TOOLTIP_HEIGHT_PLAIN / 2.0,
                    anchor_pos.x - TOOLTIP_OFFSET,
                ),
                TooltipPosition::Right => (
                    anchor_pos.y + anchor_size.y / 2.0 - TOOLTIP_HEIGHT_PLAIN / 2.0,
                    anchor_pos.x + anchor_size.x + TOOLTIP_OFFSET,
                ),
            };

            node.top = Val::Px(top);
            node.left = Val::Px(left);
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_trigger_creation() {
        let trigger = TooltipTrigger::new("Help text");
        assert_eq!(trigger.text, "Help text");
        assert_eq!(trigger.position, TooltipPosition::Top);
        assert!((trigger.delay - TOOLTIP_DELAY_DEFAULT).abs() < 0.001);
    }

    #[test]
    fn test_tooltip_positions() {
        let trigger = TooltipTrigger::new("Test")
            .bottom()
            .with_delay(0.2);

        assert_eq!(trigger.position, TooltipPosition::Bottom);
        assert!((trigger.delay - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_tooltip_dismiss() {
        // Can't test without ECS, but we can test the state machine
        let mut tooltip = Tooltip::new("Test", Entity::PLACEHOLDER);
        assert_eq!(tooltip.animation_state, TooltipAnimationState::Entering);

        tooltip.dismiss();
        assert_eq!(tooltip.animation_state, TooltipAnimationState::Exiting);
    }

    #[test]
    fn test_rich_tooltip() {
        let rich = RichTooltip::new("Supporting text")
            .with_title("Title")
            .with_action("Learn more");

        assert_eq!(rich.title, Some("Title".to_string()));
        assert_eq!(rich.supporting_text, "Supporting text");
        assert_eq!(rich.action, Some("Learn more".to_string()));
    }
}
