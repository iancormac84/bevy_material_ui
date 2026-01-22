//! Material Design 3 Select (Dropdown) component
//!
//! Select menus display a list of choices on a temporary surface and allow users to select one.
//! Reference: <https://m3.material.io/components/menus/overview>

use bevy::prelude::*;

use crate::{
    i18n::{MaterialI18n, MaterialLanguage, MaterialLanguageOverride},
    icons::{icon_by_name, MaterialIcon, ICON_EXPAND_MORE},
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

use crate::scroll::ScrollContainer;
use crate::scroll::ScrollContent;

/// Plugin for the select component
pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<SelectChangeEvent>().add_systems(
            Update,
            (
                select_interaction_system,
                select_style_system,
                select_content_style_system,
                select_theme_refresh_system,
                select_localization_system,
                select_dropdown_rebuild_options_system,
                select_dropdown_sync_system,
                select_dropdown_virtualization_system,
                select_option_interaction_system,
                select_telemetry_system,
            ),
        );
    }
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectLocalization {
    pub label_key: Option<String>,
    pub supporting_text_key: Option<String>,
    pub error_text_key: Option<String>,
}

impl SelectLocalization {
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.label_key = Some(key.into());
        self
    }

    pub fn supporting_text_key(mut self, key: impl Into<String>) -> Self {
        self.supporting_text_key = Some(key.into());
        self
    }

    pub fn error_text_key(mut self, key: impl Into<String>) -> Self {
        self.error_text_key = Some(key.into());
        self
    }

    fn is_enabled(&self) -> bool {
        self.label_key.is_some()
            || self.supporting_text_key.is_some()
            || self.error_text_key.is_some()
    }
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
struct SelectLocalizationState {
    last_revision: u64,
    last_language: String,
}

fn resolve_language_tag(
    mut entity: Entity,
    child_of: &Query<&ChildOf>,
    overrides: &Query<&MaterialLanguageOverride>,
    global: &MaterialLanguage,
) -> String {
    if let Ok(ov) = overrides.get(entity) {
        return ov.tag.clone();
    }

    while let Ok(parent) = child_of.get(entity) {
        entity = parent.parent();
        if let Ok(ov) = overrides.get(entity) {
            return ov.tag.clone();
        }
    }

    global.tag.clone()
}

fn select_localization_system(
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut selects: Query<(
        Entity,
        &SelectLocalization,
        &mut MaterialSelect,
        Option<&mut SelectLocalizationState>,
    )>,
    mut commands: Commands,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let global_revision = i18n.revision();

    for (entity, loc, mut select, state) in selects.iter_mut() {
        if !loc.is_enabled() {
            continue;
        }

        let resolved_language = resolve_language_tag(entity, &child_of, &overrides, &language);

        let needs_update = match &state {
            Some(s) => s.last_revision != global_revision || s.last_language != resolved_language,
            None => true,
        };

        if !needs_update {
            continue;
        }

        if let Some(key) = loc.label_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.label.as_deref() != Some(next.as_str()) {
                    select.label = Some(next);
                }
            }
        }

        if let Some(key) = loc.supporting_text_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.supporting_text.as_deref() != Some(next.as_str()) {
                    select.supporting_text = Some(next);
                }
            }
        }

        if let Some(key) = loc.error_text_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.error_text.as_deref() != Some(next.as_str()) {
                    select.error_text = Some(next);
                }
            }
        }

        // Localize option labels.
        for option in select.options.iter_mut() {
            let Some(key) = option.label_key.as_deref() else {
                continue;
            };

            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if option.label != next {
                    option.label = next;
                }
            }
        }

        if let Some(mut state) = state {
            state.last_revision = global_revision;
            state.last_language = resolved_language;
        } else {
            commands.entity(entity).insert(SelectLocalizationState {
                last_revision: global_revision,
                last_language: resolved_language,
            });
        }
    }
}

fn select_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    selects: Query<(&TestId, &Children), With<MaterialSelect>>,
    children_query: Query<&Children>,
    virtual_rows: Query<(), With<SelectVirtualRow>>,
    mut queries: ParamSet<(
        Query<(), With<SelectDisplayText>>,
        Query<(), With<SelectDropdownArrow>>,
        Query<(), With<SelectDropdown>>,
        Query<&SelectOptionItem>,
        Query<(), With<SelectOptionLabelText>>,
        Query<(), With<SelectOptionIcon>>,
    )>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (test_id, children) in selects.iter() {
        let base = test_id.id();

        let mut found_display = false;
        let mut found_arrow = false;
        let mut found_dropdown = false;

        let mut options: Vec<(Entity, usize)> = Vec::new();

        let mut stack: Vec<Entity> = children.iter().collect();
        while let Some(entity) = stack.pop() {
            if !found_display && queries.p0().get(entity).is_ok() {
                found_display = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/display_text"),
                });
            }

            if !found_arrow && queries.p1().get(entity).is_ok() {
                found_arrow = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/arrow"),
                });
            }

            if !found_dropdown && queries.p2().get(entity).is_ok() {
                found_dropdown = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/dropdown"),
                });
            }

            if let Ok(option) = queries.p3().get(entity) {
                // Virtualized rows are reused for multiple indices as you scroll; tagging them
                // with an index-based test id would be misleading.
                if virtual_rows.get(entity).is_ok() {
                    continue;
                }
                let option_base = format!("{base}/option/{}", option.index);
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: option_base.clone(),
                });
                options.push((entity, option.index));
            }

            if let Ok(children) = children_query.get(entity) {
                stack.extend(children.iter());
            }
        }

        // Tag label/icon nodes under each option row with stable derived IDs.
        for (row_entity, index) in options {
            let Ok(children) = children_query.get(row_entity) else {
                continue;
            };

            let mut found_label = false;
            let mut found_icon = false;
            let mut stack: Vec<Entity> = children.iter().collect();
            while let Some(entity) = stack.pop() {
                if !found_icon && queries.p5().get(entity).is_ok() {
                    found_icon = true;
                    commands.queue(InsertTestIdIfExists {
                        entity,
                        id: format!("{base}/option/{index}/icon"),
                    });
                }

                if !found_label && queries.p4().get(entity).is_ok() {
                    found_label = true;
                    commands.queue(InsertTestIdIfExists {
                        entity,
                        id: format!("{base}/option/{index}/label"),
                    });
                }

                if found_label && found_icon {
                    break;
                }

                if let Ok(children) = children_query.get(entity) {
                    stack.extend(children.iter());
                }
            }
        }
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
    /// Optional i18n key for the label.
    pub label_key: Option<String>,
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
            label_key: None,
            value: None,
            icon: None,
            disabled: false,
        }
    }

    /// Set the label from an i18n key.
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.label = String::new();
        self.label_key = Some(key.into());
        self
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

const SELECT_DROPDOWN_PADDING_Y: f32 = 8.0;
const SELECT_DROPDOWN_PADDING_TOTAL: f32 = SELECT_DROPDOWN_PADDING_Y * 2.0;

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

/// Update select child visuals (text colors, dropdown surface, option selection highlight)
/// whenever select state changes.
fn select_content_style_system(
    theme: Option<Res<MaterialTheme>>,
    changed_selects: Query<Entity, Changed<MaterialSelect>>,
    selects: Query<&MaterialSelect>,
    mut text_colors: ParamSet<(
        Query<(&ChildOf, &mut TextColor), With<SelectDisplayText>>,
        Query<(&ChildOf, &mut MaterialIcon), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut MaterialIcon, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (
            With<SelectDropdown>,
            Without<SelectOptionItem>,
            Without<MaterialSelect>,
        ),
    >,
    mut option_rows: Query<
        (
            &SelectOwner,
            &SelectOptionItem,
            &mut BackgroundColor,
            &Children,
        ),
        (Without<SelectDropdown>, Without<MaterialSelect>),
    >,
) {
    let Some(theme) = theme else { return };
    if changed_selects.iter().next().is_none() {
        return;
    }

    for (parent, mut color) in text_colors.p0().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.text_color(&theme);
        }
    }

    for (parent, mut color) in text_colors.p1().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.color = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else {
            continue;
        };

        let is_selected = select
            .selected_index
            .is_some_and(|i| i == option_item.index);
        row_bg.0 = if is_selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        let base = theme.on_surface;
        let is_disabled = select
            .options
            .get(option_item.index)
            .is_some_and(|o| o.disabled);
        let text_color = if is_disabled {
            base.with_alpha(0.38)
        } else {
            base
        };

        for child in children.iter() {
            if let Ok(mut c) = text_colors.p2().get_mut(child) {
                c.0 = text_color;
            }
            if let Ok(mut c) = text_colors.p3().get_mut(child) {
                c.color = text_color;
            }
        }
    }
}

/// Refresh select visuals when the theme changes.
fn select_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    selects: Query<&MaterialSelect>,
    mut triggers: Query<
        (&MaterialSelect, &mut BackgroundColor, &mut BorderColor),
        (Without<SelectDropdown>, Without<SelectOptionItem>),
    >,
    mut text_colors: ParamSet<(
        Query<(&ChildOf, &mut TextColor), With<SelectDisplayText>>,
        Query<(&ChildOf, &mut MaterialIcon), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut MaterialIcon, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (
            With<SelectDropdown>,
            Without<SelectOptionItem>,
            Without<MaterialSelect>,
        ),
    >,
    mut option_rows: Query<
        (
            &SelectOwner,
            &SelectOptionItem,
            &mut BackgroundColor,
            &Children,
        ),
        (Without<SelectDropdown>, Without<MaterialSelect>),
    >,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (select, mut bg, mut border) in triggers.iter_mut() {
        bg.0 = select.container_color(&theme);
        *border = BorderColor::all(select.indicator_color(&theme));
    }

    for (parent, mut color) in text_colors.p0().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.text_color(&theme);
        }
    }

    for (parent, mut color) in text_colors.p1().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.color = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else {
            continue;
        };

        let is_selected = select
            .selected_index
            .is_some_and(|i| i == option_item.index);
        row_bg.0 = if is_selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        let base = theme.on_surface;
        let is_disabled = select
            .options
            .get(option_item.index)
            .is_some_and(|o| o.disabled);
        let text_color = if is_disabled {
            base.with_alpha(0.38)
        } else {
            base
        };

        for child in children.iter() {
            if let Ok(mut c) = text_colors.p2().get_mut(child) {
                c.0 = text_color;
            }
            if let Ok(mut c) = text_colors.p3().get_mut(child) {
                c.color = text_color;
            }
        }
    }
}

/// Builder for select components
pub struct SelectBuilder {
    select: MaterialSelect,
    width: Val,
    localization: SelectLocalization,
    dropdown_max_height: Option<Val>,
    dropdown_virtualize: bool,
}

impl SelectBuilder {
    /// Create a new select builder
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            select: MaterialSelect::new(options),
            width: Val::Px(210.0),
            localization: SelectLocalization::default(),
            dropdown_max_height: None,
            dropdown_virtualize: false,
        }
    }

    /// Set a maximum height for the dropdown list.
    ///
    /// When set, the dropdown becomes vertically scrollable so long lists don't expand
    /// beyond this height.
    pub fn dropdown_max_height(mut self, height: Val) -> Self {
        self.dropdown_max_height = Some(height);
        self
    }

    /// Enable dropdown virtualization.
    ///
    /// When enabled, the dropdown only spawns a small fixed pool of option row entities and
    /// reuses them as you scroll. This significantly improves performance for very large option
    /// lists.
    ///
    /// Note: virtualization requires `dropdown_max_height(...)` so the dropdown is scrollable.
    pub fn virtualize(mut self, enabled: bool) -> Self {
        self.dropdown_virtualize = enabled;
        self
    }

    /// Localize the select's placeholder/label via a translation key.
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.label_key(key);
        self
    }

    /// Localize the select's supporting text via a translation key.
    pub fn supporting_text_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.supporting_text_key(key);
        self
    }

    /// Localize the select's error text via a translation key.
    pub fn error_text_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.error_text_key(key);
        self
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
            self.localization,
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
                border_radius: BorderRadius::top(Val::Px(CornerRadius::EXTRA_SMALL)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
        )
    }
}

/// Marker for select dropdown
#[derive(Component)]
pub struct SelectDropdown;

/// Internal marker for the dropdown's scroll/content container.
///
/// Option rows are spawned under this entity.
#[derive(Component)]
struct SelectDropdownContent;

/// Optional max height configuration for a select dropdown.
///
/// When present on `SelectDropdownContent`, the dropdown viewport height is computed as:
/// `min(max_height, content_height)` so the dropdown only becomes scrollable when needed.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
struct SelectDropdownMaxHeight(pub Val);

/// Internal marker for a virtualized dropdown list.
///
/// Virtualized dropdowns render a fixed pool of rows with top/bottom spacers.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
struct SelectDropdownVirtualized {
    pool_size: usize,
}

#[derive(Component)]
struct SelectVirtualTopSpacer;

#[derive(Component)]
struct SelectVirtualBottomSpacer;

#[derive(Component)]
struct SelectVirtualRow;

#[derive(Component)]
struct SelectVirtualIcon;

/// Internal marker for option icons rendered as embedded bitmaps.
#[derive(Component)]
struct SelectOptionIcon;

/// Internal marker used to route option clicks back to the owning select.
#[derive(Component, Clone, Copy)]
struct SelectOwner(Entity);

/// Marker for select option item (component attached to each option in the dropdown)
#[derive(Component)]
pub struct SelectOptionItem {
    /// Index of this option in the options list
    pub index: usize,
    /// Display label for this option
    pub label: String,
}

/// Marker for select container (parent of trigger and dropdown)
#[derive(Component)]
pub struct SelectContainer;

/// Marker for select trigger button
#[derive(Component)]
pub struct SelectTrigger {
    /// Available options
    #[allow(dead_code)]
    pub options: Vec<String>,
    /// Currently selected index
    pub selected_index: usize,
}

/// Marker for select's displayed text
#[derive(Component)]
pub struct SelectDisplayText;

/// Marker for the dropdown arrow text node.
#[derive(Component)]
pub struct SelectDropdownArrow;

/// Marker for select option label text nodes.
#[derive(Component)]
pub struct SelectOptionLabelText;

/// Rebuild dropdown option rows when `MaterialSelect.options` changes.
///
/// The select component spawns option rows at build time. Some UIs (like the
/// showcase Translations view) populate options dynamically after spawn.
fn select_dropdown_rebuild_options_system(
    theme: Option<Res<MaterialTheme>>,
    selects: Query<(Entity, &MaterialSelect, &Children), Changed<MaterialSelect>>,
    dropdowns: Query<(), With<SelectDropdown>>,
    dropdown_contents: Query<(), With<SelectDropdownContent>>,
    dropdown_max_heights: Query<&SelectDropdownMaxHeight>,
    mut dropdown_virtualized: Query<&mut SelectDropdownVirtualized>,
    mut dropdown_content_nodes: Query<&mut Node, With<SelectDropdownContent>>,
    dropdown_children: Query<&Children, With<SelectDropdown>>,
    content_children: Query<&Children, With<SelectDropdownContent>>,
    is_scroll_content: Query<(), With<ScrollContent>>,
    virtual_top_spacers: Query<(), With<SelectVirtualTopSpacer>>,
    virtual_rows: Query<(), With<SelectVirtualRow>>,
    virtual_bottom_spacers: Query<(), With<SelectVirtualBottomSpacer>>,
    option_rows: Query<(), With<SelectOptionItem>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    let Some(theme) = theme else { return };

    for (select_entity, select, children) in selects.iter() {
        // Find dropdown child.
        let Some(dropdown_entity) = children.iter().find(|e| dropdowns.get(*e).is_ok()) else {
            continue;
        };

        // Find the content container where option rows live.
        let content_entity = dropdown_children
            .get(dropdown_entity)
            .ok()
            .and_then(|kids| kids.iter().find(|e| dropdown_contents.get(*e).is_ok()))
            .unwrap_or(dropdown_entity);

        // For scrollable dropdowns, the UI system re-parents children under a `ScrollContent`
        // wrapper. Look there when enumerating rows so we don't accidentally spawn duplicates.
        let mut list_root = content_entity;
        if let Ok(kids) = children_query.get(content_entity) {
            if let Some(wrapper) = kids
                .iter()
                .find(|&child| is_scroll_content.get(child).is_ok())
            {
                list_root = wrapper;
            }
        }

        // Virtualized dropdowns manage their own row pool; don't despawn/rebuild rows here.
        // We still update the viewport height (below) so the dropdown only scrolls when needed.
        if let Ok(mut virt) = dropdown_virtualized.get_mut(content_entity) {
            let options = select.options.clone();

            // Keep viewport height in sync.
            let mut desired_pool_size = virt.pool_size;
            if let Ok(max_h) = dropdown_max_heights.get(content_entity).copied() {
                if let Ok(mut node) = dropdown_content_nodes.get_mut(content_entity) {
                    match max_h.0 {
                        Val::Px(max_px) => {
                            let items_px = (options.len() as f32) * SELECT_OPTION_HEIGHT;
                            let viewport_items_px = items_px.min(max_px);

                            node.height =
                                Val::Px(SELECT_DROPDOWN_PADDING_TOTAL + viewport_items_px);
                            node.max_height = Val::Px(SELECT_DROPDOWN_PADDING_TOTAL + max_px);

                            // Resize the row pool based on the (items) viewport.
                            let visible =
                                (viewport_items_px / SELECT_OPTION_HEIGHT).ceil() as usize;
                            desired_pool_size = (visible + 2).max(1);
                        }
                        other => {
                            node.height = other;
                            node.max_height = other;
                        }
                    }
                }
            }

            // If the dropdown was spawned with empty options (common for dynamic population),
            // the initial pool can be too small. Rebuild the pool when it needs to grow/shrink.
            if desired_pool_size != virt.pool_size {
                virt.pool_size = desired_pool_size;

                // Remove old virtual rows + spacers.
                if let Ok(kids) = children_query.get(list_root) {
                    let mut to_remove = Vec::new();
                    for child in kids.iter() {
                        if virtual_top_spacers.get(child).is_ok()
                            || virtual_rows.get(child).is_ok()
                            || virtual_bottom_spacers.get(child).is_ok()
                        {
                            to_remove.push(child);
                        }
                    }

                    for row in to_remove {
                        let mut stack = vec![row];
                        let mut to_despawn = Vec::new();
                        while let Some(e) = stack.pop() {
                            to_despawn.push(e);
                            if let Ok(kids) = children_query.get(e) {
                                stack.extend(kids.iter());
                            }
                        }

                        for e in to_despawn.into_iter().rev() {
                            commands.entity(e).despawn();
                        }
                    }
                }

                // Spawn new virtual pool.
                let icon_default = icon_by_name("check").expect("embedded icon 'check' not found");
                let option_text_color = theme.on_surface;
                let selected_index = select.selected_index;

                commands.entity(list_root).with_children(|list| {
                    list.spawn((
                        SelectVirtualTopSpacer,
                        Node {
                            height: Val::Px(0.0),
                            min_height: Val::Px(0.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                    ));

                    for pool_index in 0..desired_pool_size {
                        let (index, label) = options
                            .get(pool_index)
                            .map(|o| (pool_index, o.label.clone()))
                            .unwrap_or((usize::MAX, String::new()));

                        let is_disabled = options.get(index).is_some_and(|o| o.disabled);
                        let is_selected = selected_index.is_some_and(|i| i == index);
                        let row_bg = if is_selected {
                            theme.secondary_container
                        } else {
                            Color::NONE
                        };

                        list.spawn((
                            SelectVirtualRow,
                            SelectOwner(select_entity),
                            SelectOptionItem {
                                index,
                                label: label.clone(),
                            },
                            Button,
                            Interaction::None,
                            Node {
                                height: Val::Px(SELECT_OPTION_HEIGHT),
                                min_height: Val::Px(SELECT_OPTION_HEIGHT),
                                flex_shrink: 0.0,
                                padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(Spacing::MEDIUM),
                                ..default()
                            },
                            BackgroundColor(row_bg),
                        ))
                        .with_children(|row| {
                            row.spawn((
                                SelectVirtualIcon,
                                SelectOptionIcon,
                                MaterialIcon::new(icon_default)
                                    .with_size(20.0)
                                    .with_color(option_text_color),
                                Node {
                                    display: Display::None,
                                    ..default()
                                },
                            ));

                            row.spawn((
                                SelectOptionLabelText,
                                Text::new(label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(if is_disabled {
                                    option_text_color.with_alpha(0.38)
                                } else {
                                    option_text_color
                                }),
                            ));
                        });
                    }

                    list.spawn((
                        SelectVirtualBottomSpacer,
                        Node {
                            height: Val::Px(0.0),
                            min_height: Val::Px(0.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                    ));
                });
            }

            continue;
        }

        // Collect current option row entities.
        // Note: If the container was spawned with zero children, it won't have a `Children`
        // component, so we treat a missing Children as "zero existing rows".
        let existing_rows: Vec<Entity> = content_children
            .get(list_root)
            .map(|kids| {
                kids.iter()
                    .filter(|e| option_rows.get(*e).is_ok())
                    .collect()
            })
            .unwrap_or_default();

        if existing_rows.len() == select.options.len() {
            continue;
        }

        // Remove old rows.
        for row in existing_rows {
            let mut stack = vec![row];
            let mut to_despawn = Vec::new();
            while let Some(e) = stack.pop() {
                to_despawn.push(e);
                if let Ok(kids) = children_query.get(e) {
                    stack.extend(kids.iter());
                }
            }

            for e in to_despawn.into_iter().rev() {
                commands.entity(e).despawn();
            }
        }

        // Spawn new rows.
        let option_text_color = theme.on_surface;
        let selected_index = select.selected_index;
        let options = select.options.clone();

        // If this dropdown is configured with a max height, compute the viewport height
        // based on the number of options so rows keep the standard height.
        if let Ok(max_h) = dropdown_max_heights.get(content_entity).copied() {
            if let Ok(mut node) = dropdown_content_nodes.get_mut(content_entity) {
                match max_h.0 {
                    Val::Px(max_px) => {
                        // `dropdown_max_height(Px)` caps the visible *items* area, not the padding.
                        // This keeps the "N rows" mental model intuitive (e.g. 240px => 5 rows at 48px).
                        let items_px = (options.len() as f32) * SELECT_OPTION_HEIGHT;
                        let viewport_items_px = items_px.min(max_px);

                        node.height = Val::Px(SELECT_DROPDOWN_PADDING_TOTAL + viewport_items_px);
                        node.max_height = Val::Px(SELECT_DROPDOWN_PADDING_TOTAL + max_px);
                    }
                    other => {
                        // For non-pixel values we can't compute content height reliably,
                        // so use the configured height as the viewport height.
                        node.height = other;
                        node.max_height = other;
                    }
                }
            }
        }

        commands.entity(content_entity).with_children(|dropdown| {
            for (index, option) in options.iter().enumerate() {
                let is_disabled = option.disabled;
                let is_selected = selected_index.is_some_and(|i| i == index);
                let row_bg = if is_selected {
                    theme.secondary_container
                } else {
                    Color::NONE
                };

                dropdown
                    .spawn((
                        SelectOwner(select_entity),
                        SelectOptionItem {
                            index,
                            label: option.label.clone(),
                        },
                        Button,
                        Interaction::None,
                        Node {
                            height: Val::Px(SELECT_OPTION_HEIGHT),
                            min_height: Val::Px(SELECT_OPTION_HEIGHT),
                            flex_shrink: 0.0,
                            padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(Spacing::MEDIUM),
                            ..default()
                        },
                        BackgroundColor(row_bg),
                    ))
                    .with_children(|row| {
                        if let Some(icon) = &option.icon {
                            if let Some(id) = icon_by_name(icon.as_str()) {
                                row.spawn((
                                    SelectOptionIcon,
                                    MaterialIcon::new(id).with_size(20.0).with_color(
                                        if is_disabled {
                                            option_text_color.with_alpha(0.38)
                                        } else {
                                            option_text_color
                                        },
                                    ),
                                ));
                            }
                        }

                        row.spawn((
                            SelectOptionLabelText,
                            Text::new(option.label.clone()),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(if is_disabled {
                                option_text_color.with_alpha(0.38)
                            } else {
                                option_text_color
                            }),
                        ));
                    });
            }
        });
    }
}

/// Keep dropdown visibility + displayed text in sync with `MaterialSelect`.
fn select_dropdown_sync_system(
    mut selects: Query<(Entity, &MaterialSelect, &Children), Changed<MaterialSelect>>,
    mut dropdowns: Query<&mut Visibility, With<SelectDropdown>>,
    mut display_texts: Query<&mut Text, (With<SelectDisplayText>, Without<SelectOptionLabelText>)>,
    mut option_rows: Query<(&SelectOwner, &mut SelectOptionItem, &Children)>,
    mut option_labels: Query<&mut Text, (With<SelectOptionLabelText>, Without<SelectDisplayText>)>,
) {
    for (select_entity, select, children) in selects.iter_mut() {
        // Update dropdown visibility
        for child in children.iter() {
            if let Ok(mut vis) = dropdowns.get_mut(child) {
                *vis = if select.open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }

        // Update displayed text
        let placeholder = select.label.as_deref().unwrap_or("");

        let display = select
            .selected_option()
            .map(|o| o.label.as_str())
            .unwrap_or(placeholder);

        for child in children.iter() {
            if let Ok(mut text) = display_texts.get_mut(child) {
                *text = Text::new(display);
            }
        }

        // Update option row labels in the dropdown.
        for (owner, mut option_item, row_children) in option_rows.iter_mut() {
            if owner.0 != select_entity {
                continue;
            }

            let Some(opt) = select.options.get(option_item.index) else {
                continue;
            };

            if option_item.label != opt.label {
                option_item.label = opt.label.clone();
            }

            for child in row_children.iter() {
                if let Ok(mut text) = option_labels.get_mut(child) {
                    *text = Text::new(opt.label.clone());
                }
            }
        }
    }
}

/// Handle clicks on option items.
fn select_option_interaction_system(
    mut interactions: Query<(&Interaction, &SelectOptionItem, &SelectOwner), Changed<Interaction>>,
    mut selects: Query<(Entity, &mut MaterialSelect)>,
    mut events: MessageWriter<SelectChangeEvent>,
) {
    for (interaction, option_item, owner) in interactions.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok((select_entity, mut select)) = selects.get_mut(owner.0) else {
            continue;
        };

        // Ignore disabled options
        let Some(option) = select.options.get(option_item.index).cloned() else {
            continue;
        };
        if option.disabled {
            continue;
        }

        select.selected_index = Some(option_item.index);
        select.open = false;
        select.focused = true;

        events.write(SelectChangeEvent {
            entity: select_entity,
            index: option_item.index,
            option,
        });
    }
}

// (no icon font system; icons are embedded bitmaps)

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material selects as children
pub trait SpawnSelectChild {
    /// Spawn a filled select
    fn spawn_filled_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    );

    /// Spawn an outlined select
    fn spawn_outlined_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    );

    /// Spawn a select with full builder control
    fn spawn_select_with(&mut self, theme: &MaterialTheme, builder: SelectBuilder);
}

impl SpawnSelectChild for ChildSpawnerCommands<'_> {
    fn spawn_filled_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    ) {
        self.spawn_select_with(theme, SelectBuilder::new(options).label(label).filled());
    }

    fn spawn_outlined_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    ) {
        self.spawn_select_with(theme, SelectBuilder::new(options).label(label).outlined());
    }

    fn spawn_select_with(&mut self, theme: &MaterialTheme, builder: SelectBuilder) {
        let label_color = builder.select.label_color(theme);
        let text_color = builder.select.text_color(theme);
        let option_text_color = theme.on_surface;

        let dropdown_max_height = builder.dropdown_max_height;
        let dropdown_virtualize = builder.dropdown_virtualize;

        // Clone options for building the dropdown list
        let options = builder.select.options.clone();
        let selected_index = builder.select.selected_index;
        let placeholder = if builder.select.label.is_some() {
            builder.select.label.clone().unwrap_or_default()
        } else if builder.localization.label_key.is_some() {
            // Let `select_localization_system` resolve the placeholder without flashing a hard-coded string.
            String::new()
        } else {
            "Select".to_string()
        };

        let mut select_entity_commands = self.spawn(builder.build(theme));
        let select_entity = select_entity_commands.id();

        select_entity_commands.with_children(|select| {
            // Display text
            let display_label = selected_index
                .and_then(|idx| options.get(idx))
                .map(|o| o.label.as_str())
                .unwrap_or(placeholder.as_str());

            select.spawn((
                SelectDisplayText,
                Text::new(display_label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Dropdown arrow
            select.spawn((
                SelectDropdownArrow,
                MaterialIcon::from_name(ICON_EXPAND_MORE)
                    .expect("embedded icon 'expand_more' not found")
                    .with_size(20.0)
                    .with_color(label_color),
            ));

            // Dropdown list (hidden by default)
            select
                .spawn((
                    SelectDropdown,
                    Visibility::Hidden,
                    // Ensure the dropdown renders above later siblings (e.g. code blocks).
                    // NOTE: Dialog scrims in this project use `GlobalZIndex(1000)`.
                    // If the dropdown is promoted to a root node by `GlobalZIndex`, it must
                    // be above modal overlays, otherwise it will render "behind" dialogs.
                    GlobalZIndex(1100),
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(SELECT_HEIGHT + 4.0),
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        // The outer node draws the dropdown surface.
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                ))
                .with_children(|dropdown| {
                    // Content container. Option rows are spawned under this child.
                    // If `dropdown_max_height` is set, this becomes a scroll container.
                    let mut virtual_pool_size: Option<usize> = None;

                    let mut content = if let Some(max_height) = dropdown_max_height {
                        let viewport_height = match max_height {
                            Val::Px(max_px) => {
                                let items_px = (options.len() as f32) * SELECT_OPTION_HEIGHT;
                                let viewport_items_px = items_px.min(max_px);
                                Val::Px(SELECT_DROPDOWN_PADDING_TOTAL + viewport_items_px)
                            }
                            other => other,
                        };

                        // Virtualization is only supported for pixel max heights and only makes sense
                        // when the dropdown needs to scroll.
                        let wants_virtualize = dropdown_virtualize
                            && matches!(max_height, Val::Px(_))
                            && (options.len() as f32 * SELECT_OPTION_HEIGHT)
                                > match max_height {
                                    Val::Px(px) => px,
                                    _ => f32::INFINITY,
                                };

                        if wants_virtualize {
                            let pool_size = match viewport_height {
                                Val::Px(viewport_px) => {
                                    // Count visible rows based on the items area (excluding padding).
                                    let available_for_rows =
                                        (viewport_px - SELECT_DROPDOWN_PADDING_TOTAL).max(0.0);
                                    let visible =
                                        (available_for_rows / SELECT_OPTION_HEIGHT).ceil() as usize;
                                    (visible + 2).max(1)
                                }
                                _ => 12,
                            };

                            virtual_pool_size = Some(pool_size);

                            dropdown.spawn((
                                SelectDropdownContent,
                                SelectDropdownMaxHeight(max_height),
                                SelectDropdownVirtualized { pool_size },
                                SelectOwner(select_entity),
                                ScrollContainer::vertical(),
                                ScrollPosition::default(),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: viewport_height,
                                    max_height,
                                    overflow: Overflow::scroll_y(),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::vertical(Val::Px(SELECT_DROPDOWN_PADDING_Y)),
                                    ..default()
                                },
                            ))
                        } else {
                            dropdown.spawn((
                                SelectDropdownContent,
                                SelectDropdownMaxHeight(max_height),
                                ScrollContainer::vertical(),
                                ScrollPosition::default(),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: viewport_height,
                                    max_height,
                                    overflow: Overflow::scroll_y(),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::vertical(Val::Px(SELECT_DROPDOWN_PADDING_Y)),
                                    ..default()
                                },
                            ))
                        }
                    } else {
                        dropdown.spawn((
                            SelectDropdownContent,
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::vertical(Val::Px(8.0)),
                                ..default()
                            },
                        ))
                    };

                    // Virtualized dropdown: fixed pool of rows + spacers.
                    if let Some(pool_size) = virtual_pool_size {
                        let icon_default =
                            icon_by_name("check").expect("embedded icon 'check' not found");

                        content.with_children(|list| {
                            list.spawn((
                                SelectVirtualTopSpacer,
                                Node {
                                    height: Val::Px(0.0),
                                    min_height: Val::Px(0.0),
                                    flex_shrink: 0.0,
                                    ..default()
                                },
                            ));

                            for pool_index in 0..pool_size {
                                let (index, label) = options
                                    .get(pool_index)
                                    .map(|o| (pool_index, o.label.clone()))
                                    .unwrap_or((usize::MAX, String::new()));

                                let is_disabled = options.get(index).is_some_and(|o| o.disabled);
                                let is_selected = selected_index.is_some_and(|i| i == index);
                                let row_bg = if is_selected {
                                    theme.secondary_container
                                } else {
                                    Color::NONE
                                };

                                list.spawn((
                                    SelectVirtualRow,
                                    SelectOwner(select_entity),
                                    SelectOptionItem {
                                        index,
                                        label: label.clone(),
                                    },
                                    Button,
                                    Interaction::None,
                                    Node {
                                        height: Val::Px(SELECT_OPTION_HEIGHT),
                                        min_height: Val::Px(SELECT_OPTION_HEIGHT),
                                        flex_shrink: 0.0,
                                        padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(Spacing::MEDIUM),
                                        ..default()
                                    },
                                    BackgroundColor(row_bg),
                                ))
                                .with_children(|row| {
                                    // Virtual icon placeholder. We'll toggle display based on the option.
                                    row.spawn((
                                        SelectVirtualIcon,
                                        SelectOptionIcon,
                                        MaterialIcon::new(icon_default)
                                            .with_size(20.0)
                                            .with_color(option_text_color),
                                        Node {
                                            display: Display::None,
                                            ..default()
                                        },
                                    ));

                                    row.spawn((
                                        SelectOptionLabelText,
                                        Text::new(label),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(if is_disabled {
                                            option_text_color.with_alpha(0.38)
                                        } else {
                                            option_text_color
                                        }),
                                    ));
                                });
                            }

                            list.spawn((
                                SelectVirtualBottomSpacer,
                                Node {
                                    height: Val::Px(0.0),
                                    min_height: Val::Px(0.0),
                                    flex_shrink: 0.0,
                                    ..default()
                                },
                            ));
                        });
                    } else {
                        // Non-virtualized: spawn every option row.
                        content.with_children(|dropdown| {
                            for (index, option) in options.iter().enumerate() {
                                let is_disabled = option.disabled;
                                let is_selected = selected_index.is_some_and(|i| i == index);
                                let row_bg = if is_selected {
                                    theme.secondary_container
                                } else {
                                    Color::NONE
                                };

                                dropdown
                                    .spawn((
                                        SelectOwner(select_entity),
                                        SelectOptionItem {
                                            index,
                                            label: option.label.clone(),
                                        },
                                        Button,
                                        Interaction::None,
                                        Node {
                                            height: Val::Px(SELECT_OPTION_HEIGHT),
                                            min_height: Val::Px(SELECT_OPTION_HEIGHT),
                                            flex_shrink: 0.0,
                                            padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(Spacing::MEDIUM),
                                            ..default()
                                        },
                                        BackgroundColor(row_bg),
                                    ))
                                    .with_children(|row| {
                                        // Optional leading icon
                                        if let Some(icon) = &option.icon {
                                            if let Some(id) = icon_by_name(icon.as_str()) {
                                                row.spawn((
                                                    SelectOptionIcon,
                                                    MaterialIcon::new(id)
                                                        .with_size(20.0)
                                                        .with_color(if is_disabled {
                                                            option_text_color.with_alpha(0.38)
                                                        } else {
                                                            option_text_color
                                                        }),
                                                ));
                                            }
                                        }

                                        row.spawn((
                                            SelectOptionLabelText,
                                            Text::new(option.label.clone()),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(if is_disabled {
                                                option_text_color.with_alpha(0.38)
                                            } else {
                                                option_text_color
                                            }),
                                        ));
                                    });
                            }
                        });
                    }
                });
        });
    }
}

/// Update virtualized dropdown rows based on scroll position.
fn select_dropdown_virtualization_system(
    theme: Option<Res<MaterialTheme>>,
    selects: Query<&MaterialSelect>,
    children_query: Query<&Children>,
    is_scroll_content: Query<(), With<ScrollContent>>,
    mut contents: Query<
        (
            Entity,
            &SelectOwner,
            &SelectDropdownVirtualized,
            &ScrollPosition,
            &Children,
        ),
        With<SelectDropdownContent>,
    >,
    mut nodes: ParamSet<(
        Query<&mut Node, With<SelectVirtualTopSpacer>>,
        Query<&mut Node, With<SelectVirtualBottomSpacer>>,
        Query<
            (
                &mut Node,
                &mut SelectOptionItem,
                &mut BackgroundColor,
                &Children,
            ),
            With<SelectVirtualRow>,
        >,
        Query<(&mut MaterialIcon, &mut Node), With<SelectVirtualIcon>>,
    )>,
    mut label_texts: Query<(&mut Text, &mut TextColor), With<SelectOptionLabelText>>,
) {
    let Some(theme) = theme else { return };

    for (content_entity, owner, virt, scroll_pos, content_children) in contents.iter_mut() {
        let Ok(select) = selects.get(owner.0) else {
            continue;
        };

        if !select.open {
            continue;
        }

        // Find the actual list root. For ScrollContainers, children are moved under ScrollContent.
        let mut list_root = content_entity;
        if let Some(wrapper) = content_children
            .iter()
            .find(|c| is_scroll_content.get(*c).is_ok())
        {
            list_root = wrapper;
        }

        let Ok(list_children) = children_query.get(list_root) else {
            continue;
        };

        let options_len = select.options.len();
        let pool_size = virt.pool_size.max(1);

        // Account for the dropdown's internal vertical padding.
        let scroll_y = (scroll_pos.y - SELECT_DROPDOWN_PADDING_Y).max(0.0);
        let mut start_index = (scroll_y / SELECT_OPTION_HEIGHT).floor() as usize;
        if options_len > pool_size {
            start_index = start_index.min(options_len - pool_size);
        } else {
            start_index = 0;
        }

        let top_px = (start_index as f32) * SELECT_OPTION_HEIGHT;
        let bottom_px =
            (options_len.saturating_sub(start_index + pool_size) as f32) * SELECT_OPTION_HEIGHT;

        // Update spacers.
        {
            let mut top_spacers = nodes.p0();
            for child in list_children.iter() {
                if let Ok(mut node) = top_spacers.get_mut(child) {
                    node.height = Val::Px(top_px);
                    node.min_height = Val::Px(top_px);
                }
            }
        }
        {
            let mut bottom_spacers = nodes.p1();
            for child in list_children.iter() {
                if let Ok(mut node) = bottom_spacers.get_mut(child) {
                    node.height = Val::Px(bottom_px);
                    node.min_height = Val::Px(bottom_px);
                }
            }
        }

        // Update row pool.
        let base_text = theme.on_surface;

        let mut row_i = 0usize;
        for child in list_children.iter() {
            // First pass: update the row container + label text. Capture row children so we can
            // update icons in a second pass without conflicting ParamSet borrows.
            let (row_children_entities, icon_name, text_color) = {
                let mut row_query = nodes.p2();
                let Ok((mut row_node, mut item, mut row_bg, row_children)) =
                    row_query.get_mut(child)
                else {
                    continue;
                };

                let idx = start_index + row_i;
                row_i += 1;

                if idx >= options_len {
                    row_node.display = Display::None;
                    item.index = usize::MAX;
                    item.label.clear();
                    (Vec::new(), None, base_text)
                } else {
                    row_node.display = Display::Flex;

                    let opt = &select.options[idx];
                    item.index = idx;
                    if item.label != opt.label {
                        item.label = opt.label.clone();
                    }

                    let is_disabled = opt.disabled;
                    let is_selected = select.selected_index.is_some_and(|i| i == idx);
                    row_bg.0 = if is_selected {
                        theme.secondary_container
                    } else {
                        Color::NONE
                    };

                    let text_color = if is_disabled {
                        base_text.with_alpha(0.38)
                    } else {
                        base_text
                    };

                    for row_child in row_children.iter() {
                        if let Ok((mut text, mut color)) = label_texts.get_mut(row_child) {
                            *text = Text::new(opt.label.clone());
                            color.0 = text_color;
                        }
                    }

                    (
                        row_children.iter().collect::<Vec<_>>(),
                        opt.icon.as_deref().map(|s| s.to_string()),
                        text_color,
                    )
                }
            };

            // Second pass: update icons.
            if !row_children_entities.is_empty() {
                let mut icon_query = nodes.p3();
                for row_child in row_children_entities {
                    if let Ok((mut icon, mut icon_node)) = icon_query.get_mut(row_child) {
                        if let Some(name) = icon_name.as_deref() {
                            if let Some(id) = icon_by_name(name) {
                                icon.id = id;
                                icon.color = text_color;
                                icon_node.display = Display::Flex;
                            } else {
                                icon_node.display = Display::None;
                            }
                        } else {
                            icon_node.display = Display::None;
                        }
                    }
                }
            }

            if row_i >= pool_size {
                break;
            }
        }
    }
}
