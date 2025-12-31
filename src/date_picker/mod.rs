//! Material Design 3 Date Picker Component
//!
//! A standalone date picker matching Material Design 3 specifications.
//! Supports single date or date range selection with calendar and text input modes.

use bevy::prelude::*;

use crate::theme::MaterialTheme;
use crate::icons::material_icon_names;
use crate::tokens::{CornerRadius, Spacing};

mod calendar;
mod constraints;
mod range_selector;
mod types;

pub use constraints::*;
pub use range_selector::*;
pub use types::*;

/// Plugin for the Date Picker component.
pub struct DatePickerPlugin;

impl Plugin for DatePickerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<DatePickerSubmitEvent>()
            .add_message::<DatePickerCancelEvent>()
            .add_systems(
                Update,
                (
                    date_picker_visibility_system,
                    date_picker_keyboard_dismiss_system,
                    date_picker_mode_toggle_system,
                    date_picker_month_nav_system,
                    date_picker_year_selector_toggle_system,
                    date_picker_year_selection_system,
                    date_picker_day_selection_system,
                    date_picker_action_system,
                ),
            )
            .add_systems(
                Update,
                (
                    date_picker_rebuild_content_system,
                    date_picker_render_system,
                    date_picker_view_visibility_system,
                    date_picker_theme_system,
                ),
            );
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Date picker selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatePickerMode {
    /// Select a single date
    Single,
    /// Select a date range (start and end)
    Range,
}

/// Date picker input mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateInputMode {
    /// Visual calendar grid
    Calendar,
    /// Text input with format validation
    Text,
}

/// Material Design 3 Date Picker Component
#[derive(Component, Clone)]
pub struct MaterialDatePicker {
    /// Whether the picker is open
    pub open: bool,
    /// Picker title
    pub title: String,
    /// Selection mode (single or range)
    pub mode: DatePickerMode,
    /// Input mode (calendar or text)
    pub input_mode: DateInputMode,
    /// Current selection
    pub(crate) selector: Box<dyn DateSelector>,
    /// Calendar constraints
    pub(crate) constraints: CalendarConstraints,
    /// Currently displayed month
    pub(crate) display_month: Month,
    /// Whether year selector is shown
    pub(crate) showing_years: bool,
    /// First day of week
    pub first_day_of_week: Weekday,
    /// Dismiss on scrim click
    pub dismiss_on_scrim_click: bool,
    /// Dismiss on escape key
    pub dismiss_on_escape: bool,
}

impl MaterialDatePicker {
    /// Returns the current selection (single date or range).
    pub fn selection(&self) -> Option<DateSelection> {
        self.selector.selection()
    }
}

impl std::fmt::Debug for MaterialDatePicker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaterialDatePicker")
            .field("open", &self.open)
            .field("title", &self.title)
            .field("mode", &self.mode)
            .field("input_mode", &self.input_mode)
            .field("selector", &"<DateSelector>")
            .field("constraints", &self.constraints)
            .field("display_month", &self.display_month)
            .field("showing_years", &self.showing_years)
            .field("first_day_of_week", &self.first_day_of_week)
            .field("dismiss_on_scrim_click", &self.dismiss_on_scrim_click)
            .field("dismiss_on_escape", &self.dismiss_on_escape)
            .finish()
    }
}

/// Builder for Material Date Picker
#[derive(Debug, Clone)]
pub struct DatePickerBuilder {
    title: String,
    mode: DatePickerMode,
    input_mode: DateInputMode,
    initial_selection: Option<DateSelection>,
    constraints: CalendarConstraints,
    first_day_of_week: Weekday,
    dismiss_on_scrim_click: bool,
    dismiss_on_escape: bool,
    width: Val,
}

impl Default for DatePickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DatePickerBuilder {
    pub fn new() -> Self {
        Self {
            title: "Select date".to_string(),
            mode: DatePickerMode::Single,
            input_mode: DateInputMode::Calendar,
            initial_selection: None,
            constraints: CalendarConstraints::default(),
            first_day_of_week: Weekday::Sun,
            dismiss_on_scrim_click: true,
            dismiss_on_escape: true,
            width: Val::Px(360.0),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn mode(mut self, mode: DatePickerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn input_mode(mut self, mode: DateInputMode) -> Self {
        self.input_mode = mode;
        self
    }

    pub fn initial_selection(mut self, selection: DateSelection) -> Self {
        self.initial_selection = Some(selection);
        self
    }

    pub fn single_date(mut self, date: Date) -> Self {
        self.mode = DatePickerMode::Single;
        self.initial_selection = Some(DateSelection::Single(date));
        self
    }

    pub fn date_range(mut self, start: Date, end: Option<Date>) -> Self {
        self.mode = DatePickerMode::Range;
        self.initial_selection = Some(DateSelection::Range { start, end });
        self
    }

    pub fn constraints(mut self, constraints: CalendarConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn first_day_of_week(mut self, day: Weekday) -> Self {
        self.first_day_of_week = day;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn dismiss_on_scrim_click(mut self, enabled: bool) -> Self {
        self.dismiss_on_scrim_click = enabled;
        self
    }

    pub fn dismiss_on_escape(mut self, enabled: bool) -> Self {
        self.dismiss_on_escape = enabled;
        self
    }

    fn build_picker(&self) -> MaterialDatePicker {
        let selector: Box<dyn DateSelector> = match self.mode {
            DatePickerMode::Single => {
                let mut sel = SingleDateSelector::new();
                if let Some(DateSelection::Single(date)) = self.initial_selection {
                    sel.set_selection(DateSelection::Single(date));
                }
                Box::new(sel)
            }
            DatePickerMode::Range => {
                let mut sel = RangeDateSelector::new();
                if let Some(selection) = &self.initial_selection {
                    sel.set_selection(selection.clone());
                }
                Box::new(sel)
            }
        };

        let display_month = if let Some(selection) = &self.initial_selection {
            match selection {
                DateSelection::Single(date) => Month::new(date.year, date.month),
                DateSelection::Range { start, .. } => Month::new(start.year, start.month),
            }
        } else {
            self.constraints.opening
        };

        MaterialDatePicker {
            open: false,
            title: self.title.clone(),
            mode: self.mode,
            input_mode: self.input_mode,
            selector,
            constraints: self.constraints.clone(),
            display_month,
            showing_years: false,
            first_day_of_week: self.first_day_of_week,
            dismiss_on_scrim_click: self.dismiss_on_scrim_click,
            dismiss_on_escape: self.dismiss_on_escape,
        }
    }
}

// ============================================================================
// Events
// ============================================================================

#[derive(Event, Message)]
pub struct DatePickerSubmitEvent {
    pub entity: Entity,
    pub selection: DateSelection,
}

#[derive(Event, Message)]
pub struct DatePickerCancelEvent {
    pub entity: Entity,
}

// ============================================================================
// Internal Components
// ============================================================================

#[derive(Component)]
struct DatePickerScrim {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerDialog {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerModeToggle {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerModeToggleLabel {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerMonthNav {
    picker: Entity,
    delta: i32,
}

#[derive(Component)]
struct DatePickerYearToggle {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerYearToggleIcon {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerYearCell {
    picker: Entity,
    year: i32,
}

#[derive(Component)]
struct DatePickerDayCell {
    picker: Entity,
    date: Option<Date>,
}

#[derive(Component)]
struct DatePickerAction {
    picker: Entity,
    is_confirm: bool,
}

#[derive(Component)]
struct DatePickerLabel {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerMonthLabel {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerTextInputValue {
    picker: Entity,
}

#[derive(Component)]
struct DatePickerCalendarView {
    picker: Entity,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct DatePickerCalendarBuiltState {
    month: Month,
    first_day_of_week: Weekday,
}

#[derive(Component)]
struct DatePickerYearView {
    picker: Entity,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct DatePickerYearBuiltState {
    current_year: i32,
    start_year: i32,
    end_year: i32,
}

#[derive(Component)]
struct DatePickerTextView {
    picker: Entity,
}

// ============================================================================
// Systems
// ============================================================================

fn date_picker_visibility_system(
    pickers: Query<(Entity, &MaterialDatePicker), Changed<MaterialDatePicker>>,
    mut root_nodes: Query<&mut Node, Without<DatePickerDialog>>,
) {
    for (entity, picker) in pickers.iter() {
        if let Ok(mut node) = root_nodes.get_mut(entity) {
            node.display = if picker.open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn date_picker_keyboard_dismiss_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut pickers: Query<(Entity, &mut MaterialDatePicker)>,
    mut cancel_events: MessageWriter<DatePickerCancelEvent>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    for (entity, mut picker) in pickers.iter_mut() {
        if picker.open && picker.dismiss_on_escape {
            picker.open = false;
            cancel_events.write(DatePickerCancelEvent { entity });
        }
    }
}

fn date_picker_mode_toggle_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    toggles: Query<(&Interaction, &DatePickerModeToggle), Changed<Interaction>>,
) {
    for (interaction, toggle) in toggles.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(toggle.picker) {
            if !picker.open {
                continue;
            }

            picker.input_mode = match picker.input_mode {
                DateInputMode::Calendar => DateInputMode::Text,
                DateInputMode::Text => DateInputMode::Calendar,
            };

            // If we leave calendar mode, make sure the year grid doesn't stay open.
            if picker.input_mode != DateInputMode::Calendar {
                picker.showing_years = false;
            }
        }
    }
}

fn date_picker_month_nav_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    nav: Query<(&Interaction, &DatePickerMonthNav), Changed<Interaction>>,
) {
    for (interaction, nav) in nav.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(nav.picker) {
            if picker.open
                && picker.input_mode == DateInputMode::Calendar
                && !picker.showing_years
            {
                picker.display_month = picker.display_month.add_months(nav.delta);
                
                // Clamp to constraints
                if picker.display_month < picker.constraints.start {
                    picker.display_month = picker.constraints.start;
                }
                if picker.display_month > picker.constraints.end {
                    picker.display_month = picker.constraints.end;
                }
            }
        }
    }
}

fn date_picker_year_selector_toggle_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    toggles: Query<(&Interaction, &DatePickerYearToggle), Changed<Interaction>>,
) {
    for (interaction, toggle) in toggles.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(toggle.picker) {
            if picker.open && picker.input_mode == DateInputMode::Calendar {
                picker.showing_years = !picker.showing_years;
            }
        }
    }
}

fn date_picker_year_selection_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    years: Query<(&Interaction, &DatePickerYearCell), Changed<Interaction>>,
) {
    for (interaction, cell) in years.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(cell.picker) {
            if picker.open && picker.showing_years {
                picker.display_month = Month::new(cell.year, picker.display_month.month);
                picker.showing_years = false;
            }
        }
    }
}

fn date_picker_day_selection_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    days: Query<(&Interaction, &DatePickerDayCell), Changed<Interaction>>,
) {
    for (interaction, cell) in days.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Some(date) = cell.date {
            if let Ok(mut picker) = pickers.get_mut(cell.picker) {
                if picker.open && !picker.showing_years && picker.input_mode == DateInputMode::Calendar {
                    // Validate date
                    if !picker.constraints.validator.is_valid(date) {
                        continue;
                    }
                    
                    // Update selection based on mode
                    let current = picker.selector.selection();
                    match picker.mode {
                        DatePickerMode::Single => {
                            picker.selector.set_selection(DateSelection::Single(date));
                        }
                        DatePickerMode::Range => {
                            match current {
                                None | Some(DateSelection::Single(_)) => {
                                    // Start new range
                                    picker.selector.set_selection(DateSelection::Range {
                                        start: date,
                                        end: None,
                                    });
                                }
                                Some(DateSelection::Range { start, end: None }) => {
                                    // Complete range
                                    if date >= start {
                                        picker.selector.set_selection(DateSelection::Range {
                                            start,
                                            end: Some(date),
                                        });
                                    } else {
                                        // Swap if clicked before start
                                        picker.selector.set_selection(DateSelection::Range {
                                            start: date,
                                            end: Some(start),
                                        });
                                    }
                                }
                                Some(DateSelection::Range { .. }) => {
                                    // Start new range
                                    picker.selector.set_selection(DateSelection::Range {
                                        start: date,
                                        end: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn collect_descendants(root: Entity, children_query: &Query<&Children>, out: &mut Vec<Entity>) {
    if let Ok(children) = children_query.get(root) {
        for child in children.iter() {
            out.push(child);
            collect_descendants(child, children_query, out);
        }
    }
}

fn date_picker_rebuild_content_system(
    mut commands: Commands,
    pickers: Query<(Entity, &MaterialDatePicker), Changed<MaterialDatePicker>>,
    mut calendar_views: Query<(
        Entity,
        &DatePickerCalendarView,
        Option<&Children>,
        Option<&mut DatePickerCalendarBuiltState>,
    )>,
    mut year_views: Query<(
        Entity,
        &DatePickerYearView,
        Option<&Children>,
        Option<&mut DatePickerYearBuiltState>,
    )>,
    children_query: Query<&Children>,
    theme: Res<MaterialTheme>,
) {
    for (picker_entity, picker) in pickers.iter() {
        if !picker.open {
            continue;
        }

        // Rebuild the calendar grid when calendar-related state changes.
        // (This is necessary because the initial UI is generated at spawn-time.)
        for (view_entity, view, children, built_state) in calendar_views.iter_mut() {
            if view.picker != picker_entity {
                continue;
            }

            let desired_state = DatePickerCalendarBuiltState {
                month: picker.display_month,
                first_day_of_week: picker.first_day_of_week,
            };

            let needs_rebuild = match built_state.as_deref() {
                Some(state) => *state != desired_state,
                None => true,
            };

            if !needs_rebuild {
                continue;
            }

            match built_state {
                Some(mut state) => {
                    *state = desired_state;
                }
                None => {
                    commands.entity(view_entity).insert(desired_state);
                }
            }

            if let Some(children) = children {
                let mut to_despawn = Vec::new();
                for child in children.iter() {
                    to_despawn.push(child);
                    collect_descendants(child, &children_query, &mut to_despawn);
                }
                to_despawn.reverse();
                for entity in to_despawn {
                    commands.entity(entity).despawn();
                }
            }

            let display_month = picker.display_month;
            let first_day_of_week = picker.first_day_of_week;
            let on_surface = theme.on_surface;

            commands.entity(view_entity).with_children(|calendar| {
                // Days of week header (rotated based on first day of week)
                let all_days = ["S", "M", "T", "W", "T", "F", "S"];
                let first_day_of_week_index = crate::date_picker::types::weekday_index(first_day_of_week) as usize;
                let mut rotated = Vec::with_capacity(7);
                for i in 0..7 {
                    rotated.push(all_days[(first_day_of_week_index + i) % 7]);
                }

                calendar
                    .spawn(Node {
                        justify_content: JustifyContent::SpaceAround,
                        column_gap: Val::Px(Spacing::SMALL),
                        ..default()
                    })
                    .with_children(|header| {
                        for &day in rotated.iter() {
                            header.spawn((
                                Text::new(day),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(24.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ));
                        }
                    });

                // Calculate calendar grid layout
                let first_day = display_month.first_day();
                let first_weekday = crate::date_picker::types::weekday_for_date(first_day);
                let first_weekday_index = crate::date_picker::types::weekday_index(first_weekday);

                // Calculate offset: how many cells before day 1
                let offset = (first_weekday_index - first_day_of_week_index as i32 + 7) % 7;
                let days_in_month = crate::date_picker::types::days_in_month(display_month.year, display_month.month);

                // Calendar grid (6 weeks x 7 days = 42 cells)
                for week_idx in 0..6 {
                    calendar
                        .spawn(Node {
                            justify_content: JustifyContent::SpaceAround,
                            column_gap: Val::Px(Spacing::SMALL),
                            ..default()
                        })
                        .with_children(|week| {
                            for day_idx in 0..7 {
                                let cell_idx = week_idx * 7 + day_idx;
                                let day_offset = cell_idx as i32 - offset as i32;

                                // Calculate if this cell contains a valid day
                                if day_offset >= 0 && day_offset < days_in_month as i32 {
                                    let day_number = (day_offset + 1) as u8;
                                    let date = Date::new(display_month.year, display_month.month, day_number);
                                    let is_today = date == Date::today();
                                    let is_valid = picker.constraints.validator.is_valid(date);

                                    let base_text_color = if is_valid {
                                        on_surface
                                    } else {
                                        on_surface.with_alpha(0.38)
                                    };

                                    // Valid dates become buttons; invalid dates are static.
                                    if is_valid {
                                        let (bg_color, text_color) = if is_today {
                                            (theme.primary_container, theme.on_primary_container)
                                        } else {
                                            (Color::NONE, base_text_color)
                                        };

                                        week
                                            .spawn((
                                                Button,
                                                DatePickerDayCell {
                                                    picker: picker_entity,
                                                    date: Some(date),
                                                },
                                                Interaction::None,
                                                Node {
                                                    width: Val::Px(40.0),
                                                    height: Val::Px(40.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                BackgroundColor(bg_color),
                                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                            ))
                                            .with_children(|cell| {
                                                cell.spawn((
                                                    Text::new(day_number.to_string()),
                                                    TextFont {
                                                        font_size: 14.0,
                                                        ..default()
                                                    },
                                                    TextColor(text_color),
                                                ));
                                            });
                                    } else {
                                        week
                                            .spawn((
                                                Node {
                                                    width: Val::Px(40.0),
                                                    height: Val::Px(40.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                BackgroundColor(Color::NONE),
                                            ))
                                            .with_children(|cell| {
                                                cell.spawn((
                                                    Text::new(day_number.to_string()),
                                                    TextFont {
                                                        font_size: 14.0,
                                                        ..default()
                                                    },
                                                    TextColor(base_text_color),
                                                ));
                                            });
                                    }
                                } else {
                                    // Empty cell for days outside current month
                                    week.spawn(Node {
                                        width: Val::Px(40.0),
                                        height: Val::Px(40.0),
                                        ..default()
                                    });
                                }
                            }
                        });
                }
            });
        }

        // Rebuild the year grid so the highlighted year stays in sync with display_month.
        for (view_entity, view, children, built_state) in year_views.iter_mut() {
            if view.picker != picker_entity {
                continue;
            }

            let desired_state = DatePickerYearBuiltState {
                current_year: picker.display_month.year,
                start_year: picker.constraints.start.year,
                end_year: picker.constraints.end.year,
            };

            let needs_rebuild = match built_state.as_deref() {
                Some(state) => *state != desired_state,
                None => true,
            };

            if !needs_rebuild {
                continue;
            }

            match built_state {
                Some(mut state) => {
                    *state = desired_state;
                }
                None => {
                    commands.entity(view_entity).insert(desired_state);
                }
            }

            if let Some(children) = children {
                let mut to_despawn = Vec::new();
                for child in children.iter() {
                    to_despawn.push(child);
                    collect_descendants(child, &children_query, &mut to_despawn);
                }
                to_despawn.reverse();
                for entity in to_despawn {
                    commands.entity(entity).despawn();
                }
            }

            let current_year = picker.display_month.year;
            let start_year = picker.constraints.start.year;
            let end_year = picker.constraints.end.year;
            let on_surface = theme.on_surface;

            commands.entity(view_entity).with_children(|year_grid| {
                let mut years = Vec::new();
                for year in start_year..=end_year {
                    years.push(year);
                }

                for year_row in years.chunks(3) {
                    year_grid
                        .spawn(Node {
                            justify_content: JustifyContent::SpaceAround,
                            column_gap: Val::Px(Spacing::MEDIUM),
                            ..default()
                        })
                        .with_children(|row| {
                            for &year in year_row {
                                let is_current = year == current_year;
                                let (bg_color, text_color) = if is_current {
                                    (theme.primary, theme.on_primary)
                                } else {
                                    (Color::NONE, on_surface)
                                };

                                row
                                    .spawn((
                                        Button,
                                        DatePickerYearCell {
                                            picker: picker_entity,
                                            year,
                                        },
                                        Interaction::None,
                                        Node {
                                            width: Val::Px(90.0),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new(year.to_string()),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(text_color),
                                        ));
                                    });
                            }
                        });
                }
            });
        }
    }
}

fn date_picker_action_system(
    mut pickers: Query<&mut MaterialDatePicker>,
    actions: Query<(&Interaction, &DatePickerAction), Changed<Interaction>>,
    scrim: Query<(&Interaction, &DatePickerScrim), Changed<Interaction>>,
    mut submit_events: MessageWriter<DatePickerSubmitEvent>,
    mut cancel_events: MessageWriter<DatePickerCancelEvent>,
) {
    // Handle scrim clicks
    for (interaction, scrim) in scrim.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(scrim.picker) {
            if picker.open && picker.dismiss_on_scrim_click {
                picker.open = false;
                cancel_events.write(DatePickerCancelEvent {
                    entity: scrim.picker,
                });
            }
        }
    }
    
    // Handle action buttons
    for (interaction, action) in actions.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        
        if let Ok(mut picker) = pickers.get_mut(action.picker) {
            if !picker.open {
                continue;
            }
            
            if action.is_confirm {
                if let Some(selection) = picker.selector.selection() {
                    picker.open = false;
                    submit_events.write(DatePickerSubmitEvent {
                        entity: action.picker,
                        selection,
                    });
                }
            } else {
                picker.open = false;
                cancel_events.write(DatePickerCancelEvent {
                    entity: action.picker,
                });
            }
        }
    }
}

fn date_picker_render_system(
    pickers: Query<(Entity, &MaterialDatePicker), Changed<MaterialDatePicker>>,
    mut day_cells: Query<(&DatePickerDayCell, &mut BackgroundColor, &Children)>,
    mut texts: Query<&mut TextColor>,
    mut text_nodes: Query<(
        &mut Text,
        Option<&DatePickerLabel>,
        Option<&DatePickerMonthLabel>,
        Option<&DatePickerTextInputValue>,
    )>,
    mut toggle_icons: Query<(
        &mut crate::icons::svg::SvgIcon,
        Option<&DatePickerModeToggleLabel>,
        Option<&DatePickerYearToggleIcon>,
    )>,
    theme: Res<MaterialTheme>,
) {
    // Update calendar UI based on picker state changes
    for (picker_entity, picker) in pickers.iter() {
        if !picker.open {
            continue;
        }
        
        let selection = picker.selector.selection();

        // Update selection header label
        let selection_text = match selection.as_ref() {
            Some(DateSelection::Single(date)) => format!("{}", date),
            Some(DateSelection::Range { start, end: Some(end) }) => format!("{} - {}", start, end),
            Some(DateSelection::Range { start, end: None }) => format!("{} - ...", start),
            None => "Select date".to_string(),
        };
        // Update all text nodes tied to this picker
        let month_text = picker.display_month.display_name();
        let text_value = match selection.as_ref() {
            Some(DateSelection::Single(date)) => {
                format!("{:02}/{:02}/{:04}", date.month as u8, date.day, date.year)
            }
            _ => String::new(),
        };

        for (mut text, selection_label, month_label, text_input_value) in text_nodes.iter_mut() {
            if let Some(label) = selection_label {
                if label.picker == picker_entity {
                    text.0 = selection_text.clone();
                }
            }
            if let Some(label) = month_label {
                if label.picker == picker_entity {
                    text.0 = month_text.clone();
                }
            }
            if let Some(value) = text_input_value {
                if value.picker == picker_entity {
                    text.0 = text_value.clone();
                }
            }
        }

        // Update toggle icons (input mode + year dropdown) without conflicting queries.
        // When in calendar mode, show the EDIT icon; when in text mode, show the CALENDAR icon.
        let mode_toggle_icon_name = if picker.input_mode == DateInputMode::Calendar {
            material_icon_names::material_ic_edit_black_24dp
        } else {
            material_icon_names::material_ic_calendar_black_24dp
        };
        let desired_mode_icon = mode_toggle_icon_name;

        // Month dropdown chevron icon (down when years hidden; up when showing years).
        let year_toggle_icon_name = if picker.showing_years {
            material_icon_names::material_ic_menu_arrow_up_black_24dp
        } else {
            material_icon_names::material_ic_menu_arrow_down_black_24dp
        };
        let desired_year_icon = year_toggle_icon_name;

        for (mut icon, mode_marker, year_marker) in toggle_icons.iter_mut() {
            if let Some(marker) = mode_marker {
                if marker.picker == picker_entity {
                    icon.name = desired_mode_icon.to_string();
                    icon.color = theme.on_surface;
                }
            }
            if let Some(marker) = year_marker {
                if marker.picker == picker_entity {
                    icon.name = desired_year_icon.to_string();
                    icon.color = theme.on_surface;
                }
            }
        }
        
        // Update day cell highlighting based on selection
        for (cell, mut bg, children) in day_cells.iter_mut() {
            if cell.picker != picker_entity {
                continue;
            }
            
            if let Some(date) = cell.date {
                let is_today = date == Date::today();
                let is_valid = picker.constraints.validator.is_valid(date);
                let is_selected = match &selection {
                    Some(DateSelection::Single(selected)) => date == *selected,
                    Some(DateSelection::Range { start, end }) => {
                        date == *start || end.map_or(false, |e| date == e)
                    }
                    None => false,
                };
                
                let in_range = match &selection {
                    Some(DateSelection::Range { start, end: Some(end) }) => {
                        date > *start && date < *end
                    }
                    _ => false,
                };
                
                // Apply colors based on state
                let (bg_color, text_color) = if !is_valid {
                    (Color::NONE, theme.on_surface.with_alpha(0.38))
                } else if is_selected {
                    (theme.primary, theme.on_primary)
                } else if in_range {
                    (theme.primary_container.with_alpha(0.3), theme.on_surface)
                } else if is_today {
                    (theme.primary_container, theme.on_primary_container)
                } else {
                    (Color::NONE, theme.on_surface)
                };
                
                *bg = BackgroundColor(bg_color);
                
                // Update text color
                for child in children.iter() {
                    if let Ok(mut text_color_comp) = texts.get_mut(child) {
                        *text_color_comp = TextColor(text_color);
                    }
                }
            }
        }
    }
}

fn date_picker_view_visibility_system(
    pickers: Query<(Entity, &MaterialDatePicker), Changed<MaterialDatePicker>>,
    mut views: ParamSet<(
        Query<(&DatePickerCalendarView, &mut Node)>,
        Query<(&DatePickerYearView, &mut Node)>,
        Query<(&DatePickerTextView, &mut Node)>,
    )>,
) {
    for (picker_entity, picker) in pickers.iter() {
        if !picker.open {
            continue;
        }

        {
            let mut calendar_views = views.p0();
            for (view, mut node) in calendar_views.iter_mut() {
                if view.picker != picker_entity {
                    continue;
                }
                node.display = if picker.input_mode == DateInputMode::Calendar && !picker.showing_years {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }

        {
            let mut year_views = views.p1();
            for (view, mut node) in year_views.iter_mut() {
                if view.picker != picker_entity {
                    continue;
                }
                node.display = if picker.showing_years {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }

        {
            let mut text_views = views.p2();
            for (view, mut node) in text_views.iter_mut() {
                if view.picker != picker_entity {
                    continue;
                }
                node.display = if picker.input_mode == DateInputMode::Text {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

fn date_picker_theme_system(
    theme: Res<MaterialTheme>,
    mut backgrounds: Query<(
        &mut BackgroundColor,
        Option<&DatePickerScrim>,
        Option<&DatePickerDialog>,
    )>,
) {
    if !theme.is_changed() {
        return;
    }
    
    for (mut bg, scrim, dialog) in backgrounds.iter_mut() {
        if scrim.is_some() {
            *bg = BackgroundColor(theme.scrim.with_alpha(0.32));
        } else if dialog.is_some() {
            *bg = BackgroundColor(theme.surface_container_high);
        }
    }
}

// ============================================================================
// Spawn Trait
// ============================================================================

pub trait SpawnDatePicker {
    fn spawn_date_picker(&mut self, theme: &MaterialTheme, builder: DatePickerBuilder) -> Entity;
}

impl SpawnDatePicker for ChildSpawnerCommands<'_> {
    fn spawn_date_picker(&mut self, theme: &MaterialTheme, builder: DatePickerBuilder) -> Entity {
        let picker = builder.build_picker();
        let width = builder.width;
        let bg_color = theme.surface_container_high;
        let on_surface = theme.on_surface;
        
        // Capture picker values before moving it
        let display_month = picker.display_month;
        let first_day_of_week = picker.first_day_of_week;
        let showing_years = picker.showing_years;
        let input_mode = picker.input_mode;
        
        // Create simplified placeholder UI - full implementation in future update
        let mut root = self.spawn((
            picker,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            GlobalZIndex(9999),
        ));
        let entity = root.id();
        
        root.with_children(|root| {
            // Scrim overlay
            root.spawn((
                DatePickerScrim {
                    picker: entity,
                },
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.scrim.with_alpha(0.32)),
                ZIndex(0),
            ));
            
            // Dialog container with calendar
            root.spawn((
                DatePickerDialog {
                    picker: entity,
                },
                Node {
                    width,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(Spacing::LARGE)),
                    row_gap: Val::Px(Spacing::MEDIUM),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderRadius::all(Val::Px(CornerRadius::EXTRA_LARGE)),
                BoxShadow::default(),
                ZIndex(1),
            )).with_children(|dialog| {
                // Selection header
                let selection_text = match builder.initial_selection.as_ref() {
                    Some(DateSelection::Single(date)) => format!("{}", date),
                    Some(DateSelection::Range { start, end: Some(end) }) => {
                        format!("{} - {}", start, end)
                    }
                    Some(DateSelection::Range { start, end: None }) => {
                        format!("{} - ...", start)
                    }
                    None => "Select date".to_string(),
                };
                
                dialog.spawn((
                    DatePickerLabel {
                        picker: entity,
                    },
                    Text::new(selection_text),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.primary),
                    Node {
                        margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                        ..default()
                    },
                ));
                
                // Title row with mode toggle
                dialog.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                }).with_children(|title_row| {
                    title_row.spawn((
                        Text::new("Select Date"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(on_surface),
                    ));
                    
                    // Input mode toggle button (calendar <-> text)
                    title_row.spawn((
                        Button,
                        DatePickerModeToggle {
                            picker: entity,
                        },
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    )).with_children(|btn| {
                        let mode_toggle_icon_name = if input_mode == DateInputMode::Calendar {
                            material_icon_names::material_ic_edit_black_24dp
                        } else {
                            material_icon_names::material_ic_calendar_black_24dp
                        };
                        btn.spawn((
                            DatePickerModeToggleLabel {
                                picker: entity,
                            },
                            crate::icons::svg::SvgIcon::new(mode_toggle_icon_name)
                                .with_size(20.0)
                                .with_color(on_surface),
                        ));
                    });
                });
                
                // Month navigation header
                dialog.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                }).with_children(|nav_row| {
                    // Previous month button
                    nav_row.spawn((
                        Button,
                        DatePickerMonthNav {
                            picker: entity,
                            delta: -1,
                        },
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    )).with_children(|btn| {
                        btn.spawn((
                            crate::icons::svg::SvgIcon::new(
                                material_icon_names::material_ic_keyboard_arrow_previous_black_24dp,
                            )
                            .with_size(20.0)
                            .with_color(on_surface),
                        ));
                    });
                    
                    // Month/year display (clickable to toggle year selector)
                    nav_row.spawn((
                        Button,
                        DatePickerYearToggle {
                            picker: entity,
                        },
                        Interaction::None,
                        Node {
                            padding: UiRect::all(Val::Px(Spacing::SMALL)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
                    ))
                    .with_children(|btn| {
                        btn.spawn(Node {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                DatePickerMonthLabel { picker: entity },
                                Text::new(display_month.display_name()),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(on_surface),
                            ));

                            let year_toggle_icon_name = if showing_years {
                                material_icon_names::material_ic_menu_arrow_up_black_24dp
                            } else {
                                material_icon_names::material_ic_menu_arrow_down_black_24dp
                            };
                            row.spawn((
                                DatePickerYearToggleIcon { picker: entity },
                                crate::icons::svg::SvgIcon::new(year_toggle_icon_name)
                                    .with_size(18.0)
                                    .with_color(on_surface),
                            ));
                        });
                    });
                    
                    // Next month button
                    nav_row.spawn((
                        Button,
                        DatePickerMonthNav {
                            picker: entity,
                            delta: 1,
                        },
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    )).with_children(|btn| {
                        btn.spawn((
                            crate::icons::svg::SvgIcon::new(
                                material_icon_names::material_ic_keyboard_arrow_next_black_24dp,
                            )
                            .with_size(20.0)
                            .with_color(on_surface),
                        ));
                    });
                });
                
                // Always spawn all three views with appropriate initial display state
                
                // Text input view
                let text_display = if input_mode == DateInputMode::Text {
                    Display::Flex
                } else {
                    Display::None
                };
                
                dialog.spawn((
                        DatePickerTextView {
                            picker: entity,
                        },
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(Spacing::MEDIUM),
                            padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                            display: text_display,
                            ..default()
                        },
                    )).with_children(|text_area| {
                        text_area.spawn((
                            Text::new("MM/DD/YYYY"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                            Node {
                                margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                                ..default()
                            },
                        ));
                        
                        // Text input field (placeholder - full text input requires text editing support)
                        text_area.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(56.0),
                                padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container_highest),
                            BorderColor::all(theme.outline),
                            BorderRadius::all(Val::Px(CornerRadius::SMALL)),
                        )).with_children(|field| {
                            let date_text = match builder.initial_selection.as_ref() {
                                Some(DateSelection::Single(date)) => {
                                    format!("{:02}/{:02}/{:04}", date.month as u8, date.day, date.year)
                                }
                                _ => String::new(),
                            };
                            
                            field.spawn((
                                DatePickerTextInputValue {
                                    picker: entity,
                                },
                                Text::new(date_text),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(on_surface),
                            ));
                        });
                        
                        // Help text
                        text_area.spawn((
                            Text::new("Enter date in MM/DD/YYYY format"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));
                    });
                
                // Year selector view
                let year_display = if showing_years { Display::Flex } else { Display::None };
                
                dialog.spawn((
                        DatePickerYearView {
                            picker: entity,
                        },
                        DatePickerYearBuiltState {
                            current_year: display_month.year,
                            start_year: builder.constraints.start.year,
                            end_year: builder.constraints.end.year,
                        },
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(Spacing::SMALL),
                            max_height: Val::Px(300.0),
                            overflow: Overflow::scroll_y(),
                            display: year_display,
                            ..default()
                        },
                    )).with_children(|year_grid| {
                        let current_year = display_month.year;
                        let start_year = builder.constraints.start.year;
                        let end_year = builder.constraints.end.year;
                        
                        // Create year buttons in rows of 3
                        let mut years = Vec::new();
                        for year in start_year..=end_year {
                            years.push(year);
                        }
                        
                        for year_row in years.chunks(3) {
                            year_grid.spawn(Node {
                                justify_content: JustifyContent::SpaceAround,
                                column_gap: Val::Px(Spacing::MEDIUM),
                                ..default()
                            }).with_children(|row| {
                                for &year in year_row {
                                    let is_current = year == current_year;
                                    let (bg_color, text_color) = if is_current {
                                        (theme.primary, theme.on_primary)
                                    } else {
                                        (Color::NONE, on_surface)
                                    };
                                    
                                    row.spawn((
                                        Button,
                                        DatePickerYearCell {
                                            picker: entity,
                                            year,
                                        },
                                        Interaction::None,
                                        Node {
                                            width: Val::Px(90.0),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                    )).with_children(|btn| {
                                        btn.spawn((
                                            Text::new(year.to_string()),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(text_color),
                                        ));
                                    });
                                }
                            });
                        }
                    });
                
                // Calendar grid view
                let calendar_display = if input_mode == DateInputMode::Calendar && !showing_years {
                    Display::Flex
                } else {
                    Display::None
                };
                
                dialog.spawn((
                        DatePickerCalendarView {
                            picker: entity,
                        },
                        DatePickerCalendarBuiltState {
                            month: display_month,
                            first_day_of_week,
                        },
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(Spacing::SMALL),
                            display: calendar_display,
                            ..default()
                        },
                    )).with_children(|calendar| {
                    // Days of week header
                    calendar.spawn(Node {
                        justify_content: JustifyContent::SpaceAround,
                        column_gap: Val::Px(Spacing::SMALL),
                        ..default()
                    }).with_children(|header| {
                        for day in ["S", "M", "T", "W", "T", "F", "S"] {
                            header.spawn((
                                Text::new(day),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(24.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ));
                        }
                    });
                    
                    // Calculate calendar grid layout
                    let first_day = display_month.first_day();
                    let first_weekday = crate::date_picker::types::weekday_for_date(first_day);
                    let first_weekday_index = crate::date_picker::types::weekday_index(first_weekday);
                    let first_day_of_week_index = crate::date_picker::types::weekday_index(first_day_of_week);
                    
                    // Calculate offset: how many cells before day 1
                    let offset = (first_weekday_index - first_day_of_week_index + 7) % 7;
                    let days_in_month = crate::date_picker::types::days_in_month(
                        display_month.year,
                        display_month.month
                    );
                    
                    // Calendar grid (6 weeks x 7 days = 42 cells)
                    for week_idx in 0..6 {
                        calendar.spawn(Node {
                            justify_content: JustifyContent::SpaceAround,
                            column_gap: Val::Px(Spacing::SMALL),
                            ..default()
                        }).with_children(|week| {
                            for day_idx in 0..7 {
                                let position = week_idx * 7 + day_idx;
                                let day_offset = position - offset;
                                
                                // Calculate if this cell contains a valid day
                                if day_offset >= 0 && day_offset < days_in_month as i32 {
                                    let day_number = (day_offset + 1) as u8;
                                    let date = Date::new(display_month.year, display_month.month, day_number);
                                    let is_today = date == Date::today();
                                    let is_valid = builder.constraints.validator.is_valid(date);
                                    
                                    // Determine cell colors based on state
                                    let (bg_color, text_color, enabled) = if !is_valid {
                                        // Disabled date - outside constraints
                                        (Color::NONE, theme.on_surface.with_alpha(0.38), false)
                                    } else if is_today {
                                        (theme.primary_container, theme.on_primary_container, true)
                                    } else {
                                        (Color::NONE, on_surface, true)
                                    };
                                    
                                    let mut cell_spawn = week.spawn((
                                        Button,
                                        DatePickerDayCell {
                                            picker: entity,
                                            date: Some(date),
                                        },
                                        Interaction::None,
                                        Node {
                                            width: Val::Px(40.0),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                    ));
                                    
                                    // Only enable interaction if date is valid
                                    if !enabled {
                                        cell_spawn.insert(Interaction::None);
                                    }
                                    
                                    cell_spawn.with_children(|cell| {
                                        cell.spawn((
                                            Text::new(day_number.to_string()),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(text_color),
                                        ));
                                    });
                                } else {
                                    // Empty cell for days outside current month
                                    week.spawn(Node {
                                        width: Val::Px(40.0),
                                        height: Val::Px(40.0),
                                        ..default()
                                    });
                                }
                            }
                        });
                    }
                });
                
                // Action buttons
                dialog.spawn(Node {
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(Spacing::SMALL),
                    margin: UiRect::top(Val::Px(Spacing::MEDIUM)),
                    ..default()
                }).with_children(|actions| {
                    // Cancel button
                    actions.spawn((
                        DatePickerAction {
                            picker: entity,
                            is_confirm: false,
                        },
                        Interaction::None,
                        Text::new("Cancel"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.primary),
                        Node {
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            ..default()
                        },
                    ));
                    
                    // OK button
                    actions.spawn((
                        DatePickerAction {
                            picker: entity,
                            is_confirm: true,
                        },
                        Interaction::None,
                        Text::new("OK"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.primary),
                        Node {
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                            ..default()
                        },
                    ));
                });  
            });
        });
        
        entity
    }
}
