use bevy_material_ui::switch::MaterialSwitch;
use criterion::{black_box, Criterion};

/// Benchmark switch component
pub fn bench_switch(c: &mut Criterion) {
    let mut group = c.benchmark_group("Switch Component");

    group.bench_function("create_off", |b| {
        b.iter(|| black_box(MaterialSwitch::new().selected(black_box(false))))
    });

    group.bench_function("create_on", |b| {
        b.iter(|| black_box(MaterialSwitch::new().selected(black_box(true))))
    });

    group.bench_function("create_with_icon", |b| {
        b.iter(|| black_box(MaterialSwitch::new().with_icon().selected(black_box(true))))
    });

    group.finish();
}
