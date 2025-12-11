//! Material Design 3 Text Field component
//!
//! Text fields let users enter and edit text.
//! Reference: <https://m3.material.io/components/text-fields/overview>

use bevy::prelude::*;

use crate::{
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the text field component
pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TextFieldChangeEvent>()
            .add_message::<TextFieldSubmitEvent>()
            .add_systems(Update, (text_field_focus_system, text_field_style_system));
    }
}

/// Text field variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextFieldVariant {
    /// Filled text field - Has background fill
    #[default]
    Filled,
    /// Outlined text field - Has border outline
    Outlined,
}

/// Material text field component
#[derive(Component)]
pub struct MaterialTextField {
    /// Text field variant
    pub variant: TextFieldVariant,
    /// Current text value
    pub value: String,
    /// Placeholder/hint text
    pub placeholder: String,
    /// Label text
    pub label: Option<String>,
    /// Supporting text below the field
    pub supporting_text: Option<String>,
    /// Leading icon
    pub leading_icon: Option<String>,
    /// Trailing icon
    pub trailing_icon: Option<String>,
    /// Whether the field is disabled
    pub disabled: bool,
    /// Whether the field has an error
    pub error: bool,
    /// Error message
    pub error_text: Option<String>,
    /// Maximum character count (None = unlimited)
    pub max_length: Option<usize>,
    /// Whether the field is focused
    pub focused: bool,
    /// Whether the field has content
    pub has_content: bool,
}

impl MaterialTextField {
    /// Create a new text field
    pub fn new() -> Self {
        Self {
            variant: TextFieldVariant::default(),
            value: String::new(),
            placeholder: String::new(),
            label: None,
            supporting_text: None,
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
            error: false,
            error_text: None,
            max_length: None,
            focused: false,
            has_content: false,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initial value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.has_content = !self.value.is_empty();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self.error = true;
        self
    }

    /// Set max length
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Get the container color (for filled variant)
    pub fn container_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.04);
        }

        match self.variant {
            TextFieldVariant::Filled => theme.surface_container_highest,
            TextFieldVariant::Outlined => Color::NONE,
        }
    }

    /// Get the active indicator / outline color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the label color
    pub fn label_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the input text color
    pub fn input_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the placeholder text color
    pub fn placeholder_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the supporting text color
    pub fn supporting_text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }

    /// Check if label should be floating (raised above input)
    pub fn is_label_floating(&self) -> bool {
        self.focused || self.has_content
    }
}

impl Default for MaterialTextField {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when text field value changes
#[derive(Event, bevy::prelude::Message)]
pub struct TextFieldChangeEvent {
    pub entity: Entity,
    pub value: String,
}

/// Event when text field is submitted (Enter pressed)
#[derive(Event, bevy::prelude::Message)]
pub struct TextFieldSubmitEvent {
    pub entity: Entity,
    pub value: String,
}

/// Text field dimensions
pub const TEXT_FIELD_HEIGHT: f32 = 56.0;
pub const TEXT_FIELD_MIN_WIDTH: f32 = 210.0;

/// System to handle text field focus
fn text_field_focus_system(
    mut interaction_query: Query<
        (&Interaction, &mut MaterialTextField),
        (Changed<Interaction>, With<MaterialTextField>),
    >,
) {
    for (interaction, mut text_field) in interaction_query.iter_mut() {
        if text_field.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                text_field.focused = true;
            }
            Interaction::Hovered | Interaction::None => {
                // Focus is only lost when clicking elsewhere
                // This would need a more complex focus management system
            }
        }
    }
}

/// System to update text field styles
fn text_field_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut text_fields: Query<
        (&MaterialTextField, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialTextField>,
    >,
) {
    let Some(theme) = theme else { return };

    for (text_field, mut bg_color, mut border_color) in text_fields.iter_mut() {
        *bg_color = BackgroundColor(text_field.container_color(&theme));
        *border_color = BorderColor::all(text_field.indicator_color(&theme));
    }
}

/// Builder for text fields
pub struct TextFieldBuilder {
    text_field: MaterialTextField,
    width: Val,
}

impl TextFieldBuilder {
    /// Create a new text field builder
    pub fn new() -> Self {
        Self {
            text_field: MaterialTextField::new(),
            width: Val::Px(TEXT_FIELD_MIN_WIDTH),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.text_field.variant = variant;
        self
    }

    /// Make filled variant
    pub fn filled(self) -> Self {
        self.variant(TextFieldVariant::Filled)
    }

    /// Make outlined variant
    pub fn outlined(self) -> Self {
        self.variant(TextFieldVariant::Outlined)
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.text_field.value = value.into();
        self.text_field.has_content = !self.text_field.value.is_empty();
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.text_field.placeholder = placeholder.into();
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.text_field.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.text_field.supporting_text = Some(text.into());
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.text_field.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.text_field.trailing_icon = Some(icon.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.text_field.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.text_field.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.text_field.error_text = Some(text.into());
        self.text_field.error = true;
        self
    }

    /// Set max length
    pub fn max_length(mut self, max: usize) -> Self {
        self.text_field.max_length = Some(max);
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the text field bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.text_field.container_color(theme);
        let border_color = self.text_field.indicator_color(theme);
        let is_outlined = self.text_field.variant == TextFieldVariant::Outlined;

        (
            self.text_field,
            Button,
            Node {
                width: self.width,
                height: Val::Px(TEXT_FIELD_HEIGHT),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                border: if is_outlined {
                    UiRect::all(Val::Px(1.0))
                } else {
                    UiRect::bottom(Val::Px(1.0))
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::top(Val::Px(if is_outlined {
                CornerRadius::EXTRA_SMALL
            } else {
                CornerRadius::EXTRA_SMALL
            })),
        )
    }
}

impl Default for TextFieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for the label element
#[derive(Component)]
pub struct TextFieldLabel;

/// Marker for the input element
#[derive(Component)]
pub struct TextFieldInput;

/// Marker for the supporting text element
#[derive(Component)]
pub struct TextFieldSupportingText;
