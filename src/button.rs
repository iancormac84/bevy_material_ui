//! Material Design 3 Button component
//!
//! Buttons communicate actions that users can take.
//! Reference: <https://m3.material.io/components/buttons/overview>

use bevy::prelude::*;

use crate::{
    elevation::Elevation,
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the button component
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ButtonClickEvent>()
            .add_systems(Update, (button_interaction_system, button_style_system));
    }
}

/// Button variants following Material Design 3
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    /// Elevated button - Use for emphasis on surfaces
    Elevated,
    /// Filled button - High emphasis actions
    #[default]
    Filled,
    /// Filled tonal button - Medium emphasis
    FilledTonal,
    /// Outlined button - Medium emphasis, secondary actions
    Outlined,
    /// Text button - Low emphasis actions
    Text,
}

/// Material button component
#[derive(Component, Clone)]
pub struct MaterialButton {
    /// Button variant style
    pub variant: ButtonVariant,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Button label text
    pub label: String,
    /// Optional leading icon
    pub icon: Option<String>,
    /// Whether this button is in a pressed state
    pub pressed: bool,
    /// Whether this button is hovered
    pub hovered: bool,
}

impl MaterialButton {
    /// Create a new material button
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            variant: ButtonVariant::default(),
            disabled: false,
            label: label.into(),
            icon: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the button variant
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set whether the button is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the button icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Get the background color based on state and theme
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        match self.variant {
            ButtonVariant::Elevated => theme.surface_container_low,
            ButtonVariant::Filled => {
                if self.pressed {
                    theme.primary
                } else if self.hovered {
                    theme.primary
                } else {
                    theme.primary
                }
            }
            ButtonVariant::FilledTonal => theme.secondary_container,
            ButtonVariant::Outlined => Color::NONE,
            ButtonVariant::Text => Color::NONE,
        }
    }

    /// Get the text color based on state and theme
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        match self.variant {
            ButtonVariant::Elevated => theme.primary,
            ButtonVariant::Filled => theme.on_primary,
            ButtonVariant::FilledTonal => theme.on_secondary_container,
            ButtonVariant::Outlined => theme.primary,
            ButtonVariant::Text => theme.primary,
        }
    }

    /// Get the border color based on state and theme
    pub fn border_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        match self.variant {
            ButtonVariant::Outlined => theme.outline,
            _ => Color::NONE,
        }
    }

    /// Get the elevation for this button variant
    pub fn elevation(&self) -> Elevation {
        if self.disabled {
            return Elevation::Level0;
        }

        match self.variant {
            ButtonVariant::Elevated => {
                if self.pressed {
                    Elevation::Level1
                } else if self.hovered {
                    Elevation::Level2
                } else {
                    Elevation::Level1
                }
            }
            ButtonVariant::Filled | ButtonVariant::FilledTonal => {
                if self.pressed || self.hovered {
                    Elevation::Level1
                } else {
                    Elevation::Level0
                }
            }
            _ => Elevation::Level0,
        }
    }

    /// Get the state layer opacity
    pub fn state_layer_opacity(&self) -> f32 {
        if self.disabled {
            0.0
        } else if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }
}

/// Event fired when a button is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct ButtonClickEvent {
    /// The button entity that was clicked
    pub entity: Entity,
}

/// System to handle button interactions
fn button_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialButton),
        (Changed<Interaction>, With<MaterialButton>),
    >,
    mut click_events: MessageWriter<ButtonClickEvent>,
) {
    for (entity, interaction, mut button) in interaction_query.iter_mut() {
        if button.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                button.pressed = true;
                button.hovered = false;
                click_events.write(ButtonClickEvent { entity });
            }
            Interaction::Hovered => {
                button.pressed = false;
                button.hovered = true;
            }
            Interaction::None => {
                button.pressed = false;
                button.hovered = false;
            }
        }
    }
}

/// System to update button visual styles based on state
fn button_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<
        (&MaterialButton, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialButton>,
    >,
) {
    let Some(theme) = theme else { return };

    for (button, mut bg_color, mut border_color) in buttons.iter_mut() {
        *bg_color = BackgroundColor(button.background_color(&theme));
        *border_color = BorderColor::all(button.border_color(&theme));
    }
}

/// Builder for creating Material buttons with proper styling
pub struct MaterialButtonBuilder {
    button: MaterialButton,
}

impl MaterialButtonBuilder {
    /// Create a new button builder
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            button: MaterialButton::new(label),
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    /// Make this an elevated button
    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    /// Make this a filled button
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    /// Make this a filled tonal button
    pub fn filled_tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    /// Make this an outlined button
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    /// Make this a text button
    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    /// Set the button as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button.disabled = disabled;
        self
    }

    /// Add an icon to the button
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.button.icon = Some(icon.into());
        self
    }

    /// Build the button bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.button.background_color(theme);
        let border_color = self.button.border_color(theme);
        let border_width = if self.button.variant == ButtonVariant::Outlined {
            1.0
        } else {
            0.0
        };

        (
            self.button,
            Button,
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(Spacing::EXTRA_LARGE), Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(border_width)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

/// Helper function to spawn a material button with a text child
pub fn spawn_material_button(
    commands: &mut Commands,
    theme: &MaterialTheme,
    label: impl Into<String>,
    variant: ButtonVariant,
) -> Entity {
    let label_text = label.into();
    let builder = MaterialButtonBuilder::new(label_text.clone()).variant(variant);
    let button = builder.button.clone();
    let text_color = button.text_color(theme);

    commands
        .spawn(MaterialButtonBuilder::new(label_text.clone()).variant(variant).build(theme))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label_text),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        })
        .id()
}
