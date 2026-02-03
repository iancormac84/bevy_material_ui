//! Material Design 3 List component
//!
//! Lists are continuous, vertical indexes of text and images.
//! Reference: <https://m3.material.io/components/lists/overview>

use bevy::prelude::*;
use bevy::ui::{ComputedNode, ScrollPosition};

use bevy::ecs::system::Command;

use crate::{
    icons::{icon_by_name, IconStyle, MaterialIcon},
    ripple::RippleHost,
    scroll::ScrollContainerBuilder,
    theme::{blend_state_layer, MaterialTheme},
    tokens::Spacing,
};

/// Maximum depth to traverse when searching for ancestor entities.
/// This prevents infinite loops in case of circular references or pathological entity hierarchies.
const MAX_ANCESTOR_DEPTH: usize = 32;

#[derive(Debug)]
struct DespawnDescendantsIfExists {
    entity: Entity,
}

impl Command for DespawnDescendantsIfExists {
    fn apply(self, world: &mut World) {
        if world.get_entity(self.entity).is_err() {
            return;
        }

        // Collect and despawn all descendants while leaving the root entity intact.
        let mut stack: Vec<Entity> = Vec::new();
        if let Some(children) = world.get::<Children>(self.entity) {
            stack.extend(children.iter());
        }

        while let Some(entity) = stack.pop() {
            // Another queued command in the same flush may have already despawned this entity.
            // Avoid calling `World::despawn` on missing entities to prevent warning spam.
            if world.get_entity(entity).is_err() {
                continue;
            }

            if let Some(children) = world.get::<Children>(entity) {
                stack.extend(children.iter());
            }

            let _ = world.despawn(entity);
        }
    }
}

fn resolve_icon_id(icon: &str) -> Option<crate::icons::material_icons::IconId> {
    let icon = icon.trim();
    if icon.is_empty() {
        return None;
    }

    icon_by_name(icon)
}

/// Plugin for the list component
pub struct ListPlugin;

impl Plugin for ListPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ListItemClickEvent>().add_systems(
            Update,
            (
                list_item_interaction_system,
                list_selection_system,
                list_virtual_cache_system,
                list_virtualization_system,
                list_item_style_system,
                list_item_text_style_system,
            ),
        );
    }
}

/// Data-backed list item model used for virtualized lists.
///
/// This is intentionally separate from the `MaterialListItem` component because virtualized
/// lists reuse a small pool of UI row entities.
#[derive(Debug, Clone)]
pub struct ListItemModel {
    pub variant: ListItemVariant,
    pub disabled: bool,
    pub selected: bool,
    pub headline: String,
    pub supporting_text: Option<String>,
    pub trailing_text: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
}

impl From<MaterialListItem> for ListItemModel {
    fn from(item: MaterialListItem) -> Self {
        Self {
            variant: item.variant,
            disabled: item.disabled,
            selected: item.selected,
            headline: item.headline,
            supporting_text: item.supporting_text,
            trailing_text: item.trailing_text,
            leading_icon: item.leading_icon,
            trailing_icon: item.trailing_icon,
        }
    }
}

/// Optional data/config component for lists.
///
/// - If `virtualize` is enabled and `items` is non-empty, the list will render using a fixed
///   pool of row entities and spacers instead of spawning all items.
#[derive(Component, Debug, Default)]
pub struct MaterialListData {
    pub items: Vec<ListItemModel>,
    pub virtualize: bool,
    pub overscan_rows: usize,
}

#[derive(Component, Debug, Default)]
struct ListVirtualCache {
    // prefix_heights[i] = sum of heights for items [0..i)
    prefix_heights: Vec<f32>,
    total_height: f32,
    min_item_height: f32,
}

#[derive(Component, Debug, Clone, Copy)]
struct ListVirtualOwner(Entity);

#[derive(Component, Debug)]
struct ListVirtualTopSpacer;

#[derive(Component, Debug)]
struct ListVirtualBottomSpacer;

#[derive(Component, Debug)]
struct ListVirtualRow {
    pool_slot: usize,
    item_index: Option<usize>,
}

/// Selection behavior for a list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListSelectionMode {
    /// Clicking items does not change their `selected` state.
    #[default]
    None,
    /// Exactly one item is selected at a time.
    Single,
    /// Multiple items may be selected.
    Multi,
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
pub struct MaterialList {
    pub selection_mode: ListSelectionMode,
}

impl MaterialList {
    /// Create a new list
    pub fn new() -> Self {
        Self {
            selection_mode: ListSelectionMode::None,
        }
    }

    pub fn with_selection_mode(mut self, mode: ListSelectionMode) -> Self {
        self.selection_mode = mode;
        self
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

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        let base = if self.selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            let state_color = theme.on_surface;
            if base == Color::NONE {
                state_color.with_alpha(state_opacity)
            } else {
                blend_state_layer(base, state_color, state_opacity)
            }
        } else {
            base
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

fn list_selection_system(
    mut click_events: MessageReader<ListItemClickEvent>,
    parents: Query<&ChildOf>,
    lists: Query<&MaterialList>,
    mut list_data: Query<&mut MaterialListData>,
    children_query: Query<&Children>,
    mut items: Query<&mut MaterialListItem>,
    virtual_rows: Query<(&ListVirtualOwner, &ListVirtualRow)>,
) {
    for event in click_events.read() {
        // If this click came from a virtualized row, selection must be updated in list data.
        if let Ok((owner, row)) = virtual_rows.get(event.entity) {
            let list_entity = owner.0;
            let Ok(list) = lists.get(list_entity) else {
                continue;
            };
            let Ok(mut data) = list_data.get_mut(list_entity) else {
                continue;
            };
            let Some(index) = row.item_index else {
                continue;
            };
            if index >= data.items.len() {
                continue;
            }

            match list.selection_mode {
                ListSelectionMode::None => {}
                ListSelectionMode::Multi => {
                    data.items[index].selected = !data.items[index].selected;
                }
                ListSelectionMode::Single => {
                    for (i, item) in data.items.iter_mut().enumerate() {
                        item.selected = i == index;
                    }
                }
            }

            // The virtualization system will refresh the row pool.
            continue;
        }

        // Find the nearest ancestor that is a MaterialList.
        let mut current = Some(event.entity);
        let mut list_entity = None;
        for _ in 0..MAX_ANCESTOR_DEPTH {
            let Some(e) = current else { break };
            if lists.get(e).is_ok() {
                list_entity = Some(e);
                break;
            }
            current = parents.get(e).ok().map(|p| p.0);
        }

        let Some(list_entity) = list_entity else {
            continue;
        };
        let Ok(list) = lists.get(list_entity) else {
            continue;
        };

        match list.selection_mode {
            ListSelectionMode::None => {}
            ListSelectionMode::Multi => {
                if let Ok(mut clicked) = items.get_mut(event.entity) {
                    clicked.selected = !clicked.selected;
                }
            }
            ListSelectionMode::Single => {
                // Select the clicked item and clear any other selected items in this list.
                // Traverse the list subtree to support wrappers (e.g., scroll content).
                let mut stack: Vec<Entity> = vec![list_entity];
                while let Some(node) = stack.pop() {
                    if let Ok(children) = children_query.get(node) {
                        for child in children.iter() {
                            if let Ok(mut item) = items.get_mut(child) {
                                item.selected = child == event.entity;
                            }
                            stack.push(child);
                        }
                    }
                }

                // If the clicked entity isn't under the list (unexpected), still force it selected.
                if let Ok(mut clicked) = items.get_mut(event.entity) {
                    clicked.selected = true;
                }
            }
        }
    }
}

fn list_virtual_cache_system(
    mut commands: Commands,
    lists: Query<(Entity, &MaterialListData), (With<ScrollableList>, Without<ListVirtualCache>)>,
    changed: Query<(Entity, &MaterialListData), (With<ScrollableList>, Changed<MaterialListData>)>,
) {
    fn build_cache(items: &[ListItemModel]) -> ListVirtualCache {
        let mut prefix_heights: Vec<f32> = Vec::with_capacity(items.len() + 1);
        prefix_heights.push(0.0);

        let mut total = 0.0;
        let mut min_h = f32::INFINITY;
        for item in items.iter() {
            let h = item.variant.height();
            if h < min_h {
                min_h = h;
            }
            total += h;
            prefix_heights.push(total);
        }

        if !min_h.is_finite() {
            min_h = 56.0;
        }

        ListVirtualCache {
            prefix_heights,
            total_height: total,
            min_item_height: min_h,
        }
    }

    for (entity, data) in lists.iter() {
        commands.entity(entity).insert(build_cache(&data.items));
    }

    for (entity, data) in changed.iter() {
        commands.entity(entity).insert(build_cache(&data.items));
    }
}

fn list_virtualization_system(
    mut commands: Commands,
    theme: Option<Res<MaterialTheme>>,
    lists: Query<
        (
            Entity,
            &MaterialListData,
            &ListVirtualCache,
            &ScrollPosition,
            &ComputedNode,
        ),
        With<ScrollableList>,
    >,
    top_spacers: Query<(Entity, &ListVirtualOwner), With<ListVirtualTopSpacer>>,
    bottom_spacers: Query<(Entity, &ListVirtualOwner), With<ListVirtualBottomSpacer>>,
    mut nodes: Query<&mut Node>,
    mut rows: Query<(
        Entity,
        &ListVirtualOwner,
        &mut ListVirtualRow,
        &mut MaterialListItem,
        &mut BackgroundColor,
        Option<&mut Visibility>,
    )>,
) {
    let Some(theme) = theme else {
        return;
    };

    for (list_entity, data, cache, scroll_pos, computed) in lists.iter() {
        if !data.virtualize || data.items.is_empty() {
            continue;
        }

        // Compute viewport and scroll offset.
        let viewport_h = computed.size().y.max(0.0);
        let scroll_y = scroll_pos.y.max(0.0);

        // Determine start index via binary search on prefix heights.
        let prefix = &cache.prefix_heights;
        let mut lo = 0usize;
        let mut hi = data.items.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if prefix[mid + 1] <= scroll_y {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut start = lo;
        if data.overscan_rows > 0 {
            start = start.saturating_sub(data.overscan_rows);
        }

        let mut end = start;
        let target_bottom = (scroll_y + viewport_h).min(cache.total_height);
        while end < data.items.len() && prefix[end] < target_bottom {
            end += 1;
        }
        end = (end + data.overscan_rows).min(data.items.len());

        // If we don't have a row pool yet, create spacers + pool rows.
        let has_pool = top_spacers.iter().any(|(_, owner)| owner.0 == list_entity);
        if !has_pool {
            let estimated_visible = (viewport_h / cache.min_item_height).ceil().max(1.0) as usize;
            let pool_size = (estimated_visible + 2 * data.overscan_rows)
                .min(data.items.len())
                .max(1);

            commands.entity(list_entity).with_children(|parent| {
                parent.spawn((
                    ListVirtualTopSpacer,
                    ListVirtualOwner(list_entity),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(0.0),
                        ..default()
                    },
                ));

                for slot in 0..pool_size {
                    // Spawn a placeholder row; it will be populated on first update.
                    parent
                        .spawn((
                            ListVirtualOwner(list_entity),
                            ListVirtualRow {
                                pool_slot: slot,
                                item_index: None,
                            },
                            MaterialListItem::new(""),
                            Button,
                            crate::ripple::RippleHost::new(),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(ListItemVariant::OneLine.height()),
                                padding: UiRect::axes(
                                    Val::Px(Spacing::LARGE),
                                    Val::Px(Spacing::SMALL),
                                ),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(Spacing::LARGE),
                                flex_shrink: 0.0,
                                min_height: Val::Px(ListItemVariant::OneLine.height()),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            Visibility::Inherited,
                        ))
                        .with_children(|_| {});
                }

                parent.spawn((
                    ListVirtualBottomSpacer,
                    ListVirtualOwner(list_entity),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(0.0),
                        ..default()
                    },
                ));
            });
        }

        // Update spacer heights.
        let top_h = prefix[start];
        let bottom_h = cache.total_height - prefix[end];

        for (entity, owner) in top_spacers.iter() {
            if owner.0 != list_entity {
                continue;
            }
            if let Ok(mut node) = nodes.get_mut(entity) {
                node.height = Val::Px(top_h);
                node.min_height = Val::Px(top_h);
                node.flex_shrink = 0.0;
            }
        }
        for (entity, owner) in bottom_spacers.iter() {
            if owner.0 != list_entity {
                continue;
            }
            if let Ok(mut node) = nodes.get_mut(entity) {
                node.height = Val::Px(bottom_h.max(0.0));
                node.min_height = Val::Px(bottom_h.max(0.0));
                node.flex_shrink = 0.0;
            }
        }

        // Assign visible indices to pool rows by pool slot.
        for (row_entity, owner, mut row, mut item_comp, mut bg, vis) in rows.iter_mut() {
            if owner.0 != list_entity {
                continue;
            }

            let Some(target_index) = start.checked_add(row.pool_slot) else {
                row.item_index = None;
                if let Some(mut visibility) = vis {
                    *visibility = Visibility::Hidden;
                }
                continue;
            };

            if target_index >= end || target_index >= data.items.len() {
                row.item_index = None;
                if let Some(mut visibility) = vis {
                    *visibility = Visibility::Hidden;
                }
                continue;
            }

            if let Some(mut visibility) = vis {
                *visibility = Visibility::Inherited;
            }

            let model = &data.items[target_index];
            let needs_rebuild = row.item_index != Some(target_index);

            // Update component fields.
            if needs_rebuild {
                item_comp.variant = model.variant;
                item_comp.headline = model.headline.clone();
                item_comp.supporting_text = model.supporting_text.clone();
                item_comp.trailing_text = model.trailing_text.clone();
                item_comp.leading_icon = model.leading_icon.clone();
                item_comp.trailing_icon = model.trailing_icon.clone();
                item_comp.pressed = false;
                item_comp.hovered = false;
            }
            item_comp.disabled = model.disabled;
            item_comp.selected = model.selected;

            let h = model.variant.height();
            if let Ok(mut node) = nodes.get_mut(row_entity) {
                node.height = Val::Px(h);
                node.min_height = Val::Px(h);
                node.flex_shrink = 0.0;
            }

            *bg = BackgroundColor(item_comp.background_color(&theme));

            if needs_rebuild {
                commands.queue(DespawnDescendantsIfExists { entity: row_entity });
                let model_clone = model.clone();
                commands.entity(row_entity).with_children(|row_children| {
                    let item_for_colors = MaterialListItem {
                        variant: model_clone.variant,
                        disabled: model_clone.disabled,
                        selected: model_clone.selected,
                        headline: model_clone.headline.clone(),
                        supporting_text: model_clone.supporting_text.clone(),
                        trailing_text: model_clone.trailing_text.clone(),
                        leading_icon: model_clone.leading_icon.clone(),
                        trailing_icon: model_clone.trailing_icon.clone(),
                        leading_avatar: None,
                        leading_video: None,
                        pressed: false,
                        hovered: false,
                    };

                    let headline_color = item_for_colors.headline_color(&theme);
                    let supporting_color = item_for_colors.supporting_text_color(&theme);
                    let icon_color = item_for_colors.icon_color(&theme);

                    // Leading content
                    if let Some(icon_str) = model_clone.leading_icon.as_deref() {
                        if let Some(icon_id) = resolve_icon_id(icon_str) {
                            row_children
                                .spawn((
                                    ListItemLeading,
                                    Node {
                                        width: Val::Px(56.0),
                                        height: Val::Px(56.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                ))
                                .with_children(|leading| {
                                    leading.spawn((
                                        MaterialIcon::new(icon_id),
                                        IconStyle::outlined()
                                            .with_color(icon_color)
                                            .with_size(24.0),
                                    ));
                                });
                        }
                    }

                    // Body
                    row_children
                        .spawn((
                            ListItemBody,
                            Node {
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                ..default()
                            },
                        ))
                        .with_children(|body| {
                            body.spawn((
                                ListItemHeadline,
                                Text::new(&model_clone.headline),
                                TextFont {
                                    font_size: FontSize::Px(16.0),
                                    ..default()
                                },
                                TextColor(headline_color),
                            ));

                            if let Some(ref supporting) = model_clone.supporting_text {
                                body.spawn((
                                    ListItemSupportingText,
                                    Text::new(supporting),
                                    TextFont {
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(supporting_color),
                                ));
                            }
                        });

                    // Trailing
                    if model_clone.trailing_text.is_some() || model_clone.trailing_icon.is_some() {
                        row_children
                            .spawn((
                                ListItemTrailing,
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(Spacing::MEDIUM),
                                    ..default()
                                },
                            ))
                            .with_children(|trailing| {
                                if let Some(ref text) = model_clone.trailing_text {
                                    trailing.spawn((
                                        Text::new(text),
                                        TextFont {
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(supporting_color),
                                    ));
                                }

                                if let Some(icon_str) = model_clone.trailing_icon.as_deref() {
                                    if let Some(icon_id) = resolve_icon_id(icon_str) {
                                        trailing.spawn((
                                            MaterialIcon::new(icon_id),
                                            IconStyle::outlined()
                                                .with_color(icon_color)
                                                .with_size(24.0),
                                        ));
                                    }
                                }
                            });
                    }
                });
            }

            row.item_index = Some(target_index);
        }
    }
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

/// System to update list item text colors when item state changes
fn list_item_text_style_system(
    theme: Option<Res<MaterialTheme>>,
    changed_items: Query<(&MaterialListItem, &Children), Changed<MaterialListItem>>,
    mut headline_texts: Query<&mut TextColor, With<ListItemHeadline>>,
    mut supporting_texts: Query<
        &mut TextColor,
        (With<ListItemSupportingText>, Without<ListItemHeadline>),
    >,
    children_query: Query<&Children>,
) {
    let Some(theme) = theme else { return };

    for (item, children) in changed_items.iter() {
        let headline_color = item.headline_color(&theme);
        let supporting_color = item.supporting_text_color(&theme);

        // Update direct children
        for child in children.iter() {
            if let Ok(mut text_color) = headline_texts.get_mut(child) {
                *text_color = TextColor(headline_color);
            }
            if let Ok(mut text_color) = supporting_texts.get_mut(child) {
                *text_color = TextColor(supporting_color);
            }

            // Check nested children (for body containers)
            if let Ok(grandchildren) = children_query.get(child) {
                for grandchild in grandchildren.iter() {
                    if let Ok(mut text_color) = headline_texts.get_mut(grandchild) {
                        *text_color = TextColor(headline_color);
                    }
                    if let Ok(mut text_color) = supporting_texts.get_mut(grandchild) {
                        *text_color = TextColor(supporting_color);
                    }
                }
            }
        }
    }
}

/// Builder for lists
pub struct ListBuilder {
    /// Maximum height before scrolling (None = no limit)
    max_height: Option<f32>,
    /// Whether to show scrollbar
    show_scrollbar: bool,
    /// Selection behavior
    selection_mode: ListSelectionMode,
    /// Data-backed items for optional virtualization.
    items: Vec<ListItemModel>,
    /// Whether to render using a fixed pool of rows.
    virtualize: bool,
    /// Extra rows to render above/below the viewport.
    overscan_rows: usize,
}

impl ListBuilder {
    /// Create a new list builder
    pub fn new() -> Self {
        Self {
            max_height: None,
            show_scrollbar: true,
            selection_mode: ListSelectionMode::None,
            items: Vec::new(),
            virtualize: false,
            overscan_rows: 2,
        }
    }

    /// Provide data-backed items for the list.
    ///
    /// This is required when using `.virtualize(true)`.
    pub fn items(mut self, items: Vec<ListItemModel>) -> Self {
        self.items = items;
        self
    }

    /// Provide data-backed items from `ListItemBuilder`s.
    pub fn items_from_builders(mut self, items: Vec<ListItemBuilder>) -> Self {
        self.items = items.into_iter().map(|b| b.item.into()).collect();
        self
    }

    /// Enable/disable list virtualization.
    ///
    /// When enabled (and `items` is non-empty), the list will reuse a small pool of row entities
    /// and update their content as the user scrolls.
    pub fn virtualize(mut self, virtualize: bool) -> Self {
        self.virtualize = virtualize;
        self
    }

    /// Set the number of overscan rows for virtualization.
    pub fn overscan_rows(mut self, rows: usize) -> Self {
        self.overscan_rows = rows;
        self
    }

    /// Set list selection behavior.
    pub fn selection_mode(mut self, mode: ListSelectionMode) -> Self {
        self.selection_mode = mode;
        self
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
            MaterialList::new().with_selection_mode(self.selection_mode),
            MaterialListData {
                items: self.items,
                virtualize: self.virtualize,
                overscan_rows: self.overscan_rows,
            },
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
            MaterialList::new().with_selection_mode(self.selection_mode),
            MaterialListData {
                items: self.items,
                virtualize: self.virtualize,
                overscan_rows: self.overscan_rows,
            },
            ScrollableList,
            ScrollContainerBuilder::new()
                .vertical()
                .with_scrollbars(self.show_scrollbar)
                .build(),
            ScrollPosition::default(),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height,
                max_height: height,
                // Important for scroll containers inside flex columns:
                // allow shrinking so overflow/scrolling can happen.
                min_height: Val::Px(0.0),
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                // Bevy's scroll system expects both axes to be `Scroll`.
                // Actual scroll direction is controlled by `ScrollContainer.direction`.
                overflow: Overflow::scroll(),
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
                flex_direction: FlexDirection::Row,
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

/// Marker for headline text in list item
#[derive(Component)]
pub struct ListItemHeadline;

/// Marker for supporting text in list item
#[derive(Component)]
pub struct ListItemSupportingText;

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

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material lists and list items as children
///
/// This trait provides a clean API for spawning lists within UI hierarchies.
///
/// ## Example:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::list::SpawnListChild;
/// use bevy_material_ui::theme::MaterialTheme;
///
/// fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
///     commands.spawn(Node::default()).with_children(|children| {
///         children.spawn_list(|list| {
///             list.spawn_list_item(&theme, "Item 1", None::<String>);
///             list.spawn_list_item(&theme, "Item 2", Some("Supporting text"));
///         });
///     });
/// }
/// ```
pub trait SpawnListChild {
    /// Spawn a list container
    fn spawn_list(&mut self, with_children: impl FnOnce(&mut ChildSpawnerCommands));

    /// Spawn a list item with headline and optional supporting text
    fn spawn_list_item(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        supporting: Option<impl Into<String>>,
    );

    /// Spawn a list item with full builder control
    fn spawn_list_item_with(&mut self, theme: &MaterialTheme, builder: ListItemBuilder);

    /// Spawn a list divider
    fn spawn_list_divider(&mut self, theme: &MaterialTheme, inset: bool);
}

impl SpawnListChild for ChildSpawnerCommands<'_> {
    fn spawn_list(&mut self, with_children: impl FnOnce(&mut ChildSpawnerCommands)) {
        self.spawn(ListBuilder::new().build())
            .with_children(with_children);
    }

    fn spawn_list_item(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        supporting: Option<impl Into<String>>,
    ) {
        let headline_str = headline.into();
        let supporting_str = supporting.map(|s| s.into());
        let has_supporting = supporting_str.is_some();

        let builder = if has_supporting {
            ListItemBuilder::new(&headline_str).two_line()
        } else {
            ListItemBuilder::new(&headline_str)
        };

        let headline_color = theme.on_surface;
        let supporting_color = theme.on_surface_variant;

        self.spawn(builder.build(theme)).with_children(|item| {
            // Body content
            item.spawn((
                ListItemBody,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
            ))
            .with_children(|body| {
                // Headline
                body.spawn((
                    ListItemHeadline,
                    Text::new(&headline_str),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                // Supporting text (if provided)
                if let Some(ref supporting) = supporting_str {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(supporting),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(supporting_color),
                    ));
                }
            });
        });
    }

    fn spawn_list_item_with(&mut self, theme: &MaterialTheme, builder: ListItemBuilder) {
        let headline = builder.item.headline.clone();
        let supporting_text = builder.item.supporting_text.clone();
        let trailing_text = builder.item.trailing_text.clone();
        let leading_icon = builder.item.leading_icon.clone();
        let trailing_icon = builder.item.trailing_icon.clone();

        let headline_color = builder.item.headline_color(theme);
        let supporting_color = builder.item.supporting_text_color(theme);
        let icon_color = builder.item.icon_color(theme);

        self.spawn(builder.build(theme)).with_children(|item| {
            // Leading content
            if let Some(icon_str) = leading_icon.as_deref() {
                if let Some(icon_id) = resolve_icon_id(icon_str) {
                    item.spawn((
                        ListItemLeading,
                        Node {
                            width: Val::Px(56.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|leading| {
                        leading.spawn((
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });
                }
            }

            // Body
            item.spawn((
                ListItemBody,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
            ))
            .with_children(|body| {
                body.spawn((
                    ListItemHeadline,
                    Text::new(&headline),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                if let Some(ref supporting) = supporting_text {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(supporting),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(supporting_color),
                    ));
                }
            });

            // Trailing content
            if trailing_text.is_some() || trailing_icon.is_some() {
                item.spawn((
                    ListItemTrailing,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::MEDIUM),
                        ..default()
                    },
                ))
                .with_children(|trailing| {
                    if let Some(ref text) = trailing_text {
                        trailing.spawn((
                            Text::new(text),
                            TextFont {
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                            TextColor(supporting_color),
                        ));
                    }

                    if let Some(icon_str) = trailing_icon.as_deref() {
                        if let Some(icon_id) = resolve_icon_id(icon_str) {
                            trailing.spawn((
                                MaterialIcon::new(icon_id),
                                IconStyle::outlined().with_color(icon_color).with_size(24.0),
                            ));
                        }
                    }
                });
            }
        });
    }

    fn spawn_list_divider(&mut self, theme: &MaterialTheme, inset: bool) {
        self.spawn(create_list_divider(theme, inset));
    }
}
