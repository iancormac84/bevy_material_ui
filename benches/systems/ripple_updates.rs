use bevy::prelude::*;
use bevy_material_ui::ripple::Ripple;
use criterion::{black_box, BenchmarkId, Criterion};

use super::setup_app;

/// Benchmark ripple effect updates
pub fn bench_ripple_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ripple Updates");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("update_ripples", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities with ripples
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        Ripple::new(Vec2::ZERO, 100.0, Color::WHITE),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    // Simulate ripple update system
                    let world = app.world_mut();
                    let mut query = world.query::<&mut Ripple>();
                    for mut ripple in query.iter_mut(world) {
                        // Simulate animation progress
                        ripple.scale = (ripple.scale + 0.016).min(1.0);
                        if ripple.scale >= 1.0 && !ripple.fading_out {
                            ripple.start_fade_out();
                        }
                        black_box(&ripple);
                    }
                })
            },
        );
    }

    group.finish();
}
