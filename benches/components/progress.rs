use bevy_material_ui::progress::{MaterialCircularProgress, MaterialLinearProgress};
use criterion::{black_box, Criterion};

/// Benchmark progress indicator
pub fn bench_progress(c: &mut Criterion) {
    let mut group = c.benchmark_group("Progress Component");

    group.bench_function("create_linear_determinate", |b| {
        b.iter(|| black_box(MaterialLinearProgress::new().with_progress(black_box(0.5))))
    });

    group.bench_function("create_linear_indeterminate", |b| {
        b.iter(|| black_box(MaterialLinearProgress::new().indeterminate()))
    });

    group.bench_function("create_circular", |b| {
        b.iter(|| black_box(MaterialCircularProgress::new().with_progress(black_box(0.75))))
    });

    group.finish();
}
