use bevy_material_ui::divider::MaterialDivider;
use criterion::{black_box, Criterion};

/// Benchmark divider component
pub fn bench_divider(c: &mut Criterion) {
    let mut group = c.benchmark_group("Divider Component");

    group.bench_function("create_horizontal", |b| {
        b.iter(|| black_box(MaterialDivider::new()))
    });

    group.bench_function("create_vertical", |b| {
        b.iter(|| black_box(MaterialDivider::vertical()))
    });

    group.bench_function("create_inset", |b| {
        b.iter(|| black_box(MaterialDivider::new().inset()))
    });

    group.finish();
}
