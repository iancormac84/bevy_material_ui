//! Material Design 3 List component
//!
//! Lists are continuous, vertical indexes of text and images.
//! Reference: <https://m3.material.io/components/lists/overview>

use bevy::prelude::*;
use bevy::ui::ScrollPosition;

use crate::{
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::Spacing,
};

/// Plugin for the list component
pub struct ListPlugin;

impl Plugin for ListPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ListItemClickEvent>()
            .add_systems(Update, (list_item_interaction_system, list_item_style_system));
    }
}

/// List item variants based on content
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ListItemVariant {
    /// One line of text
    #[default]
    OneLine,
    /// Two lines of text
    TwoLine,
    /// Three lines of text
    ThreeLine,
}

impl ListItemVariant {
    /// Get the height for this variant
    pub fn height(&self) -> f32 {
        match self {
            ListItemVariant::OneLine => 56.0,
            ListItemVariant::TwoLine => 72.0,
            ListItemVariant::ThreeLine => 88.0,
        }
    }
}

/// Material list container
#[derive(Component, Default)]
pub struct MaterialList;

impl MaterialList {
    /// Create a new list
    pub fn new() -> Self {
        Self
    }
}

/// Material list item component
#[derive(Component)]
pub struct MaterialListItem {
    /// Item variant
    pub variant: ListItemVariant,
    /// Whether the item is disabled
    pub disabled: bool,
    /// Whether the item is selected
    pub selected: bool,
    /// Headline text (primary text)
    pub headline: String,
    /// Supporting text (secondary text)
    pub supporting_text: Option<String>,
    /// Trailing supporting text
    pub trailing_text: Option<String>,
    /// Leading icon
    pub leading_icon: Option<String>,
    /// Trailing icon
    pub trailing_icon: Option<String>,
    /// Leading avatar/image URL
    pub leading_avatar: Option<String>,
    /// Leading video thumbnail URL
    pub leading_video: Option<String>,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialListItem {
    /// Create a new list item
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            variant: ListItemVariant::default(),
            disabled: false,
            selected: false,
            headline: headline.into(),
            supporting_text: None,
            trailing_text: None,
            leading_icon: None,
            trailing_icon: None,
            leading_avatar: None,
            leading_video: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: ListItemVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set trailing text
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
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

    /// Set leading avatar
    pub fn leading_avatar(mut self, url: impl Into<String>) -> Self {
        self.leading_avatar = Some(url.into());
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

    /// Get the background color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.selected {
            theme.secondary_container
        } else {
            Color::NONE
        }
    }

    /// Get the headline color
    pub fn headline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the supporting text color
    pub fn supporting_text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
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

/// Event when list item is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct ListItemClickEvent {
    pub entity: Entity,
}

/// System to handle list item interactions
fn list_item_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialListItem),
        (Changed<Interaction>, With<MaterialListItem>),
    >,
    mut click_events: MessageWriter<ListItemClickEvent>,
) {
    for (entity, interaction, mut item) in interaction_query.iter_mut() {
        if item.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                item.pressed = true;
                item.hovered = false;
                click_events.write(ListItemClickEvent { entity });
            }
            Interaction::Hovered => {
                item.pressed = false;
                item.hovered = true;
            }
            Interaction::None => {
                item.pressed = false;
                item.hovered = false;
            }
        }
    }
}

/// System to update list item styles
fn list_item_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut items: Query<(&MaterialListItem, &mut BackgroundColor), Changed<MaterialListItem>>,
) {
    let Some(theme) = theme else { return };

    for (item, mut bg_color) in items.iter_mut() {
        *bg_color = BackgroundColor(item.background_color(&theme));
    }
}

/// Builder for lists
pub struct ListBuilder {
    /// Maximum height before scrolling (None = no limit)
    max_height: Option<f32>,
    /// Whether to show scrollbar
    show_scrollbar: bool,
}

impl ListBuilder {
    /// Create a new list builder
    pub fn new() -> Self {
        Self {
            max_height: None,
            show_scrollbar: true,
        }
    }

    /// Set maximum height (enables scrolling)
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }
    
    /// Set maximum visible items (enables scrolling based on item count)
    /// Uses one-line item height (56px) as reference
    pub fn max_visible_items(mut self, count: usize) -> Self {
        self.max_height = Some(count as f32 * 56.0);
        self
    }
    
    /// Set maximum visible items with specific variant height
    pub fn max_visible_items_variant(mut self, count: usize, variant: ListItemVariant) -> Self {
        self.max_height = Some(count as f32 * variant.height());
        self
    }
    
    /// Hide the scrollbar (content still scrollable)
    pub fn hide_scrollbar(mut self) -> Self {
        self.show_scrollbar = false;
        self
    }

    /// Build the list bundle (non-scrollable)
    pub fn build(self) -> impl Bundle {
        (
            MaterialList::new(),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                ..default()
            },
        )
    }
    
    /// Build a scrollable list bundle
    /// Uses scroll_y() for native scrolling - ensure scroll position is clamped externally
    pub fn build_scrollable(self) -> impl Bundle {
        let height = self.max_height.map(Val::Px).unwrap_or(Val::Auto);
        (
            MaterialList::new(),
            ScrollableList,
            ScrollPosition::default(),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height,
                max_height: height,
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
        )
    }
}

/// Marker for scrollable lists
#[derive(Component)]
pub struct ScrollableList;

impl Default for ListBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for list items
pub struct ListItemBuilder {
    item: MaterialListItem,
}

impl ListItemBuilder {
    /// Create a new list item builder
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            item: MaterialListItem::new(headline),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: ListItemVariant) -> Self {
        self.item.variant = variant;
        self
    }

    /// Make one-line item
    pub fn one_line(self) -> Self {
        self.variant(ListItemVariant::OneLine)
    }

    /// Make two-line item
    pub fn two_line(self) -> Self {
        self.variant(ListItemVariant::TwoLine)
    }

    /// Make three-line item
    pub fn three_line(self) -> Self {
        self.variant(ListItemVariant::ThreeLine)
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.item.supporting_text = Some(text.into());
        self
    }

    /// Set trailing text
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.item.trailing_text = Some(text.into());
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.item.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.item.trailing_icon = Some(icon.into());
        self
    }

    /// Set leading avatar
    pub fn leading_avatar(mut self, url: impl Into<String>) -> Self {
        self.item.leading_avatar = Some(url.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.item.disabled = disabled;
        self
    }

    /// Set selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.item.selected = selected;
        self
    }

    /// Build the list item bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.item.background_color(theme);
        let height = self.item.variant.height();

        (
            self.item,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(height),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::SMALL)),
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::LARGE),
                ..default()
            },
            BackgroundColor(bg_color),
        )
    }
}

/// Marker for leading content area
#[derive(Component)]
pub struct ListItemLeading;

/// Marker for content/body area
#[derive(Component)]
pub struct ListItemBody;

/// Marker for trailing content area
#[derive(Component)]
pub struct ListItemTrailing;

/// Marker for list divider
#[derive(Component)]
pub struct ListDivider;

/// Create a list divider
pub fn create_list_divider(theme: &MaterialTheme, inset: bool) -> impl Bundle {
    (
        ListDivider,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: if inset {
                UiRect::left(Val::Px(Spacing::LARGE + 56.0)) // Account for leading element
            } else {
                UiRect::ZERO
            },
            ..default()
        },
        BackgroundColor(theme.outline_variant),
    )
}
