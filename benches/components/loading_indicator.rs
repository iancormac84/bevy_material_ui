use bevy_material_ui::loading_indicator::MaterialLoadingIndicator;
use criterion::{black_box, Criterion};

/// Benchmark loading indicator component
pub fn bench_loading_indicator(c: &mut Criterion) {
    let mut group = c.benchmark_group("LoadingIndicator Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new()))
    });

    group.bench_function("create_small", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().with_size(24.0)))
    });

    group.bench_function("create_large", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().with_size(64.0)))
    });

    group.bench_function("create_multi_color", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().multi_color()))
    });

    group.finish();
}
