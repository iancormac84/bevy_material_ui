use bevy::prelude::Startup;
use bevy_material_ui::theme::MaterialTheme;
use criterion::{black_box, BenchmarkId, Criterion};

use super::{setup_app, spawn_select_for_bench, SelectSpawnBenchConfig};

/// Benchmark select dropdown spawning with and without virtualization.
///
/// This is an ECS/system-level benchmark: it measures the cost of building the entity tree
/// for a dropdown with many options.
pub fn bench_select_dropdown_spawning(c: &mut Criterion) {
    let mut group = c.benchmark_group("Select Dropdown Spawning");

    for &count in [100usize, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("non_virtualized", count),
            &count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.insert_resource(MaterialTheme::default());
                    app.insert_resource(SelectSpawnBenchConfig {
                        options_count: count,
                        virtualize: false,
                    });
                    app.add_systems(Startup, spawn_select_for_bench);
                    app.update();
                    black_box(app.world().entities().len())
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("virtualized", count),
            &count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.insert_resource(MaterialTheme::default());
                    app.insert_resource(SelectSpawnBenchConfig {
                        options_count: count,
                        virtualize: true,
                    });
                    app.add_systems(Startup, spawn_select_for_bench);
                    app.update();
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    group.finish();
}
