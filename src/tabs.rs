//! Material Design 3 Tabs component
//!
//! Tabs organize content across different screens, data sets, and other interactions.
//! Reference: <https://m3.material.io/components/tabs/overview>

use bevy::prelude::*;

use crate::{
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::Spacing,
};

/// Plugin for the tabs component
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TabChangeEvent>()
            .add_systems(Update, tab_interaction_system)
            .add_systems(Update, tab_style_system);
    }
}

/// Tab variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabVariant {
    /// Primary tabs - For primary destinations
    #[default]
    Primary,
    /// Secondary tabs - For secondary destinations or subpages
    Secondary,
}

/// Material tabs container
#[derive(Component)]
pub struct MaterialTabs {
    /// Tab variant
    pub variant: TabVariant,
    /// Currently selected tab index
    pub selected: usize,
}

impl MaterialTabs {
    /// Create a new tabs container
    pub fn new() -> Self {
        Self {
            variant: TabVariant::default(),
            selected: 0,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initially selected tab
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
}

impl Default for MaterialTabs {
    fn default() -> Self {
        Self::new()
    }
}

/// Material tab item
#[derive(Component)]
pub struct MaterialTab {
    /// Tab index in parent container
    pub index: usize,
    /// Tab label text
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether the tab is disabled
    pub disabled: bool,
    /// Whether this tab is currently selected
    pub selected: bool,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialTab {
    /// Create a new tab
    pub fn new(index: usize, label: impl Into<String>) -> Self {
        Self {
            index,
            label: label.into(),
            icon: None,
            disabled: false,
            selected: index == 0,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Get the content color
    pub fn content_color(&self, theme: &MaterialTheme, variant: TabVariant) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.selected {
            match variant {
                TabVariant::Primary => theme.primary,
                TabVariant::Secondary => theme.on_surface,
            }
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the indicator color
    pub fn indicator_color(&self, theme: &MaterialTheme, variant: TabVariant) -> Color {
        match variant {
            TabVariant::Primary => theme.primary,
            TabVariant::Secondary => theme.primary,
        }
    }
}

/// Event when tab selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct TabChangeEvent {
    /// The tabs container entity
    pub tabs_entity: Entity,
    /// The selected tab entity
    pub tab_entity: Entity,
    /// The selected tab index
    pub index: usize,
}

/// Tab dimensions
pub const TAB_HEIGHT_PRIMARY: f32 = 64.0;
pub const TAB_HEIGHT_PRIMARY_ICON_ONLY: f32 = 48.0;
pub const TAB_HEIGHT_SECONDARY: f32 = 48.0;
pub const TAB_INDICATOR_HEIGHT: f32 = 3.0;

/// System to handle tab interactions
fn tab_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialTab, &ChildOf),
        (Changed<Interaction>, With<MaterialTab>),
    >,
    mut tabs_query: Query<(Entity, &mut MaterialTabs)>,
    mut change_events: MessageWriter<TabChangeEvent>,
) {
    for (entity, interaction, mut tab, parent) in interaction_query.iter_mut() {
        if tab.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                tab.pressed = true;
                tab.hovered = false;
                
                if !tab.selected {
                    let tab_index = tab.index;
                    
                    // Update tabs container
                    if let Ok((tabs_entity, mut tabs)) = tabs_query.get_mut(parent.parent()) {
                        tabs.selected = tab_index;
                        
                        // Deselect all other tabs
                        // This would need more complex logic to get siblings
                        
                        change_events.write(TabChangeEvent {
                            tabs_entity,
                            tab_entity: entity,
                            index: tab_index,
                        });
                    }
                    
                    tab.selected = true;
                }
            }
            Interaction::Hovered => {
                tab.pressed = false;
                tab.hovered = true;
            }
            Interaction::None => {
                tab.pressed = false;
                tab.hovered = false;
            }
        }
    }
}

/// System to update tab styles
fn tab_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut tabs: Query<(&MaterialTab, &mut BackgroundColor), Changed<MaterialTab>>,
) {
    let Some(_theme) = theme else { return };

    for (_tab, mut bg_color) in tabs.iter_mut() {
        *bg_color = BackgroundColor(Color::NONE);
    }
}

/// Builder for tabs container
pub struct TabsBuilder {
    tabs: MaterialTabs,
}

impl TabsBuilder {
    /// Create a new tabs builder
    pub fn new() -> Self {
        Self {
            tabs: MaterialTabs::new(),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.tabs.variant = variant;
        self
    }

    /// Make primary tabs
    pub fn primary(self) -> Self {
        self.variant(TabVariant::Primary)
    }

    /// Make secondary tabs
    pub fn secondary(self) -> Self {
        self.variant(TabVariant::Secondary)
    }

    /// Set initially selected tab
    pub fn selected(mut self, index: usize) -> Self {
        self.tabs.selected = index;
        self
    }

    /// Build the tabs bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let height = match self.tabs.variant {
            TabVariant::Primary => TAB_HEIGHT_PRIMARY,
            TabVariant::Secondary => TAB_HEIGHT_SECONDARY,
        };

        (
            self.tabs,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(height),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor(theme.surface),
        )
    }
}

impl Default for TabsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for individual tabs
pub struct TabBuilder {
    tab: MaterialTab,
    variant: TabVariant,
}

impl TabBuilder {
    /// Create a new tab builder
    pub fn new(index: usize, label: impl Into<String>) -> Self {
        Self {
            tab: MaterialTab::new(index, label),
            variant: TabVariant::Primary,
        }
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.tab.icon = Some(icon.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.tab.disabled = disabled;
        self
    }

    /// Set selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.tab.selected = selected;
        self
    }

    /// Set the variant (inherited from parent tabs usually)
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Build the tab bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _content_color = self.tab.content_color(theme, self.variant);

        (
            self.tab,
            Button,
            RippleHost::new(),
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )
    }
}

/// Marker for tab indicator (the active line)
#[derive(Component)]
pub struct TabIndicator;

/// Create a tab indicator
pub fn create_tab_indicator(theme: &MaterialTheme, _variant: TabVariant) -> impl Bundle {
    (
        TabIndicator,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            height: Val::Px(TAB_INDICATOR_HEIGHT),
            ..default()
        },
        BackgroundColor(theme.primary),
        BorderRadius::top(Val::Px(TAB_INDICATOR_HEIGHT)),
    )
}
