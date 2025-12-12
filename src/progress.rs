//! Material Design 3 Progress Indicators
//!
//! Progress indicators inform users about the status of ongoing processes.
//! Reference: <https://m3.material.io/components/progress-indicators/overview>

use bevy::prelude::*;

use crate::{
    theme::MaterialTheme,
    tokens::{CornerRadius, Duration},
};

/// Plugin for progress indicator components
pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            linear_progress_animation_system,
            circular_progress_animation_system,
            progress_style_system,
        ));
    }
}

/// Progress indicator variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ProgressVariant {
    /// Linear progress bar
    #[default]
    Linear,
    /// Circular progress indicator
    Circular,
}

/// Progress mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ProgressMode {
    /// Determinate - Shows specific progress
    #[default]
    Determinate,
    /// Indeterminate - Shows activity without progress
    Indeterminate,
}

/// Material linear progress indicator
#[derive(Component)]
pub struct MaterialLinearProgress {
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Progress mode
    pub mode: ProgressMode,
    /// Whether the indicator uses a 4-color approach
    pub four_color: bool,
    /// Animation state for indeterminate mode
    pub animation_progress: f32,
}

impl MaterialLinearProgress {
    /// Create a new linear progress indicator
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            mode: ProgressMode::default(),
            four_color: false,
            animation_progress: 0.0,
        }
    }

    /// Set progress value
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set to indeterminate mode
    pub fn indeterminate(mut self) -> Self {
        self.mode = ProgressMode::Indeterminate;
        self
    }

    /// Enable four-color mode
    pub fn four_color(mut self) -> Self {
        self.four_color = true;
        self
    }

    /// Get the track color
    pub fn track_color(&self, theme: &MaterialTheme) -> Color {
        theme.surface_container_highest
    }

    /// Get the indicator color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        theme.primary
    }
}

impl Default for MaterialLinearProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Material circular progress indicator
#[derive(Component)]
pub struct MaterialCircularProgress {
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Progress mode
    pub mode: ProgressMode,
    /// Whether the indicator uses a 4-color approach
    pub four_color: bool,
    /// Size of the indicator
    pub size: f32,
    /// Animation state for indeterminate mode
    pub animation_progress: f32,
    /// Rotation angle for animation
    pub rotation: f32,
}

impl MaterialCircularProgress {
    /// Create a new circular progress indicator
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            mode: ProgressMode::default(),
            four_color: false,
            size: CIRCULAR_PROGRESS_SIZE,
            animation_progress: 0.0,
            rotation: 0.0,
        }
    }

    /// Set progress value
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set to indeterminate mode
    pub fn indeterminate(mut self) -> Self {
        self.mode = ProgressMode::Indeterminate;
        self
    }

    /// Enable four-color mode
    pub fn four_color(mut self) -> Self {
        self.four_color = true;
        self
    }

    /// Set custom size
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Get the track color
    pub fn track_color(&self, _theme: &MaterialTheme) -> Color {
        Color::NONE // Circular doesn't have visible track by default
    }

    /// Get the indicator color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        theme.primary
    }
}

impl Default for MaterialCircularProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress indicator dimensions
pub const LINEAR_PROGRESS_HEIGHT: f32 = 4.0;
pub const CIRCULAR_PROGRESS_SIZE: f32 = 48.0;
pub const CIRCULAR_PROGRESS_TRACK_WIDTH: f32 = 4.0;

/// System to animate linear progress indicators
fn linear_progress_animation_system(
    time: Res<Time>,
    mut progress_bars: Query<&mut MaterialLinearProgress>,
) {
    for mut progress in progress_bars.iter_mut() {
        if progress.mode == ProgressMode::Indeterminate {
            progress.animation_progress += time.delta_secs() / Duration::LONG4;
            if progress.animation_progress > 1.0 {
                progress.animation_progress -= 1.0;
            }
        }
    }
}

/// System to animate circular progress indicators
fn circular_progress_animation_system(
    time: Res<Time>,
    mut progress_indicators: Query<&mut MaterialCircularProgress>,
) {
    for mut progress in progress_indicators.iter_mut() {
        if progress.mode == ProgressMode::Indeterminate {
            progress.animation_progress += time.delta_secs() / Duration::LONG4;
            if progress.animation_progress > 1.0 {
                progress.animation_progress -= 1.0;
            }
            
            // Rotate the indicator
            progress.rotation += time.delta_secs() * std::f32::consts::TAU / Duration::EXTRA_LONG4;
            if progress.rotation > std::f32::consts::TAU {
                progress.rotation -= std::f32::consts::TAU;
            }
        }
    }
}

/// System to update progress styles
fn progress_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut linear_progress: Query<
        (&MaterialLinearProgress, &mut BackgroundColor),
        Changed<MaterialLinearProgress>,
    >,
) {
    let Some(theme) = theme else { return };

    for (progress, mut bg_color) in linear_progress.iter_mut() {
        *bg_color = BackgroundColor(progress.track_color(&theme));
    }
}

/// Builder for linear progress
pub struct LinearProgressBuilder {
    progress: MaterialLinearProgress,
    width: Val,
}

impl LinearProgressBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            progress: MaterialLinearProgress::new(),
            width: Val::Percent(100.0),
        }
    }

    /// Set progress
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set indeterminate mode
    pub fn indeterminate(mut self) -> Self {
        self.progress.mode = ProgressMode::Indeterminate;
        self
    }

    /// Enable four-color
    pub fn four_color(mut self) -> Self {
        self.progress.four_color = true;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.progress.track_color(theme);

        (
            self.progress,
            Node {
                width: self.width,
                height: Val::Px(LINEAR_PROGRESS_HEIGHT),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
        )
    }
}

impl Default for LinearProgressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for circular progress
pub struct CircularProgressBuilder {
    progress: MaterialCircularProgress,
}

impl CircularProgressBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            progress: MaterialCircularProgress::new(),
        }
    }

    /// Set progress
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set indeterminate mode
    pub fn indeterminate(mut self) -> Self {
        self.progress.mode = ProgressMode::Indeterminate;
        self
    }

    /// Enable four-color
    pub fn four_color(mut self) -> Self {
        self.progress.four_color = true;
        self
    }

    /// Set size
    pub fn size(mut self, size: f32) -> Self {
        self.progress.size = size;
        self
    }

    /// Build the bundle
    pub fn build(self, _theme: &MaterialTheme) -> impl Bundle {
        let size = self.progress.size;

        (
            self.progress,
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                ..default()
            },
        )
    }
}

impl Default for CircularProgressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for progress indicator track
#[derive(Component)]
pub struct ProgressTrack;

/// Marker for progress indicator bar
#[derive(Component)]
pub struct ProgressIndicator;

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn progress indicators as children
/// 
/// This trait provides a clean API for spawning progress indicators within UI hierarchies.
/// 
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_linear_progress(&theme, 0.5);
///     children.spawn_indeterminate_progress(&theme);
///     children.spawn_circular_progress(&theme, 0.75);
/// });
/// ```
pub trait SpawnProgressChild {
    /// Spawn a linear progress indicator with determinate progress
    fn spawn_linear_progress(&mut self, theme: &MaterialTheme, progress: f32);
    
    /// Spawn an indeterminate linear progress indicator
    fn spawn_indeterminate_progress(&mut self, theme: &MaterialTheme);
    
    /// Spawn a circular progress indicator with determinate progress
    fn spawn_circular_progress(&mut self, theme: &MaterialTheme, progress: f32);
    
    /// Spawn an indeterminate circular progress indicator
    fn spawn_indeterminate_circular_progress(&mut self, theme: &MaterialTheme);
    
    /// Spawn a linear progress indicator with full builder control
    fn spawn_linear_progress_with(
        &mut self,
        theme: &MaterialTheme,
        builder: LinearProgressBuilder,
    );
    
    /// Spawn a circular progress indicator with full builder control
    fn spawn_circular_progress_with(
        &mut self,
        theme: &MaterialTheme,
        builder: CircularProgressBuilder,
    );
}

impl SpawnProgressChild for ChildSpawnerCommands<'_> {
    fn spawn_linear_progress(&mut self, theme: &MaterialTheme, progress: f32) {
        self.spawn_linear_progress_with(
            theme,
            LinearProgressBuilder::new().progress(progress),
        );
    }
    
    fn spawn_indeterminate_progress(&mut self, theme: &MaterialTheme) {
        self.spawn_linear_progress_with(
            theme,
            LinearProgressBuilder::new().indeterminate(),
        );
    }
    
    fn spawn_circular_progress(&mut self, theme: &MaterialTheme, progress: f32) {
        self.spawn_circular_progress_with(
            theme,
            CircularProgressBuilder::new().progress(progress),
        );
    }
    
    fn spawn_indeterminate_circular_progress(&mut self, theme: &MaterialTheme) {
        self.spawn_circular_progress_with(
            theme,
            CircularProgressBuilder::new().indeterminate(),
        );
    }
    
    fn spawn_linear_progress_with(
        &mut self,
        theme: &MaterialTheme,
        builder: LinearProgressBuilder,
    ) {
        let progress_value = builder.progress.progress;
        let indicator_color = builder.progress.indicator_color(theme);
        
        self.spawn(builder.build(theme))
            .with_children(|container| {
                // Progress indicator fill
                container.spawn((
                    ProgressIndicator,
                    Node {
                        width: Val::Percent(progress_value * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(indicator_color),
                    BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
                ));
            });
    }
    
    fn spawn_circular_progress_with(
        &mut self,
        theme: &MaterialTheme,
        builder: CircularProgressBuilder,
    ) {
        // Circular progress is typically rendered with custom drawing
        // For now, just spawn the container component
        self.spawn(builder.build(theme));
    }
}
