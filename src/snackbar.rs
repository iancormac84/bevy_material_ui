//! Material Design 3 Snackbar component
//!
//! Snackbars provide brief messages about app processes at the bottom of the screen.
//! They can contain an optional action.
//!
//! Reference: <https://m3.material.io/components/snackbar/overview>

use bevy::prelude::*;

use crate::{
    motion::{ease_standard_decelerate, ease_standard_accelerate},
    theme::MaterialTheme,
    tokens::{CornerRadius, Duration, Spacing},
};

/// Plugin for the snackbar component
pub struct SnackbarPlugin;

impl Plugin for SnackbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShowSnackbar>()
            .add_message::<DismissSnackbar>()
            .add_message::<SnackbarActionEvent>()
            .init_resource::<SnackbarQueue>()
            .add_systems(Update, (
                snackbar_queue_system,
                snackbar_animation_system,
                snackbar_timeout_system,
                snackbar_action_system,
            ));
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event to show a snackbar
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ShowSnackbar {
    /// The message to display
    pub message: String,
    /// Optional action button text
    pub action: Option<String>,
    /// Duration to show (None = use default)
    pub duration: Option<f32>,
    /// Whether this snackbar can be dismissed by swiping
    pub dismissible: bool,
}

impl ShowSnackbar {
    /// Create a simple snackbar with just a message
    pub fn message(text: impl Into<String>) -> Self {
        Self {
            message: text.into(),
            action: None,
            duration: None,
            dismissible: true,
        }
    }

    /// Create a snackbar with an action button
    pub fn with_action(text: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            message: text.into(),
            action: Some(action.into()),
            duration: None,
            dismissible: true,
        }
    }

    /// Set the duration
    pub fn duration(mut self, seconds: f32) -> Self {
        self.duration = Some(seconds);
        self
    }

    /// Set whether dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}

/// Event to dismiss the current snackbar
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct DismissSnackbar;

/// Event fired when a snackbar action is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct SnackbarActionEvent {
    /// The snackbar entity
    pub entity: Entity,
    /// The action text
    pub action: String,
}

// ============================================================================
// Resources
// ============================================================================

/// Queue of pending snackbars
#[derive(Resource, Default)]
pub struct SnackbarQueue {
    /// Queued snackbars waiting to be shown
    pub queue: Vec<ShowSnackbar>,
    /// Currently active snackbar entity
    pub active: Option<Entity>,
}

// ============================================================================
// Components
// ============================================================================

/// Snackbar container component
#[derive(Component)]
pub struct Snackbar {
    /// The message text
    pub message: String,
    /// Optional action text
    pub action: Option<String>,
    /// Duration to display (in seconds)
    pub duration: f32,
    /// Whether dismissible
    pub dismissible: bool,
    /// Current animation state
    pub animation_state: SnackbarAnimationState,
    /// Time remaining before auto-dismiss
    pub time_remaining: f32,
    /// Animation progress (0.0 = hidden, 1.0 = visible)
    pub animation_progress: f32,
}

/// Animation state for snackbar
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SnackbarAnimationState {
    /// Snackbar is entering
    #[default]
    Entering,
    /// Snackbar is visible
    Visible,
    /// Snackbar is exiting
    Exiting,
    /// Snackbar has been dismissed
    Dismissed,
}

impl Snackbar {
    /// Default duration for snackbars (4 seconds)
    pub const DEFAULT_DURATION: f32 = 4.0;
    /// Short duration (2 seconds)
    pub const SHORT_DURATION: f32 = 2.0;
    /// Long duration (10 seconds)
    pub const LONG_DURATION: f32 = 10.0;
    /// Indefinite duration (must be manually dismissed)
    pub const INDEFINITE: f32 = f32::MAX;

    /// Create a new snackbar from a ShowSnackbar event
    pub fn from_event(event: &ShowSnackbar) -> Self {
        Self {
            message: event.message.clone(),
            action: event.action.clone(),
            duration: event.duration.unwrap_or(Self::DEFAULT_DURATION),
            dismissible: event.dismissible,
            animation_state: SnackbarAnimationState::Entering,
            time_remaining: event.duration.unwrap_or(Self::DEFAULT_DURATION),
            animation_progress: 0.0,
        }
    }

    /// Start the exit animation
    pub fn dismiss(&mut self) {
        if self.animation_state != SnackbarAnimationState::Exiting {
            self.animation_state = SnackbarAnimationState::Exiting;
            self.animation_progress = 1.0;
        }
    }

    /// Check if the snackbar has been fully dismissed
    pub fn is_dismissed(&self) -> bool {
        self.animation_state == SnackbarAnimationState::Dismissed
    }
}

/// Marker for snackbar action button
#[derive(Component)]
pub struct SnackbarAction;

/// Marker for snackbar message text
#[derive(Component)]
pub struct SnackbarMessage;

/// Snackbar host - container that holds snackbars
#[derive(Component)]
pub struct SnackbarHost;

// ============================================================================
// Dimensions
// ============================================================================

/// Minimum width for snackbar
pub const SNACKBAR_MIN_WIDTH: f32 = 288.0;
/// Maximum width for snackbar
pub const SNACKBAR_MAX_WIDTH: f32 = 560.0;
/// Height for single-line snackbar
pub const SNACKBAR_HEIGHT_SINGLE: f32 = 48.0;
/// Height for two-line snackbar
pub const SNACKBAR_HEIGHT_DOUBLE: f32 = 68.0;
/// Bottom margin from screen edge
pub const SNACKBAR_MARGIN_BOTTOM: f32 = 16.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating snackbar hosts
pub struct SnackbarHostBuilder;

impl SnackbarHostBuilder {
    /// Build the snackbar host
    pub fn build() -> impl Bundle {
        (
            SnackbarHost,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(SNACKBAR_MARGIN_BOTTOM),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )
    }
}

/// Builder for creating snackbars
pub struct SnackbarBuilder {
    snackbar: Snackbar,
}

impl SnackbarBuilder {
    /// Create a new snackbar builder
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            snackbar: Snackbar {
                message: message.into(),
                action: None,
                duration: Snackbar::DEFAULT_DURATION,
                dismissible: true,
                animation_state: SnackbarAnimationState::Entering,
                time_remaining: Snackbar::DEFAULT_DURATION,
                animation_progress: 0.0,
            },
        }
    }

    /// Add an action button
    pub fn action(mut self, text: impl Into<String>) -> Self {
        self.snackbar.action = Some(text.into());
        self
    }

    /// Set the duration
    pub fn duration(mut self, seconds: f32) -> Self {
        self.snackbar.duration = seconds;
        self.snackbar.time_remaining = seconds;
        self
    }

    /// Set short duration
    pub fn short(self) -> Self {
        self.duration(Snackbar::SHORT_DURATION)
    }

    /// Set long duration
    pub fn long(self) -> Self {
        self.duration(Snackbar::LONG_DURATION)
    }

    /// Set indefinite duration
    pub fn indefinite(self) -> Self {
        self.duration(Snackbar::INDEFINITE)
    }

    /// Build the snackbar bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = theme.inverse_surface;

        (
            self.snackbar,
            Node {
                min_width: Val::Px(SNACKBAR_MIN_WIDTH),
                max_width: Val::Px(SNACKBAR_MAX_WIDTH),
                min_height: Val::Px(SNACKBAR_HEIGHT_SINGLE),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
        )
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a snackbar entity
pub fn spawn_snackbar(
    commands: &mut Commands,
    theme: &MaterialTheme,
    event: &ShowSnackbar,
    _host: Entity,
) -> Entity {
    let snackbar = Snackbar::from_event(event);
    let message = snackbar.message.clone();
    let action = snackbar.action.clone();

    commands
        .spawn((
            snackbar,
            Node {
                min_width: Val::Px(SNACKBAR_MIN_WIDTH),
                max_width: Val::Px(SNACKBAR_MAX_WIDTH),
                min_height: Val::Px(SNACKBAR_HEIGHT_SINGLE),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                // Start off-screen for animation
                bottom: Val::Px(-SNACKBAR_HEIGHT_SINGLE - SNACKBAR_MARGIN_BOTTOM),
                ..default()
            },
            BackgroundColor(theme.inverse_surface),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
        ))
        .with_children(|parent| {
            // Message text
            parent.spawn((
                SnackbarMessage,
                Text::new(&message),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.inverse_on_surface),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Action button (if provided)
            if let Some(action_text) = &action {
                parent.spawn((
                    SnackbarAction,
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(Spacing::SMALL), Val::Px(Spacing::EXTRA_SMALL)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(action_text),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.inverse_primary),
                    ));
                });
            }
        })
        .id()
}

// ============================================================================
// Systems
// ============================================================================

/// System to process the snackbar queue
fn snackbar_queue_system(
    mut commands: Commands,
    mut events: MessageReader<ShowSnackbar>,
    theme: Option<Res<MaterialTheme>>,
    mut queue: ResMut<SnackbarQueue>,
    hosts: Query<Entity, With<SnackbarHost>>,
    snackbars: Query<&Snackbar>,
) {
    let Some(theme) = theme else { return };

    // Add new events to the queue
    for event in events.read() {
        queue.queue.push(event.clone());
    }

    // Check if we can show a snackbar
    let can_show = match queue.active {
        Some(entity) => {
            // Check if active snackbar is dismissed
            snackbars.get(entity).is_ok_and(|s| s.is_dismissed())
        }
        None => true,
    };

    // Show next snackbar if queue has items and we can show
    if can_show && !queue.queue.is_empty() {
        if let Some(event) = queue.queue.first() {
            if let Some(host) = hosts.iter().next() {
                let entity = spawn_snackbar(&mut commands, &theme, event, host);
                queue.active = Some(entity);
                queue.queue.remove(0);
            }
        }
    }
}

/// System to animate snackbars
fn snackbar_animation_system(
    time: Res<Time>,
    mut snackbars: Query<(&mut Snackbar, &mut Node)>,
) {
    for (mut snackbar, mut node) in snackbars.iter_mut() {
        let dt = time.delta_secs();

        match snackbar.animation_state {
            SnackbarAnimationState::Entering => {
                snackbar.animation_progress += dt / Duration::MEDIUM2;
                if snackbar.animation_progress >= 1.0 {
                    snackbar.animation_progress = 1.0;
                    snackbar.animation_state = SnackbarAnimationState::Visible;
                }

                // Slide up animation
                let progress = ease_standard_decelerate(snackbar.animation_progress);
                let offset = (1.0 - progress) * (SNACKBAR_HEIGHT_SINGLE + SNACKBAR_MARGIN_BOTTOM);
                node.bottom = Val::Px(-offset);
            }
            SnackbarAnimationState::Visible => {
                // No animation needed
            }
            SnackbarAnimationState::Exiting => {
                snackbar.animation_progress -= dt / Duration::MEDIUM2;
                if snackbar.animation_progress <= 0.0 {
                    snackbar.animation_progress = 0.0;
                    snackbar.animation_state = SnackbarAnimationState::Dismissed;
                }

                // Slide down animation
                let progress = ease_standard_accelerate(snackbar.animation_progress);
                let offset = (1.0 - progress) * (SNACKBAR_HEIGHT_SINGLE + SNACKBAR_MARGIN_BOTTOM);
                node.bottom = Val::Px(-offset);
            }
            SnackbarAnimationState::Dismissed => {
                // Will be cleaned up
            }
        }
    }
}

/// System to handle snackbar timeout
fn snackbar_timeout_system(
    time: Res<Time>,
    mut snackbars: Query<&mut Snackbar>,
    mut queue: ResMut<SnackbarQueue>,
) {
    for mut snackbar in snackbars.iter_mut() {
        if snackbar.animation_state == SnackbarAnimationState::Visible {
            snackbar.time_remaining -= time.delta_secs();

            if snackbar.time_remaining <= 0.0 {
                snackbar.dismiss();
            }
        }
    }

    // Clean up dismissed snackbars in queue
    if let Some(entity) = queue.active {
        if let Ok(snackbar) = snackbars.get(entity) {
            if snackbar.is_dismissed() {
                queue.active = None;
            }
        }
    }
}

/// System to handle snackbar action clicks
fn snackbar_action_system(
    interactions: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<SnackbarAction>)>,
    mut snackbars: Query<(Entity, &mut Snackbar)>,
    mut events: MessageWriter<SnackbarActionEvent>,
) {
    for (interaction, parent) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((entity, mut snackbar)) = snackbars.get_mut(parent.parent()) {
                if let Some(action) = &snackbar.action {
                    events.write(SnackbarActionEvent {
                        entity,
                        action: action.clone(),
                    });
                }
                snackbar.dismiss();
            }
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
    fn test_snackbar_creation() {
        let snackbar = Snackbar::from_event(&ShowSnackbar::message("Test"));
        assert_eq!(snackbar.message, "Test");
        assert!(snackbar.action.is_none());
        assert!((snackbar.duration - Snackbar::DEFAULT_DURATION).abs() < 0.001);
    }

    #[test]
    fn test_snackbar_with_action() {
        let snackbar = Snackbar::from_event(&ShowSnackbar::with_action("Error", "Retry"));
        assert_eq!(snackbar.message, "Error");
        assert_eq!(snackbar.action, Some("Retry".to_string()));
    }

    #[test]
    fn test_snackbar_dismiss() {
        let mut snackbar = Snackbar::from_event(&ShowSnackbar::message("Test"));
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Entering);

        snackbar.dismiss();
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Exiting);
    }

    #[test]
    fn test_show_snackbar_builder() {
        let event = ShowSnackbar::message("Hello")
            .duration(5.0)
            .dismissible(false);

        assert_eq!(event.message, "Hello");
        assert_eq!(event.duration, Some(5.0));
        assert!(!event.dismissible);
    }
}
