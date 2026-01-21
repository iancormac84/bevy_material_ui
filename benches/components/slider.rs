use bevy_material_ui::slider::MaterialSlider;
use criterion::{black_box, Criterion};

/// Benchmark slider component
pub fn bench_slider(c: &mut Criterion) {
    let mut group = c.benchmark_group("Slider Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialSlider::new(black_box(0.0), black_box(100.0))))
    });

    group.bench_function("create_with_value", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0)).with_value(black_box(50.0)),
            )
        })
    });

    group.bench_function("create_with_step", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0))
                    .with_value(black_box(50.0))
                    .with_step(black_box(5.0)),
            )
        })
    });

    group.bench_function("create_full_config", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0))
                    .with_value(black_box(50.0))
                    .with_step(black_box(5.0))
                    .show_label()
                    .show_ticks(),
            )
        })
    });

    // Normalize value
    group.bench_function("normalize_value", |b| {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(50.0);
        b.iter(|| black_box(slider.normalized_value()))
    });

    group.finish();
}
