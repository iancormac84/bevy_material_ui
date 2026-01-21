use bevy_material_ui::radio::MaterialRadio;
use criterion::{black_box, Criterion};

/// Benchmark radio component
pub fn bench_radio(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radio Component");

    group.bench_function("create_single", |b| {
        b.iter(|| black_box(MaterialRadio::new().group(black_box("group1"))))
    });

    group.bench_function("create_group_5", |b| {
        b.iter(|| {
            let radios: Vec<_> = (0..5)
                .map(|i| MaterialRadio::new().group("group1").selected(i == 0))
                .collect();
            black_box(radios)
        })
    });

    group.finish();
}
