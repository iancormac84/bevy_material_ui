use bevy::prelude::*;
use bevy_material_ui::focus::Focusable;
use criterion::{black_box, BenchmarkId, Criterion};

use super::setup_app;

/// Benchmark focus ring updates
pub fn bench_focus_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Focus Updates");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("update_focus", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities with focusable components
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (Focusable::new(), Node::default())));

                b.iter(|| {
                    let world = app.world_mut();
                    let mut query = world.query::<&mut Focusable>();
                    for mut focusable in query.iter_mut(world) {
                        // Simulate focus toggle
                        focusable.focused = !focusable.focused;
                        focusable.focus_visible = focusable.focused;
                        black_box(&focusable);
                    }
                })
            },
        );
    }

    group.finish();
}
