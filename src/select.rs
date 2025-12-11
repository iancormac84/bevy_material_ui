//! Material Design 3 Select (Dropdown) component
//!
//! Select menus display a list of choices on a temporary surface and allow users to select one.
//! Reference: <https://m3.material.io/components/menus/overview>

use bevy::prelude::*;

use crate::{
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the select component
pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SelectChangeEvent>()
            .add_systems(Update, (select_interaction_system, select_style_system));
    }
}

/// Select variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectVariant {
    /// Filled select field
    #[default]
    Filled,
    /// Outlined select field
    Outlined,
}

/// Material select component
#[derive(Component)]
pub struct MaterialSelect {
    /// Select variant
    pub variant: SelectVariant,
    /// Currently selected option index
    pub selected_index: Option<usize>,
    /// Options list
    pub options: Vec<SelectOption>,
    /// Label text
    pub label: Option<String>,
    /// Supporting text
    pub supporting_text: Option<String>,
    /// Whether the select is disabled
    pub disabled: bool,
    /// Whether there's an error
    pub error: bool,
    /// Error message
    pub error_text: Option<String>,
    /// Whether the dropdown is open
    pub open: bool,
    /// Interaction states
    pub focused: bool,
    pub hovered: bool,
}

impl MaterialSelect {
    /// Create a new select
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            variant: SelectVariant::default(),
            selected_index: None,
            options,
            label: None,
            supporting_text: None,
            disabled: false,
            error: false,
            error_text: None,
            open: false,
            focused: false,
            hovered: false,
        }
    }

    /// Set variant
    pub fn with_variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initially selected option
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected_index = Some(index);
        }
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
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

    /// Get the selected option
    pub fn selected_option(&self) -> Option<&SelectOption> {
        self.selected_index.and_then(|i| self.options.get(i))
    }

    /// Get the display text for the current selection
    pub fn display_text(&self) -> String {
        self.selected_option()
            .map(|o| o.label.clone())
            .unwrap_or_default()
    }

    /// Get the container color
    pub fn container_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.04);
        }

        match self.variant {
            SelectVariant::Filled => theme.surface_container_highest,
            SelectVariant::Outlined => Color::NONE,
        }
    }

    /// Get the indicator/outline color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused || self.open {
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

        if self.focused || self.open {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the text color
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the trailing icon color
    pub fn trailing_icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }
}

/// A select option
#[derive(Debug, Clone)]
pub struct SelectOption {
    /// Display label
    pub label: String,
    /// Optional value (can be used for form submission)
    pub value: Option<String>,
    /// Optional leading icon
    pub icon: Option<String>,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new option
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            icon: None,
            disabled: false,
        }
    }

    /// Set the value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Event when selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct SelectChangeEvent {
    pub entity: Entity,
    pub index: usize,
    pub option: SelectOption,
}

/// Select dimensions
pub const SELECT_HEIGHT: f32 = 56.0;
pub const SELECT_OPTION_HEIGHT: f32 = 48.0;

/// System to handle select interactions
fn select_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut MaterialSelect),
        (Changed<Interaction>, With<MaterialSelect>),
    >,
) {
    for (interaction, mut select) in interaction_query.iter_mut() {
        if select.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                select.open = !select.open;
                select.focused = true;
            }
            Interaction::Hovered => {
                select.hovered = true;
            }
            Interaction::None => {
                select.hovered = false;
            }
        }
    }
}

/// System to update select styles
fn select_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut selects: Query<
        (&MaterialSelect, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialSelect>,
    >,
) {
    let Some(theme) = theme else { return };

    for (select, mut bg_color, mut border_color) in selects.iter_mut() {
        *bg_color = BackgroundColor(select.container_color(&theme));
        *border_color = BorderColor::all(select.indicator_color(&theme));
    }
}

/// Builder for select components
pub struct SelectBuilder {
    select: MaterialSelect,
    width: Val,
}

impl SelectBuilder {
    /// Create a new select builder
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            select: MaterialSelect::new(options),
            width: Val::Px(210.0),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.select.variant = variant;
        self
    }

    /// Make filled
    pub fn filled(self) -> Self {
        self.variant(SelectVariant::Filled)
    }

    /// Make outlined
    pub fn outlined(self) -> Self {
        self.variant(SelectVariant::Outlined)
    }

    /// Set initially selected option
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.select.options.len() {
            self.select.selected_index = Some(index);
        }
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.select.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.select.supporting_text = Some(text.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.select.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.select.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.select.error_text = Some(text.into());
        self.select.error = true;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the select bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.select.container_color(theme);
        let border_color = self.select.indicator_color(theme);
        let is_outlined = self.select.variant == SelectVariant::Outlined;

        (
            self.select,
            Button,
            Node {
                width: self.width,
                height: Val::Px(SELECT_HEIGHT),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                border: if is_outlined {
                    UiRect::all(Val::Px(1.0))
                } else {
                    UiRect::bottom(Val::Px(1.0))
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
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

/// Marker for select dropdown
#[derive(Component)]
pub struct SelectDropdown;

/// Marker for select option
#[derive(Component)]
pub struct SelectOptionItem {
    pub index: usize,
}
