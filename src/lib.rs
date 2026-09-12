//! # Bevy Material UI
//!
//! Material Design 3 UI components for the Bevy game engine.
//!
//! This library provides a comprehensive set of UI components following
//! [Material Design 3](https://m3.material.io/) guidelines, implemented
//! as Bevy ECS components and systems.
//!
//! ## Features
//!
//! - **Theme System**: Complete MD3 color scheme with light/dark mode support
//! - **Components**: Button, Card, Checkbox, Dialog, Divider, FAB, List, Menu,
//!   Progress, Radio, Ripple, Select, Slider, Switch, Tabs, TextField
//! - **Accessibility**: Built-in support for focus rings
//! - **Customization**: Token-based styling system for easy theming
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_material_ui::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(MaterialUiPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
//!     commands.spawn(Camera2d);
//!     
//!     // Create a filled button
//!     commands.spawn(
//!         MaterialButtonBuilder::new("Click Me")
//!             .filled()
//!             .build(&theme)
//!     );
//! }
//! ```
//!
//! ## Architecture
//!
//! This library follows patterns from the official Material Design implementations:
//! - [material-web](https://github.com/material-components/material-web)
//! - [material-components-flutter](https://github.com/material-components/material-components-flutter)

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

// ============================================================================
// Core modules
// ============================================================================

/// Theme and color system based on Material Design 3
pub mod theme;

/// Locale configuration and locale-driven defaults
pub mod locale;

/// Runtime localization / translation (i18n)
pub mod i18n;

/// HCT color space and dynamic color generation
pub mod color;

/// Embedded icon system
pub mod icons;

/// Typography scale definitions
pub mod typography;

/// Spacing, corner radius, duration, and easing tokens
pub mod tokens;

/// Elevation and shadow utilities
pub mod elevation;

/// Focus ring component for accessibility
pub mod focus;

/// Ripple effect component for touch feedback
pub mod ripple;

/// Telemetry and test automation support
pub mod telemetry;

// ============================================================================
// Component modules
// ============================================================================

/// Button components (filled, outlined, text, elevated, tonal)
pub mod button;

/// Button groups / segmented buttons (toggle groups)
pub mod button_group;

/// Icon button component
pub mod icon_button;

/// Floating Action Button (FAB) component
pub mod fab;

/// Card components (elevated, filled, outlined)
pub mod card;

/// Checkbox component
pub mod checkbox;

/// Radio button component
pub mod radio;

/// Switch/toggle component
pub mod switch;

/// Slider component
pub mod slider;

/// Text field components (filled, outlined)
pub mod text_field;

/// Progress indicators (linear and circular)
pub mod progress;

/// Dialog component
pub mod dialog;

/// Date picker component (Material Design 3)
pub mod date_picker;

/// Time picker component (Material Design 3)
pub mod time_picker;

/// List and list item components
pub mod list;

/// Menu and menu item components
pub mod menu;

/// Tabs component
pub mod tabs;

/// Divider component
pub mod divider;

/// Select/dropdown component
pub mod select;

/// Adaptive layout utilities (window size classes)
pub mod adaptive;

/// Material layout components (e.g. Scaffold)
pub mod layout;

/// Motion and animation utilities
pub mod motion;

/// Snackbar component for brief messages
pub mod snackbar;

/// Chip components for filters, actions, and tags
pub mod chip;

/// App bar components (top and bottom)
pub mod app_bar;

/// Badge component for notifications
pub mod badge;

/// Tooltip component for contextual help
pub mod tooltip;

/// Scroll container for scrollable content
pub mod scroll;

/// Search bar component
pub mod search;

/// Toolbar component
pub mod toolbar;

/// Loading indicator component
pub mod loading_indicator;

// ============================================================================
// Prelude
// ============================================================================

/// Prelude module for convenient imports
pub mod prelude {
    // Re-export Bevy UI types for convenience
    pub use bevy::ui::{
        BackgroundGradient, BorderGradient, ColorStop, ConicGradient, Gradient, LinearGradient,
        RadialGradient,
    };
    pub use bevy::ui::{BoxShadow, Outline, ShadowStyle};

    // Core
    pub use crate::elevation::{Elevation, ElevationShadow};
    pub use crate::focus::{
        FocusGained, FocusLost, FocusPlugin, FocusRing, Focusable, create_native_focus_outline,
    };
    pub use crate::i18n::{
        LocalizedText, MaterialI18n, MaterialI18nPlugin, MaterialLanguage,
        MaterialLanguageOverride, MaterialTranslations,
    };
    pub use crate::ripple::{Ripple, RippleHost, RipplePlugin, SpawnRipple};
    pub use crate::telemetry::{
        ElementBounds, InsertTestId, TelemetryConfig, TelemetryPlugin, TestId, test_id_if_enabled,
    };
    pub use crate::theme::{ColorScheme, MaterialTheme};
    pub use crate::tokens::{Duration, Easing, Spacing, corner_radius};
    pub use crate::typography::Typography;

    // Color System
    pub use crate::color::{Hct, MaterialColorScheme, TonalPalette};

    // Icons
    pub use crate::icons::{MaterialIcon, MaterialIconsPlugin, material_icons};

    // Button
    pub use crate::button::{
        ButtonLabel, ButtonPlugin, ButtonVariant, MaterialButton, MaterialButtonBuilder,
        SpawnButtonChild, material_button_bundle, spawn_material_button,
    };

    // Button Group
    pub use crate::button_group::{
        ButtonGroupBuilder, ButtonGroupOrientation, ButtonGroupPlugin, MaterialButtonGroup,
    };

    // Icon Button
    pub use crate::icon_button::{
        ICON_BUTTON_SIZE, ICON_SIZE, IconButtonBuilder, IconButtonClickEvent, IconButtonPlugin,
        IconButtonVariant, MaterialIconButton, SpawnIconButtonChild,
    };

    // FAB
    pub use crate::fab::{
        FabBuilder, FabClickEvent, FabColor, FabLabel, FabPlugin, FabSize, MaterialFab,
        SpawnFabChild,
    };

    // Card
    pub use crate::card::{
        CardBuilder, CardClickEvent, CardPlugin, CardVariant, MaterialCard, SpawnCardChild,
    };

    // Checkbox
    pub use crate::checkbox::{
        CHECKBOX_SIZE, CHECKBOX_TOUCH_TARGET, CheckboxBox, CheckboxBuilder, CheckboxChangeEvent,
        CheckboxIcon, CheckboxPlugin, CheckboxState, MaterialCheckbox, SpawnCheckbox,
        SpawnCheckboxChild,
    };

    // Radio
    pub use crate::radio::{
        MaterialRadio, RADIO_DOT_SIZE, RADIO_SIZE, RADIO_TOUCH_TARGET, RadioBuilder,
        RadioChangeEvent, RadioGroup, RadioInner, RadioOuter, RadioPlugin, RadioStateLayer,
        SpawnRadio, SpawnRadioChild,
    };

    // Switch
    pub use crate::switch::{
        MaterialSwitch, SWITCH_HANDLE_SIZE_PRESSED, SWITCH_HANDLE_SIZE_SELECTED,
        SWITCH_HANDLE_SIZE_UNSELECTED, SWITCH_TRACK_HEIGHT, SWITCH_TRACK_WIDTH, SpawnSwitch,
        SpawnSwitchChild, SwitchBuilder, SwitchChangeEvent, SwitchHandle, SwitchPlugin,
        SwitchStateLayer,
    };

    // Slider
    pub use crate::slider::{
        MaterialSlider, SLIDER_HANDLE_SIZE, SLIDER_HANDLE_SIZE_PRESSED, SLIDER_LABEL_HEIGHT,
        SLIDER_TICK_SIZE, SLIDER_TRACK_HEIGHT, SLIDER_TRACK_HEIGHT_ACTIVE, SliderActiveTrack,
        SliderBuilder, SliderChangeEvent, SliderHandle, SliderLabel, SliderPlugin,
        SliderTraceSettings, SliderTrack, SpawnSliderChild,
    };

    // Text Field
    pub use crate::text_field::{
        MaterialTextField, SpawnTextFieldChild, TEXT_FIELD_HEIGHT, TEXT_FIELD_MIN_WIDTH,
        TextFieldBuilder, TextFieldChangeEvent, TextFieldFormatter, TextFieldInput, TextFieldLabel,
        TextFieldPlugin, TextFieldSubmitEvent, TextFieldSupportingText, TextFieldVariant,
    };

    // Progress
    pub use crate::progress::{
        CIRCULAR_PROGRESS_SIZE, CIRCULAR_PROGRESS_TRACK_WIDTH, CircularProgressBuilder,
        LINEAR_PROGRESS_HEIGHT, LinearProgressBuilder, MaterialCircularProgress,
        MaterialLinearProgress, ProgressIndicator, ProgressMode, ProgressPlugin, ProgressTrack,
        ProgressVariant, SpawnProgressChild,
    };

    // Dialog
    pub use crate::dialog::{
        DIALOG_MAX_WIDTH, DIALOG_MIN_WIDTH, DialogActions, DialogBuilder, DialogCloseEvent,
        DialogConfirmEvent, DialogContent, DialogHeadline, DialogOpenEvent, DialogPlugin,
        DialogScrim, DialogType, MaterialDialog, MaterialDialogAnchor, MaterialDialogPlacement,
        SpawnDialogChild, create_dialog_scrim, create_dialog_scrim_for,
    };

    // Date Picker
    pub use crate::date_picker::{
        CalendarConstraints, Date, DateInputMode, DatePickerBuilder, DatePickerCancelEvent,
        DatePickerMode, DatePickerSubmitEvent, DateSelection, DateSelector, DateValidator,
        MaterialDatePicker, Month, RangeDateSelector, SingleDateSelector, SpawnDatePicker, Weekday,
    };

    // Time Picker
    pub use crate::time_picker::{
        MaterialTimePicker, SpawnTimePicker, TimeFormat, TimeInputMode, TimePeriod,
        TimePickerBuilder, TimePickerCancelEvent, TimePickerSubmitEvent,
    };

    // List
    pub use crate::list::{
        ListBuilder, ListDivider, ListItemBody, ListItemBuilder, ListItemClickEvent,
        ListItemHeadline, ListItemLeading, ListItemSupportingText, ListItemTrailing,
        ListItemVariant, ListPlugin, ListSelectionMode, MaterialList, MaterialListItem,
        ScrollableList, SpawnListChild, create_list_divider,
    };

    // Menu
    pub use crate::menu::{
        MENU_ITEM_HEIGHT, MENU_MAX_WIDTH, MENU_MIN_WIDTH, MaterialMenu, MaterialMenuItem,
        MenuAnchor, MenuBuilder, MenuCloseEvent, MenuDivider, MenuItemBuilder, MenuItemSelectEvent,
        MenuOpenEvent, MenuPlugin, SpawnMenuChild, create_menu_divider,
    };

    // Tabs
    pub use crate::tabs::{
        MaterialTab, MaterialTabs, SpawnTabsChild, TAB_HEIGHT_PRIMARY,
        TAB_HEIGHT_PRIMARY_ICON_ONLY, TAB_HEIGHT_SECONDARY, TAB_INDICATOR_HEIGHT, TabBuilder,
        TabChangeEvent, TabContent, TabIndicator, TabLabelText, TabVariant, TabsBuilder,
        TabsPlugin, create_tab_indicator,
    };

    // Divider
    pub use crate::divider::{
        DIVIDER_INSET, DIVIDER_THICKNESS, DividerBuilder, DividerVariant, MaterialDivider,
        SpawnDividerChild, horizontal_divider, inset_divider, vertical_divider,
    };

    // Select
    pub use crate::select::{
        MaterialSelect, SELECT_HEIGHT, SELECT_OPTION_HEIGHT, SelectBuilder, SelectChangeEvent,
        SelectContainer, SelectDisplayText, SelectDropdown, SelectOption, SelectOptionItem,
        SelectPlugin, SelectTrigger, SelectVariant, SpawnSelectChild,
    };

    // Adaptive Layout
    pub use crate::adaptive::{
        WindowHeightClass, WindowSizeClass, WindowSizeClassChanged, WindowSizeClassPlugin,
        WindowWidthClass,
    };

    // Layout
    pub use crate::layout::{
        AdaptiveNavigationScaffold, BottomNavigationScaffold, ListDetailScaffold,
        ModalDrawerScaffold, NavigationBarScaffold, NavigationRailScaffold,
        NavigationSuiteScaffold, PaneEntities, PaneTestIds, PermanentDrawerScaffold,
        ScaffoldEntities, ScaffoldTestIds, StandardDrawerScaffold, SupportingPanesScaffold,
        spawn_adaptive_navigation_scaffold, spawn_bottom_navigation_scaffold,
        spawn_list_detail_scaffold, spawn_modal_drawer_scaffold, spawn_navigation_bar_scaffold,
        spawn_navigation_rail_scaffold, spawn_navigation_suite_scaffold,
        spawn_permanent_drawer_scaffold, spawn_standard_drawer_scaffold,
        spawn_supporting_panes_scaffold,
    };

    // Search
    pub use crate::search::{
        MaterialSearchBar, SEARCH_BAR_HEIGHT, SearchBarAction, SearchBarBuilder,
        SearchBarClickEvent, SearchBarNavigation, SearchPlugin, SearchQueryEvent,
        SpawnSearchBarChild,
    };

    // Toolbar
    pub use crate::toolbar::{
        MaterialToolbar, SpawnToolbarChild, TOOLBAR_HEIGHT, TOOLBAR_ICON_SIZE, ToolbarAction,
        ToolbarActionEvent, ToolbarBuilder, ToolbarNavigationEvent, ToolbarPlugin,
    };

    // Loading Indicator
    pub use crate::loading_indicator::{
        LOADING_INDICATOR_SIZE, LoadingIndicatorBuilder, LoadingIndicatorPlugin, LoadingShape,
        MaterialLoadingIndicator, ShapeMorphMaterial, SpawnLoadingIndicatorChild,
    };

    // Motion
    pub use crate::motion::{
        AnimatedValue, MotionPlugin, SpringConfig, StateLayer, ease_emphasized,
        ease_emphasized_accelerate, ease_emphasized_decelerate, ease_standard,
        ease_standard_accelerate, ease_standard_decelerate,
    };

    // Snackbar
    pub use crate::snackbar::{
        DismissSnackbar, SNACKBAR_MAX_WIDTH, ShowSnackbar, Snackbar, SnackbarActionEvent,
        SnackbarAnimationState, SnackbarBuilder, SnackbarHostBuilder, SnackbarPlugin,
        SnackbarPosition, SnackbarQueue, SpawnSnackbarChild, spawn_snackbar,
    };

    // Chip
    pub use crate::chip::{
        CHIP_HEIGHT, ChipBuilder, ChipClickEvent, ChipDeleteButton, ChipDeleteEvent, ChipLabel,
        ChipLeadingIcon, ChipPlugin, ChipVariant, MaterialChip, SpawnChipChild,
    };

    // App Bar
    pub use crate::app_bar::{
        AppBarPlugin, BOTTOM_APP_BAR_HEIGHT, BottomAppBar, BottomAppBarBuilder, SpawnAppBarChild,
        TOP_APP_BAR_HEIGHT_LARGE, TOP_APP_BAR_HEIGHT_MEDIUM, TOP_APP_BAR_HEIGHT_SMALL, TopAppBar,
        TopAppBarBuilder, TopAppBarVariant,
    };

    // Badge
    pub use crate::badge::{
        BADGE_SIZE_LARGE, BADGE_SIZE_SMALL, BadgeBuilder, BadgeContent, BadgePlugin, MaterialBadge,
        SpawnBadgeChild,
    };

    // Tooltip
    pub use crate::tooltip::{
        RichTooltip, SpawnTooltipChild, TOOLTIP_DELAY_DEFAULT, TOOLTIP_DELAY_SHORT,
        TOOLTIP_HEIGHT_PLAIN, TOOLTIP_MAX_WIDTH, TOOLTIP_OFFSET, Tooltip, TooltipAnimationState,
        TooltipPlugin, TooltipPosition, TooltipText, TooltipTrigger, TooltipTriggerBuilder,
        TooltipVariant, spawn_rich_tooltip, spawn_tooltip,
    };

    // Scroll Container
    pub use crate::scroll::{
        ScrollContainer, ScrollContainerBuilder, ScrollContent, ScrollDirection, ScrollPlugin,
        ScrollbarThumbHorizontal, ScrollbarThumbVertical, ScrollbarTrackHorizontal,
        ScrollbarTrackVertical, spawn_scrollbars,
    };

    // Main plugin
    pub use crate::MaterialUiPlugin;
}

// ============================================================================
// Main Plugin
// ============================================================================

/// Main plugin that adds all Material UI functionality to your Bevy app.
///
/// This plugin will:
/// - Initialize the Material theme resource
/// - Add component plugins for all components
/// - Set up the focus and ripple systems
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::MaterialUiPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(MaterialUiPlugin)
///     .run();
/// ```
pub struct MaterialUiPlugin;

impl Plugin for MaterialUiPlugin {
    fn build(&self, app: &mut App) {
        // Core systems (theme, icons, focus, ripple).
        app.add_plugins(MaterialUiCorePlugin);

        // Component plugins
        app.add_plugins((
            button::ButtonPlugin,
            icon_button::IconButtonPlugin,
            fab::FabPlugin,
            card::CardPlugin,
            checkbox::CheckboxPlugin,
            radio::RadioPlugin,
            switch::SwitchPlugin,
            slider::SliderPlugin,
            text_field::TextFieldPlugin,
            progress::ProgressPlugin,
            dialog::DialogPlugin,
            list::ListPlugin,
            menu::MenuPlugin,
            tabs::TabsPlugin,
            select::SelectPlugin,
        ));

        // Keep plugin tuples under Bevy's arity limit.
        app.add_plugins(button_group::ButtonGroupPlugin);

        // New component plugins
        app.add_plugins((
            motion::MotionPlugin,
            snackbar::SnackbarPlugin,
            chip::ChipPlugin,
            app_bar::AppBarPlugin,
            badge::BadgePlugin,
            tooltip::TooltipPlugin,
            scroll::ScrollPlugin,
            date_picker::DatePickerPlugin,
            time_picker::TimePickerPlugin,
            search::SearchPlugin,
            toolbar::ToolbarPlugin,
            loading_indicator::LoadingIndicatorPlugin,
        ));

        // Adaptive layout
        app.add_plugins(adaptive::WindowSizeClassPlugin);
    }
}

/// Core plugin that provides the shared foundations required by most components.
///
/// This is intended to be used as an internal dependency for component plugins
/// so downstream users can do either of:
/// - `app.add_plugins(MaterialUiPlugin)` (everything)
/// - `app.add_plugins(ButtonPlugin)` (single component; core dependencies auto-added)
pub struct MaterialUiCorePlugin;

impl Plugin for MaterialUiCorePlugin {
    fn build(&self, app: &mut App) {
        // Theme is a resource; initializing it is idempotent.
        app.init_resource::<theme::MaterialTheme>();

        // Locale is a resource; initializing it is idempotent.
        app.init_resource::<locale::MaterialLocale>();

        // i18n is a plugin (assets + systems), so guard it.
        if !app.is_plugin_added::<i18n::MaterialI18nPlugin>() {
            app.add_plugins(i18n::MaterialI18nPlugin);
        }

        // These are true plugins (adding twice panics), so guard them.
        if !app.is_plugin_added::<focus::FocusPlugin>() {
            app.add_plugins(focus::FocusPlugin);
        }
        if !app.is_plugin_added::<ripple::RipplePlugin>() {
            app.add_plugins(ripple::RipplePlugin);
        }
        if !app.is_plugin_added::<icons::MaterialIconsPlugin>() {
            app.add_plugins(icons::MaterialIconsPlugin);
        }
    }
}

/// A plugin group that adds Material UI plugins in stages.
/// Use this if you want more control over which plugins are added.
pub struct MaterialUiPlugins;

impl PluginGroup for MaterialUiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>().add(MaterialUiPlugin)
    }
}
