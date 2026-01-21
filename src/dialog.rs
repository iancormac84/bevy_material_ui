//! Material Design 3 Dialog component
//!
//! Dialogs inform users about a task and can contain critical information.
//! This module leverages native `BoxShadow` for elevation shadows.
//!
//! Reference: <https://m3.material.io/components/dialogs/overview>

use bevy::ecs::system::ParamSet;
use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui::BoxShadow;
use bevy::ui::FocusPolicy;

use crate::{
    elevation::Elevation,
    i18n::LocalizedText,
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the dialog component
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<DialogOpenEvent>()
            .add_message::<DialogCloseEvent>()
            .add_message::<DialogConfirmEvent>()
            .init_resource::<DialogSpawnCounter>()
            .init_resource::<DialogOpenStack>()
            .add_systems(Startup, setup_dialog_overlay)
            .add_systems(
                Update,
                (
                    dialog_promote_to_overlay_system,
                    dialog_layer_z_index_system,
                    dialog_layer_visibility_system,
                    dialog_position_system,
                    dialog_dismiss_on_scrim_click_system,
                    dialog_dismiss_on_escape_system,
                    dialog_visibility_system,
                    dialog_scrim_visibility_system,
                    dialog_pickable_system,
                    dialog_scrim_pickable_system,
                    dialog_shadow_system,
                    dialog_telemetry_system,
                    dialog_scrim_telemetry_system,
                ),
            );
    }
}

/// Overlay root for dialog layers.
#[derive(Component)]
struct DialogOverlay;

/// Root entity that holds a dialog + its scrim.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
struct DialogLayerRootFor(Entity);

/// Override which UI element's rect is used as the dialog's placement bounds.
///
/// By default (when this component is not present), dialogs are anchored to their original
/// parent at spawn time.
///
/// You can provide your own anchor node by inserting `MaterialDialogAnchor(your_anchor_entity)`
/// on the dialog entity. This is especially useful for positioning relative to a trigger button
/// or a custom layout container.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialDialogAnchor(pub Entity);

/// Controls how the dialog is positioned relative to its `MaterialDialogAnchor`.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum MaterialDialogPlacement {
    /// Center the dialog within the anchor bounds.
    CenterInAnchor,
    /// Center the dialog within the window/viewport.
    ///
    /// This ignores the anchor bounds and instead centers within the dialog overlay.
    /// Useful for top-level dialogs or examples where you don't have a convenient anchor entity.
    CenterInViewport,
    /// Place the dialog below the anchor.
    BelowAnchor {
        /// Gap below the anchor (logical px).
        gap_px: f32,
    },
    /// Place the dialog above the anchor.
    AboveAnchor {
        /// Gap above the anchor (logical px).
        gap_px: f32,
    },
    /// Place the dialog to the right of the anchor.
    RightOfAnchor {
        /// Gap to the right of the anchor (logical px).
        gap_px: f32,
    },
    /// Place the dialog to the left of the anchor.
    LeftOfAnchor {
        /// Gap to the left of the anchor (logical px).
        gap_px: f32,
    },
}

impl Default for MaterialDialogPlacement {
    fn default() -> Self {
        Self::CenterInAnchor
    }
}

impl MaterialDialogPlacement {
    pub fn center_in_viewport() -> Self {
        Self::CenterInViewport
    }

    pub fn below_anchor(gap_px: f32) -> Self {
        Self::BelowAnchor { gap_px }
    }

    pub fn above_anchor(gap_px: f32) -> Self {
        Self::AboveAnchor { gap_px }
    }

    pub fn right_of_anchor(gap_px: f32) -> Self {
        Self::RightOfAnchor { gap_px }
    }

    pub fn left_of_anchor(gap_px: f32) -> Self {
        Self::LeftOfAnchor { gap_px }
    }
}

/// Monotonic spawn counter used to produce stable z-ordering.
#[derive(Resource, Default)]
struct DialogSpawnCounter(u64);

/// Stable spawn order for dialogs.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DialogSpawnOrder(u64);

/// Cached open stack ordered from bottom -> top.
#[derive(Resource, Default)]
struct DialogOpenStack {
    ordered: Vec<Entity>,
}

fn setup_dialog_overlay(mut commands: Commands) {
    // Full-screen container used as a portal destination for dialogs.
    // Must NOT be pickable so it doesn't interfere with input.
    commands.spawn((
        DialogOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
        GlobalZIndex(9000),
        Pickable::IGNORE,
    ));
}

/// Promote newly spawned dialogs into the global overlay so they're not clipped by
/// parent layout/overflow, while still using their original parent as an anchor.
fn dialog_promote_to_overlay_system(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    overlay_query: Query<Entity, With<DialogOverlay>>,
    mut spawn_counter: ResMut<DialogSpawnCounter>,
    added_dialogs: Query<(Entity, Option<&ChildOf>, &MaterialDialog), Added<MaterialDialog>>,
    existing_scrims: Query<(Entity, &DialogScrimFor), With<DialogScrim>>,
    existing_anchors: Query<&MaterialDialogAnchor>,
    existing_placements: Query<&MaterialDialogPlacement>,
) {
    let Some(overlay) = overlay_query.iter().next() else {
        return;
    };

    for (dialog_entity, parent, dialog) in added_dialogs.iter() {
        // Anchor placement to the caller-provided anchor if available.
        // Otherwise: use original parent if present, else fall back to the overlay.
        let anchor = existing_anchors
            .get(dialog_entity)
            .map(|a| a.0)
            .unwrap_or_else(|_| parent.map(|p| p.parent()).unwrap_or(overlay));

        // Ensure we always have a stable anchor available after promotion (ChildOf changes).
        commands
            .entity(dialog_entity)
            .insert(MaterialDialogAnchor(anchor));

        // Default placement if none was specified by caller.
        if existing_placements.get(dialog_entity).is_err() {
            commands
                .entity(dialog_entity)
                .insert(MaterialDialogPlacement::default());
        }

        // Assign stable spawn order for stacking.
        spawn_counter.0 += 1;
        let spawn_order = DialogSpawnOrder(spawn_counter.0);
        commands.entity(dialog_entity).insert(spawn_order);

        // Create a per-dialog layer root under the overlay.
        // Visibility is synced to the dialog's open state.
        let layer_root = commands
            .spawn((
                DialogLayerRootFor(dialog_entity),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::None,
                    ..default()
                },
            ))
            .id();

        // Attach the layer root to overlay, and move the dialog under it.
        commands.entity(overlay).add_child(layer_root);
        commands.entity(layer_root).add_child(dialog_entity);

        // Ensure dialog surface is always above its scrim.
        commands.entity(dialog_entity).insert((
            ZIndex(1),
            FocusPolicy::Block,
            // Dialog surface should never allow click-through.
            Pickable {
                should_block_lower: true,
                is_hoverable: false,
            },
        ));

        // Ensure we have a scrim for this dialog.
        // - If the user already spawned one via `create_dialog_scrim_for`, reuse it.
        // - Otherwise, spawn our default scrim.
        let scrim_entity = existing_scrims
            .iter()
            .find_map(|(entity, for_dialog)| (for_dialog.0 == dialog_entity).then_some(entity))
            .unwrap_or_else(|| {
                commands
                    .spawn((
                        DialogScrim,
                        DialogScrimFor(dialog_entity),
                        Node {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(dialog.scrim_color(&theme)),
                        ZIndex(0),
                    ))
                    .id()
            });

        // Make the scrim a proper input blocker for bevy_ui and bevy_picking.
        commands.entity(scrim_entity).remove::<GlobalZIndex>();
        commands.entity(scrim_entity).insert((
            // Required for bevy_ui `Interaction` updates.
            Button,
            Interaction::None,
            FocusPolicy::Block,
            if dialog.modal {
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                }
            } else {
                Pickable::IGNORE
            },
            ZIndex(0),
        ));

        commands.entity(layer_root).add_child(scrim_entity);
    }
}

/// Sync the per-dialog layer root's visibility (and scrim visibility) with dialog state.
fn dialog_layer_visibility_system(
    dialogs: Query<&MaterialDialog>,
    mut sets: ParamSet<(
        Query<(&DialogLayerRootFor, &mut Node)>,
        Query<(&DialogScrimFor, &mut Node), With<DialogScrim>>,
    )>,
) {
    for (for_dialog, mut root_node) in sets.p0().iter_mut() {
        let Ok(dialog) = dialogs.get(for_dialog.0) else {
            root_node.display = Display::None;
            continue;
        };

        root_node.display = if dialog.open {
            Display::Flex
        } else {
            Display::None
        };
    }

    for (for_dialog, mut scrim_node) in sets.p1().iter_mut() {
        let Ok(dialog) = dialogs.get(for_dialog.0) else {
            scrim_node.display = Display::None;
            continue;
        };

        // Only show the scrim when modal.
        scrim_node.display = if dialog.open && dialog.modal {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Maintain deterministic layering (nested dialogs above parents).
fn dialog_layer_z_index_system(
    mut commands: Commands,
    dialogs: Query<(Entity, &MaterialDialog, &DialogSpawnOrder)>,
    mut roots: Query<(Entity, &DialogLayerRootFor)>,
    mut open_stack: ResMut<DialogOpenStack>,
) {
    // Collect all open dialogs sorted by spawn order.
    let mut open_dialogs: Vec<(Entity, DialogSpawnOrder)> = dialogs
        .iter()
        .filter_map(|(entity, dialog, order)| dialog.open.then_some((entity, *order)))
        .collect();
    open_dialogs.sort_by_key(|(_, order)| *order);
    open_stack.ordered = open_dialogs.iter().map(|(e, _)| *e).collect();

    // Assign z-index in increasing order, leaving room for other overlays.
    // Each dialog gets a root promoted at a unique GlobalZIndex.
    for (root_entity, for_dialog) in roots.iter_mut() {
        let Some((idx, _)) = open_dialogs
            .iter()
            .enumerate()
            .find(|(_, (dialog_entity, _))| *dialog_entity == for_dialog.0)
        else {
            // Closed dialogs can keep their current z.
            continue;
        };

        // Base above other modal UIs (date/time pickers use 9999).
        let z = 10000 + (idx as i32);
        commands.entity(root_entity).insert(GlobalZIndex(z));
    }
}

/// Position dialogs relative to their original parent (anchor) rect.
///
/// This keeps dialogs independent of parent layout/clipping by rendering them in the overlay,
/// while still using the parent as the placement bounds.
fn dialog_position_system(
    mut dialogs: Query<(
        &MaterialDialog,
        &MaterialDialogAnchor,
        Option<&MaterialDialogPlacement>,
        &mut Node,
        &ComputedNode,
    )>,
    anchors: Query<(&UiGlobalTransform, &ComputedNode)>,
    roots: Query<&DialogLayerRootFor>,
    overlay_query: Query<(&UiGlobalTransform, &ComputedNode), With<DialogOverlay>>,
    windows: Query<&Window>,
) {
    let scale_factor = windows
        .iter()
        .next()
        .map(|w| w.scale_factor())
        .unwrap_or(1.0);

    let (overlay_center, overlay_size) = overlay_query
        .iter()
        .next()
        .map(|(t, c)| (t.translation, c.size()))
        .unwrap_or((Vec2::ZERO, Vec2::ZERO));
    let overlay_top_left = overlay_center - overlay_size / 2.0;

    for for_dialog in roots.iter() {
        let Ok((dialog, anchor, placement, mut dialog_node, dialog_computed)) =
            dialogs.get_mut(for_dialog.0)
        else {
            continue;
        };

        if !dialog.open {
            continue;
        }

        // Fullscreen dialogs just fill the overlay.
        if dialog.dialog_type == DialogType::FullScreen {
            dialog_node.left = Val::Px(0.0);
            dialog_node.top = Val::Px(0.0);
            continue;
        }

        let scale = scale_factor;

        let dialog_size_physical = dialog_computed.size();
        if dialog_size_physical.x <= 0.0 || dialog_size_physical.y <= 0.0 {
            // Not laid out yet.
            continue;
        }

        let placement = placement.copied().unwrap_or_default();

        let (screen_left_physical, screen_top_physical) = match placement {
            MaterialDialogPlacement::CenterInViewport => (
                // Center within the overlay (physical pixels).
                overlay_top_left.x + (overlay_size.x - dialog_size_physical.x) / 2.0,
                overlay_top_left.y + (overlay_size.y - dialog_size_physical.y) / 2.0,
            ),
            other => {
                // All other placements require an anchor rect.
                let Ok((anchor_transform, anchor_computed)) = anchors.get(anchor.0) else {
                    continue;
                };

                // UiGlobalTransform and ComputedNode sizes are physical pixels.
                let anchor_center_physical = anchor_transform.translation;
                let anchor_size_physical = anchor_computed.size();
                if anchor_size_physical.x <= 0.0 || anchor_size_physical.y <= 0.0 {
                    continue;
                }

                let anchor_top_left_physical = anchor_center_physical - anchor_size_physical / 2.0;

                match other {
                    MaterialDialogPlacement::CenterInViewport => unreachable!(),
                    MaterialDialogPlacement::CenterInAnchor => (
                        anchor_top_left_physical.x
                            + (anchor_size_physical.x - dialog_size_physical.x) / 2.0,
                        anchor_top_left_physical.y
                            + (anchor_size_physical.y - dialog_size_physical.y) / 2.0,
                    ),
                    MaterialDialogPlacement::BelowAnchor { gap_px } => {
                        let gap_physical = gap_px * scale;
                        (
                            anchor_top_left_physical.x
                                + (anchor_size_physical.x - dialog_size_physical.x) / 2.0,
                            anchor_top_left_physical.y + anchor_size_physical.y + gap_physical,
                        )
                    }
                    MaterialDialogPlacement::AboveAnchor { gap_px } => {
                        let gap_physical = gap_px * scale;
                        (
                            anchor_top_left_physical.x
                                + (anchor_size_physical.x - dialog_size_physical.x) / 2.0,
                            anchor_top_left_physical.y - dialog_size_physical.y - gap_physical,
                        )
                    }
                    MaterialDialogPlacement::RightOfAnchor { gap_px } => {
                        let gap_physical = gap_px * scale;
                        (
                            anchor_top_left_physical.x + anchor_size_physical.x + gap_physical,
                            anchor_top_left_physical.y
                                + (anchor_size_physical.y - dialog_size_physical.y) / 2.0,
                        )
                    }
                    MaterialDialogPlacement::LeftOfAnchor { gap_px } => {
                        let gap_physical = gap_px * scale;
                        (
                            anchor_top_left_physical.x - dialog_size_physical.x - gap_physical,
                            anchor_top_left_physical.y
                                + (anchor_size_physical.y - dialog_size_physical.y) / 2.0,
                        )
                    }
                }
            }
        };

        // Convert to logical pixels relative to the overlay.
        let left = (screen_left_physical - overlay_top_left.x) / scale;
        let top = (screen_top_physical - overlay_top_left.y) / scale;

        dialog_node.left = Val::Px(left);
        dialog_node.top = Val::Px(top);
    }
}

fn dialog_dismiss_on_scrim_click_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut close_events: MessageWriter<DialogCloseEvent>,
    mut dialogs: Query<&mut MaterialDialog>,
    mut scrims: Query<(&DialogScrimFor, &Interaction), (With<DialogScrim>, Changed<Interaction>)>,
) {
    for (for_dialog, interaction) in scrims.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Avoid immediately dismissing a dialog when its scrim becomes visible while the
        // mouse button is still held down from clicking the trigger.
        if !mouse.just_pressed(MouseButton::Left) {
            continue;
        }

        let Ok(mut dialog) = dialogs.get_mut(for_dialog.0) else {
            continue;
        };
        if !dialog.open || !dialog.dismiss_on_scrim_click {
            continue;
        }

        dialog.open = false;
        close_events.write(DialogCloseEvent {
            entity: for_dialog.0,
            dismissed: true,
        });
    }
}

fn dialog_dismiss_on_escape_system(
    keys: Res<ButtonInput<KeyCode>>,
    open_stack: Res<DialogOpenStack>,
    mut close_events: MessageWriter<DialogCloseEvent>,
    mut dialogs: Query<&mut MaterialDialog>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    let Some(&topmost) = open_stack.ordered.last() else {
        return;
    };

    let Ok(mut dialog) = dialogs.get_mut(topmost) else {
        return;
    };

    if !dialog.open || !dialog.dismiss_on_escape {
        return;
    }

    dialog.open = false;
    close_events.write(DialogCloseEvent {
        entity: topmost,
        dismissed: true,
    });
}

/// Update dialog pickability when dialog modality changes.
///
/// This prevents clicks from going through the dialog surface to UI behind it.
fn dialog_pickable_system(
    mut commands: Commands,
    changed_dialogs: Query<(Entity, &MaterialDialog), Changed<MaterialDialog>>,
    mut pickables: Query<&mut Pickable>,
) {
    if changed_dialogs.is_empty() {
        return;
    }

    for (entity, _dialog) in changed_dialogs.iter() {
        // Dialog surfaces should always block click-through. Modality is enforced by the scrim.
        let pickable = Pickable {
            should_block_lower: true,
            is_hoverable: false,
        };

        if let Ok(mut existing) = pickables.get_mut(entity) {
            *existing = pickable;
        } else {
            commands.entity(entity).insert(pickable);
        }
    }
}

fn dialog_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    dialogs: Query<(&TestId, &Children), With<MaterialDialog>>,
    children_query: Query<&Children>,
    headlines: Query<(), With<DialogHeadline>>,
    contents: Query<(), With<DialogContent>>,
    actions: Query<(), With<DialogActions>>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (test_id, children) in dialogs.iter() {
        let base = test_id.id();

        let mut found_headline = false;
        let mut found_content = false;
        let mut found_actions = false;

        let mut stack: Vec<Entity> = children.iter().collect();
        while let Some(entity) = stack.pop() {
            if !found_headline && headlines.get(entity).is_ok() {
                found_headline = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/headline"),
                });
            }

            if !found_content && contents.get(entity).is_ok() {
                found_content = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/content"),
                });
            }

            if !found_actions && actions.get(entity).is_ok() {
                found_actions = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/actions"),
                });
            }

            if found_headline && found_content && found_actions {
                break;
            }

            if let Ok(children) = children_query.get(entity) {
                stack.extend(children.iter());
            }
        }
    }
}

fn dialog_scrim_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    scrims: Query<(Entity, &DialogScrimFor), With<DialogScrim>>,
    dialogs: Query<&TestId, With<MaterialDialog>>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (scrim_entity, for_dialog) in scrims.iter() {
        let Ok(dialog_id) = dialogs.get(for_dialog.0) else {
            continue;
        };
        let base = dialog_id.id();

        commands.queue(InsertTestIdIfExists {
            entity: scrim_entity,
            id: format!("{base}/scrim"),
        });
    }
}

/// Dialog types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DialogType {
    /// Basic dialog with title and content
    #[default]
    Basic,
    /// Full-screen dialog
    FullScreen,
}

/// Material dialog component
#[derive(Component)]
pub struct MaterialDialog {
    /// Dialog type
    pub dialog_type: DialogType,
    /// Whether the dialog is currently open
    pub open: bool,
    /// Dialog title
    pub title: Option<String>,
    /// Dialog icon
    pub icon: Option<String>,
    /// Whether clicking the scrim closes the dialog
    pub dismiss_on_scrim_click: bool,
    /// Whether pressing Escape closes the dialog
    pub dismiss_on_escape: bool,

    /// Whether the dialog should behave as a modal (block pointer interactions behind it).
    ///
    /// When `true`, the dialog scrim will be pickable and will block lower entities from receiving
    /// pointer interactions. When `false`, the scrim will be click-through.
    pub modal: bool,
}

impl MaterialDialog {
    /// Create a new dialog
    pub fn new() -> Self {
        Self {
            dialog_type: DialogType::default(),
            open: false,
            title: None,
            icon: None,
            dismiss_on_scrim_click: true,
            dismiss_on_escape: true,
            modal: true,
        }
    }

    /// Set the dialog type
    pub fn with_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set initial open state
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Disable scrim click dismissal
    pub fn no_scrim_dismiss(mut self) -> Self {
        self.dismiss_on_scrim_click = false;
        self
    }

    /// Disable escape key dismissal
    pub fn no_escape_dismiss(mut self) -> Self {
        self.dismiss_on_escape = false;
        self
    }

    /// Set whether the dialog is modal (blocks pointer interactions behind it).
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Get the surface color
    pub fn surface_color(&self, theme: &MaterialTheme) -> Color {
        theme.surface_container_high
    }

    /// Get the scrim color
    pub fn scrim_color(&self, theme: &MaterialTheme) -> Color {
        theme.scrim.with_alpha(0.32)
    }

    /// Get the title color
    pub fn title_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_surface
    }

    /// Get the content color
    pub fn content_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_surface_variant
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        theme.secondary
    }

    /// Get the elevation
    pub fn elevation(&self) -> Elevation {
        Elevation::Level3
    }
}

impl Default for MaterialDialog {
    fn default() -> Self {
        Self::new()
    }
}

/// Event to open a dialog
#[derive(Event, bevy::prelude::Message)]
pub struct DialogOpenEvent {
    pub entity: Entity,
}

/// Event when dialog is closed
#[derive(Event, bevy::prelude::Message)]
pub struct DialogCloseEvent {
    pub entity: Entity,
    /// Whether it was dismissed (scrim/escape) vs confirmed
    pub dismissed: bool,
}

/// Event when dialog action is confirmed
#[derive(Event, bevy::prelude::Message)]
pub struct DialogConfirmEvent {
    pub entity: Entity,
}

/// Dialog dimensions
pub const DIALOG_MIN_WIDTH: f32 = 280.0;
pub const DIALOG_MAX_WIDTH: f32 = 560.0;

/// System to handle dialog visibility
fn dialog_visibility_system(
    mut dialogs: Query<(&MaterialDialog, &mut Node), Changed<MaterialDialog>>,
) {
    for (dialog, mut node) in dialogs.iter_mut() {
        node.display = if dialog.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// System to update dialog shadows using native BoxShadow
fn dialog_shadow_system(
    mut dialogs: Query<(&MaterialDialog, &mut BoxShadow), Changed<MaterialDialog>>,
) {
    for (dialog, mut shadow) in dialogs.iter_mut() {
        // Only show shadow when dialog is open
        if dialog.open {
            *shadow = dialog.elevation().to_box_shadow();
        } else {
            *shadow = BoxShadow::default();
        }
    }
}

/// Keep dialog scrims in sync with their dialog's open state.
fn dialog_scrim_visibility_system(
    dialogs: Query<&MaterialDialog>,
    mut scrims: Query<(&DialogScrimFor, &mut Node), With<DialogScrim>>,
) {
    for (for_dialog, mut node) in scrims.iter_mut() {
        let Ok(dialog) = dialogs.get(for_dialog.0) else {
            node.display = Display::None;
            continue;
        };

        node.display = if dialog.open && dialog.modal {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// Update scrim pickability when dialog modality changes.
fn dialog_scrim_pickable_system(
    dialogs: Query<&MaterialDialog>,
    mut scrims: Query<(&DialogScrimFor, &mut Pickable), With<DialogScrim>>,
) {
    for (for_dialog, mut pickable) in scrims.iter_mut() {
        let Ok(dialog) = dialogs.get(for_dialog.0) else {
            *pickable = Pickable::IGNORE;
            continue;
        };

        // Scrims should *only* block pointer interactions while visible.
        // Otherwise they can accidentally intercept clicks even when hidden.
        *pickable = if dialog.open && dialog.modal {
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
            }
        } else {
            Pickable::IGNORE
        };
    }
}

/// Builder for dialogs
pub struct DialogBuilder {
    dialog: MaterialDialog,
    title_key: Option<String>,
}

impl DialogBuilder {
    /// Create a new dialog builder
    pub fn new() -> Self {
        Self {
            dialog: MaterialDialog::new(),
            title_key: None,
        }
    }

    /// Set dialog type
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog.dialog_type = dialog_type;
        self
    }

    /// Make full-screen dialog
    pub fn full_screen(self) -> Self {
        self.dialog_type(DialogType::FullScreen)
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.dialog.title = Some(title.into());
        self
    }

    /// Set title from an i18n key.
    pub fn title_key(mut self, key: impl Into<String>) -> Self {
        self.dialog.title = Some(String::new());
        self.title_key = Some(key.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.dialog.icon = Some(icon.into());
        self
    }

    /// Start open
    pub fn open(mut self) -> Self {
        self.dialog.open = true;
        self
    }

    /// Disable scrim dismissal
    pub fn no_scrim_dismiss(mut self) -> Self {
        self.dialog.dismiss_on_scrim_click = false;
        self
    }

    /// Disable escape dismissal
    pub fn no_escape_dismiss(mut self) -> Self {
        self.dialog.dismiss_on_escape = false;
        self
    }

    /// Set whether the dialog is modal (blocks pointer interactions behind it).
    pub fn modal(mut self, modal: bool) -> Self {
        self.dialog.modal = modal;
        self
    }

    /// Build the dialog bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.dialog.surface_color(theme);
        let is_full_screen = self.dialog.dialog_type == DialogType::FullScreen;
        let modal = self.dialog.modal;

        (
            self.dialog,
            Node {
                display: Display::None, // Hidden by default
                position_type: PositionType::Absolute,
                width: if is_full_screen {
                    Val::Percent(100.0)
                } else {
                    Val::Auto
                },
                height: if is_full_screen {
                    Val::Percent(100.0)
                } else {
                    Val::Auto
                },
                min_width: if is_full_screen {
                    Val::Auto
                } else {
                    Val::Px(DIALOG_MIN_WIDTH)
                },
                max_width: if is_full_screen {
                    Val::Auto
                } else {
                    Val::Px(DIALOG_MAX_WIDTH)
                },
                padding: UiRect::all(Val::Px(Spacing::EXTRA_LARGE)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(if is_full_screen {
                0.0
            } else {
                CornerRadius::EXTRA_LARGE
            })),
            // Native Bevy 0.17 shadow support (starts hidden since dialog is closed)
            BoxShadow::default(),
            // Ensure modal dialogs block pointer interactions behind the dialog surface.
            if modal {
                Pickable {
                    should_block_lower: true,
                    is_hoverable: false,
                }
            } else {
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                }
            },
        )
    }
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for dialog scrim overlay
#[derive(Component)]
pub struct DialogScrim;

/// Associates a dialog scrim with a specific dialog entity.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogScrimFor(pub Entity);

/// Marker for dialog headline/title
#[derive(Component)]
pub struct DialogHeadline;

/// Marker for dialog content area
#[derive(Component)]
pub struct DialogContent;

/// Marker for dialog actions area
#[derive(Component)]
pub struct DialogActions;

/// Helper to create a dialog scrim
pub fn create_dialog_scrim(theme: &MaterialTheme) -> impl Bundle {
    (
        DialogScrim,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.scrim.with_alpha(0.32)),
        // Default scrim behavior is modal: block pointer interactions behind it.
        Pickable {
            should_block_lower: true,
            is_hoverable: false,
        },
    )
}

/// Helper to create a dialog scrim linked to a specific dialog entity.
///
/// The scrim starts hidden and is shown/hidden automatically based on the dialog's `open` state.
/// When `modal` is true, the scrim blocks pointer interactions behind it.
pub fn create_dialog_scrim_for(
    theme: &MaterialTheme,
    dialog_entity: Entity,
    modal: bool,
) -> impl Bundle {
    (
        DialogScrim,
        DialogScrimFor(dialog_entity),
        Node {
            display: Display::None, // Hidden by default; synced by system.
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.scrim.with_alpha(0.32)),
        if modal {
            Pickable {
                should_block_lower: true,
                is_hoverable: false,
            }
        } else {
            Pickable::IGNORE
        },
    )
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material dialogs as children
///
/// This trait provides a clean API for spawning dialogs within UI hierarchies.
///
/// ## Example:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::dialog::SpawnDialogChild;
/// use bevy_material_ui::theme::MaterialTheme;
///
/// fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
///     commands.spawn(Node::default()).with_children(|children| {
///         children.spawn_dialog(&theme, "Confirm", |dialog| {
///             dialog.spawn((Text::new("Are you sure?"), TextColor(theme.on_surface)));
///         });
///     });
/// }
/// ```
pub trait SpawnDialogChild {
    /// Spawn a dialog with headline and content builder
    fn spawn_dialog(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a dialog with full builder control
    fn spawn_dialog_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DialogBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a dialog scrim overlay
    fn spawn_dialog_scrim(&mut self, theme: &MaterialTheme);
}

impl SpawnDialogChild for ChildSpawnerCommands<'_> {
    fn spawn_dialog(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_dialog_with(theme, DialogBuilder::new().title(headline), with_content);
    }

    fn spawn_dialog_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DialogBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        let title_text: Option<String> = builder.dialog.title.clone();
        let title_key: Option<String> = builder.title_key.clone();
        let headline_color = theme.on_surface;

        self.spawn(builder.build(theme)).with_children(|dialog| {
            // Headline/Title
            if let Some(ref title) = title_text {
                if let Some(key) = title_key.as_deref() {
                    dialog.spawn((
                        DialogHeadline,
                        Text::new(""),
                        LocalizedText::new(key),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(headline_color),
                        Node {
                            margin: UiRect::bottom(Val::Px(16.0)),
                            ..default()
                        },
                    ));
                } else {
                    dialog.spawn((
                        DialogHeadline,
                        Text::new(title.as_str()),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(headline_color),
                        Node {
                            margin: UiRect::bottom(Val::Px(16.0)),
                            ..default()
                        },
                    ));
                }
            }

            // Content area
            dialog
                .spawn((
                    DialogContent,
                    Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        ..default()
                    },
                ))
                .with_children(with_content);
        });
    }

    fn spawn_dialog_scrim(&mut self, theme: &MaterialTheme) {
        self.spawn(create_dialog_scrim(theme));
    }
}
