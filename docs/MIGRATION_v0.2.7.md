# Migration Guide: v0.2.6 → v0.2.7

This guide helps you migrate your code from **bevy_material_ui v0.2.6** to **v0.2.7**.

v0.2.7 is primarily a **Bevy 0.18 compatibility** release.

## Table of Contents

- [Breaking Changes](#breaking-changes)
  - [Bevy Version Bump (0.17 → 0.18)](#bevy-version-bump-017--018)
  - [BorderRadius Is Now Stored On Node](#borderradius-is-now-stored-on-node)
- [Behavior Changes](#behavior-changes)
  - [Dialogs: Z-order, Centering, Resize Reactivity](#dialogs-z-order-centering-resize-reactivity)
- [Notes](#notes)

## Breaking Changes

### Bevy Version Bump (0.17 → 0.18)

**What changed**: `bevy_material_ui` now depends on **Bevy 0.18**.

**Impact**:
- Your app must also upgrade to Bevy 0.18.
- You may need to follow Bevy’s own migration notes, especially for UI-related changes.

**Migration**:
- Update your root `Cargo.toml` to use `bevy = "0.18"` (or `0.18.x`).
- Resolve any downstream Bevy migration errors first (these will typically be the bulk of work).

### BorderRadius Is Now Stored On Node

**What changed**: Anywhere this crate previously attached `BorderRadius` as a standalone ECS component, it now sets `Node { border_radius: ... }` instead.

This aligns with Bevy 0.18 UI changes and fixes compile/runtime issues when spawning UI bundles.

**Why this matters to you**:
- If your app code **queried or mutated `BorderRadius` as a component** on UI entities created by this crate, those entities may no longer have a standalone `BorderRadius` component.
- You should treat border radius as part of the `Node` component, via `node.border_radius`.

#### Migration Steps

**Before (v0.2.6 + Bevy 0.17-style usage):**

```rust
fn round_more(mut q: Query<&mut BorderRadius, With<MyWidgetTag>>) {
    for mut radius in &mut q {
        *radius = BorderRadius::all(Val::Px(24.0));
    }
}
```

**After (v0.2.7 + Bevy 0.18):**

```rust
fn round_more(mut q: Query<&mut Node, With<MyWidgetTag>>) {
    for mut node in &mut q {
        node.border_radius = BorderRadius::all(Val::Px(24.0));
    }
}
```

**Common symptoms**:
- Compile errors like “invalid Bundle” when trying to spawn `BorderRadius` in a tuple/bundle.
- Runtime customization systems that used `Query<&mut BorderRadius, ...>` no longer finding matches.

## Behavior Changes

### Dialogs: Z-order, Centering, Resize Reactivity

Dialogs were updated to behave correctly under Bevy 0.18, especially in examples and `bevy_minimal` configurations.

**Notable changes**:
- **Viewport centering works even if `MaterialDialogAnchor` is removed**. This enables the “center window” pattern where the dialog is centered relative to the window rather than an anchor node.
- **Center-in-window reacts to window resizing** by recomputing the centered position from the current window size each frame while the dialog is open.
- **Z-ordering is stronger / more explicit**: dialogs and their scrims are assigned explicit `GlobalZIndex` values so dialogs consistently render above other UI.

**Potential impact**:
- If your app uses other overlays with very high `GlobalZIndex` values, you may need to adjust your overlay ordering strategy.

## Notes

- This release is mostly about being a good Bevy 0.18 citizen:
  - UI border radius is set through `Node`.
  - Several internal systems were updated to match newer Bevy scheduling and UI conventions.

If you hit a migration snag, try to reduce it to:
1) a minimal repro on Bevy 0.18, and
2) whether it’s a Bevy UI behavior change vs a `bevy_material_ui` behavior change.

See also: [CHANGELOG.md](../CHANGELOG.md)
