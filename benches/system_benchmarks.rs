//! System Benchmarks
//!
//! Measures the performance of Bevy ECS systems for UI components.
//! These benchmarks use actual Bevy World and systems.

mod systems;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    systems::entity_spawning::bench_entity_spawning,
    systems::component_queries::bench_component_queries,
    systems::select_dropdown_spawning::bench_select_dropdown_spawning,
    systems::ripple_updates::bench_ripple_updates,
    systems::focus_updates::bench_focus_updates,
    systems::elevation_calculations::bench_elevation_calculations,
    systems::theme_access::bench_theme_access,
    systems::mixed_workload::bench_mixed_workload,
    systems::scroll_operations::bench_scroll_operations,
);

criterion_main!(benches);
