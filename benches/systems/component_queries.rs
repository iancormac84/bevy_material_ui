use bevy::prelude::*;
use bevy_material_ui::button::{ButtonVariant, MaterialButton};
use criterion::{black_box, BenchmarkId, Criterion};

use super::setup_app;

/// Benchmark component queries
pub fn bench_component_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("Component Queries");

    // Query buttons
    for count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("query_buttons", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    let mut query_count = 0;
                    let world = app.world_mut();
                    let mut query = world.query::<&MaterialButton>();
                    for button in query.iter(world) {
                        black_box(button);
                        query_count += 1;
                    }
                    black_box(query_count)
                })
            },
        );
    }

    // Query with filters
    for count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("query_with_filter", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities, half with different variants
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)).with_variant(if i % 2 == 0 {
                            ButtonVariant::Filled
                        } else {
                            ButtonVariant::Outlined
                        }),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    let mut query_count = 0;
                    let world = app.world_mut();
                    let mut query = world.query_filtered::<&MaterialButton, With<Node>>();
                    for button in query.iter(world) {
                        black_box(button);
                        query_count += 1;
                    }
                    black_box(query_count)
                })
            },
        );
    }

    group.finish();
}
