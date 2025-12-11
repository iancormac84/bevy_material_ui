//! Design tokens for spacing and corner radius
//!
//! Reference: <https://m3.material.io/foundations/layout/applying-layout>

/// Spacing tokens for consistent layout
pub struct Spacing;

impl Spacing {
    /// No spacing: 0dp
    pub const NONE: f32 = 0.0;
    /// Extra small spacing: 4dp
    pub const EXTRA_SMALL: f32 = 4.0;
    /// Small spacing: 8dp
    pub const SMALL: f32 = 8.0;
    /// Medium spacing: 12dp
    pub const MEDIUM: f32 = 12.0;
    /// Large spacing: 16dp
    pub const LARGE: f32 = 16.0;
    /// Extra large spacing: 24dp
    pub const EXTRA_LARGE: f32 = 24.0;
    /// XXL spacing: 32dp
    pub const XXL: f32 = 32.0;
    /// XXXL spacing: 48dp
    pub const XXXL: f32 = 48.0;
}

/// Corner radius tokens for consistent shapes
///
/// Reference: <https://m3.material.io/styles/shape/overview>
pub struct CornerRadius;

impl CornerRadius {
    /// No corner radius (sharp corners)
    pub const NONE: f32 = 0.0;
    /// Extra small radius: 4dp
    pub const EXTRA_SMALL: f32 = 4.0;
    /// Small radius: 8dp
    pub const SMALL: f32 = 8.0;
    /// Medium radius: 12dp
    pub const MEDIUM: f32 = 12.0;
    /// Large radius: 16dp
    pub const LARGE: f32 = 16.0;
    /// Extra large radius: 28dp
    pub const EXTRA_LARGE: f32 = 28.0;
    /// Full radius for pill shapes
    pub const FULL: f32 = 1000.0;
}

/// Duration tokens for animations
pub struct Duration;

impl Duration {
    /// Short1: 50ms - Subtle transitions
    pub const SHORT1: f32 = 0.05;
    /// Short2: 100ms - Quick transitions
    pub const SHORT2: f32 = 0.1;
    /// Short3: 150ms - Standard quick
    pub const SHORT3: f32 = 0.15;
    /// Short4: 200ms - Standard
    pub const SHORT4: f32 = 0.2;
    /// Medium1: 250ms - Standard medium
    pub const MEDIUM1: f32 = 0.25;
    /// Medium2: 300ms - Medium transitions
    pub const MEDIUM2: f32 = 0.3;
    /// Medium3: 350ms - Longer medium
    pub const MEDIUM3: f32 = 0.35;
    /// Medium4: 400ms - Emphasis medium
    pub const MEDIUM4: f32 = 0.4;
    /// Long1: 450ms - Emphasis transitions
    pub const LONG1: f32 = 0.45;
    /// Long2: 500ms - Complex transitions
    pub const LONG2: f32 = 0.5;
    /// Long3: 550ms - Detailed animations
    pub const LONG3: f32 = 0.55;
    /// Long4: 600ms - Extended animations
    pub const LONG4: f32 = 0.6;
    /// ExtraLong1: 700ms - Very long
    pub const EXTRA_LONG1: f32 = 0.7;
    /// ExtraLong2: 800ms - Extended
    pub const EXTRA_LONG2: f32 = 0.8;
    /// ExtraLong3: 900ms - Very extended
    pub const EXTRA_LONG3: f32 = 0.9;
    /// ExtraLong4: 1000ms - Maximum
    pub const EXTRA_LONG4: f32 = 1.0;
}

/// Easing curves for animations
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    /// Standard: For most transitions
    Standard,
    /// StandardAccelerate: Exiting elements
    StandardAccelerate,
    /// StandardDecelerate: Entering elements
    StandardDecelerate,
    /// Emphasized: High-emphasis transitions
    Emphasized,
    /// EmphasizedAccelerate: Exiting with emphasis
    EmphasizedAccelerate,
    /// EmphasizedDecelerate: Entering with emphasis
    EmphasizedDecelerate,
    /// Linear: Constant speed
    Linear,
}

impl Easing {
    /// Get the cubic bezier control points for this easing curve
    pub fn control_points(&self) -> (f32, f32, f32, f32) {
        match self {
            Easing::Standard => (0.2, 0.0, 0.0, 1.0),
            Easing::StandardAccelerate => (0.3, 0.0, 1.0, 1.0),
            Easing::StandardDecelerate => (0.0, 0.0, 0.0, 1.0),
            Easing::Emphasized => (0.2, 0.0, 0.0, 1.0),
            Easing::EmphasizedAccelerate => (0.3, 0.0, 0.8, 0.15),
            Easing::EmphasizedDecelerate => (0.05, 0.7, 0.1, 1.0),
            Easing::Linear => (0.0, 0.0, 1.0, 1.0),
        }
    }
}
