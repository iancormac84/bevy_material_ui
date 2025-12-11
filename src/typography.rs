//! Typography scale based on Material Design 3
//!
//! Reference: <https://m3.material.io/styles/typography/overview>

use bevy::prelude::*;

/// Typography scale resource containing font sizes for MD3 type scale
#[derive(Resource, Debug, Clone)]
pub struct Typography {
    // Display styles - largest type, reserved for short, important text
    /// Display large: 57sp
    pub display_large: f32,
    /// Display medium: 45sp
    pub display_medium: f32,
    /// Display small: 36sp
    pub display_small: f32,

    // Headline styles - best for short, high-emphasis text
    /// Headline large: 32sp
    pub headline_large: f32,
    /// Headline medium: 28sp
    pub headline_medium: f32,
    /// Headline small: 24sp
    pub headline_small: f32,

    // Title styles - smaller than headlines, for medium-emphasis text
    /// Title large: 22sp
    pub title_large: f32,
    /// Title medium: 16sp (medium weight)
    pub title_medium: f32,
    /// Title small: 14sp (medium weight)
    pub title_small: f32,

    // Label styles - used for buttons, tabs, and other UI components
    /// Label large: 14sp (medium weight)
    pub label_large: f32,
    /// Label medium: 12sp (medium weight)
    pub label_medium: f32,
    /// Label small: 11sp (medium weight)
    pub label_small: f32,

    // Body styles - for longer passages of text
    /// Body large: 16sp
    pub body_large: f32,
    /// Body medium: 14sp
    pub body_medium: f32,
    /// Body small: 12sp
    pub body_small: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            // Display
            display_large: 57.0,
            display_medium: 45.0,
            display_small: 36.0,

            // Headline
            headline_large: 32.0,
            headline_medium: 28.0,
            headline_small: 24.0,

            // Title
            title_large: 22.0,
            title_medium: 16.0,
            title_small: 14.0,

            // Label
            label_large: 14.0,
            label_medium: 12.0,
            label_small: 11.0,

            // Body
            body_large: 16.0,
            body_medium: 14.0,
            body_small: 12.0,
        }
    }
}

impl Typography {
    /// Create a scaled typography set (useful for different screen densities)
    pub fn scaled(scale: f32) -> Self {
        let default = Self::default();
        Self {
            display_large: default.display_large * scale,
            display_medium: default.display_medium * scale,
            display_small: default.display_small * scale,
            headline_large: default.headline_large * scale,
            headline_medium: default.headline_medium * scale,
            headline_small: default.headline_small * scale,
            title_large: default.title_large * scale,
            title_medium: default.title_medium * scale,
            title_small: default.title_small * scale,
            label_large: default.label_large * scale,
            label_medium: default.label_medium * scale,
            label_small: default.label_small * scale,
            body_large: default.body_large * scale,
            body_medium: default.body_medium * scale,
            body_small: default.body_small * scale,
        }
    }
}
