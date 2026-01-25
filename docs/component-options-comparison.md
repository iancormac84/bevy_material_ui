# Component options and demo coverage

This document tracks component demo coverage and serves as a safe checklist. It does **not** list unverified option parity against Material 3. The API surface in the codebase is the source of truth.

If you need option coverage details for a component, derive them from the public types in the source (for example the component builder structs and theme tokens).

## Demo coverage

| Component | Example demo | Notes |
|---|---|---|
| App Bar | [examples/app_bar_demo.rs](../examples/app_bar_demo.rs) | |
| Badge | [examples/badge_demo.rs](../examples/badge_demo.rs) | |
| Button | [examples/button_demo.rs](../examples/button_demo.rs) | |
| Button Group | [examples/button_group_demo.rs](../examples/button_group_demo.rs) | |
| Card | [examples/card_demo.rs](../examples/card_demo.rs) | |
| Checkbox | [examples/checkbox_demo.rs](../examples/checkbox_demo.rs) | |
| Chip | [examples/chip_demo.rs](../examples/chip_demo.rs) | |
| Date Picker | [examples/date_picker_demo.rs](../examples/date_picker_demo.rs) | |
| Dialog | [examples/dialog_demo.rs](../examples/dialog_demo.rs) | |
| Divider | [examples/divider_demo.rs](../examples/divider_demo.rs) | |
| Elevation | [examples/elevation_demo.rs](../examples/elevation_demo.rs) | |
| FAB | [examples/fab_demo.rs](../examples/fab_demo.rs) | |
| Icon Button | [examples/icon_button_demo.rs](../examples/icon_button_demo.rs) | |
| Icons | [examples/icons_demo.rs](../examples/icons_demo.rs), [examples/all_icon_buttons.rs](../examples/all_icon_buttons.rs) | |
| Layout Scaffolds | [examples/layouts_demo.rs](../examples/layouts_demo.rs) | Includes drawer scaffolds and layout stacks. |
| List | [examples/list_demo.rs](../examples/list_demo.rs) | |
| Loading Indicator | [examples/loading_indicator_demo.rs](../examples/loading_indicator_demo.rs) | |
| Menu | [examples/menu_demo.rs](../examples/menu_demo.rs) | |
| Motion | [examples/motion_demo.rs](../examples/motion_demo.rs) | |
| Progress | [examples/progress_demo.rs](../examples/progress_demo.rs) | |
| Radio | [examples/radio_demo.rs](../examples/radio_demo.rs) | |
| Ripple | [examples/ripple_demo.rs](../examples/ripple_demo.rs) | |
| Scroll | [examples/scroll_demo.rs](../examples/scroll_demo.rs) | |
| Search | [examples/search_demo.rs](../examples/search_demo.rs) | |
| Select | [examples/select_demo.rs](../examples/select_demo.rs) | |
| Slider | [examples/slider_demo.rs](../examples/slider_demo.rs) | |
| Snackbar | [examples/snackbar_demo.rs](../examples/snackbar_demo.rs) | |
| Switch | [examples/switch_demo.rs](../examples/switch_demo.rs) | |
| Tabs | [examples/tabs_demo.rs](../examples/tabs_demo.rs) | |
| Text Field | [examples/textfield_demo.rs](../examples/textfield_demo.rs) | |
| Theme | [examples/theme_demo.rs](../examples/theme_demo.rs) | |
| Time Picker | [examples/time_picker_demo.rs](../examples/time_picker_demo.rs) | |
| Toolbar | [examples/toolbar_demo.rs](../examples/toolbar_demo.rs) | |
| Tooltip | [examples/tooltip_demo.rs](../examples/tooltip_demo.rs) | |
| Translations | [examples/translations_demo.rs](../examples/translations_demo.rs) | |
| Typography | [examples/typography_demo.rs](../examples/typography_demo.rs) | |
| Ui Shapes | [examples/ui_shapes_demo.rs](../examples/ui_shapes_demo.rs) | |


## Coverage notes (current pass)

This pass validated the API surface for the newly added demos:

- Elevation: `Elevation` enum and `to_box_shadow()` in [src/elevation.rs](../src/elevation.rs)
- Typography: `Typography` scale in [src/typography.rs](../src/typography.rs)
- Motion: `AnimatedValue`, `SpringAnimation`, `StateLayer` in [src/motion.rs](../src/motion.rs)
- Ripple: `RippleHost`, `SpawnRipple`, `Ripple` in [src/ripple.rs](../src/ripple.rs)
- Ui Shapes: `ShapePath`, `UiShapeBuilder`, `UiShapePlugin` in [src/ui_shapes.rs](../src/ui_shapes.rs)
