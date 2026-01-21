use bevy_material_ui::fab::{FabSize, MaterialFab};
use criterion::{black_box, Criterion};

/// Benchmark FAB component creation
pub fn bench_fab(c: &mut Criterion) {
    let mut group = c.benchmark_group("FAB Component");

    group.bench_function("create_regular", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add"))))
    });

    group.bench_function("create_small", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).with_size(FabSize::Small)))
    });

    group.bench_function("create_large", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).with_size(FabSize::Large)))
    });

    group.bench_function("create_extended", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).extended(black_box("Create"))))
    });

    group.finish();
}
