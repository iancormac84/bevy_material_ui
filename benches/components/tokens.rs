use bevy_material_ui::{
    elevation::Elevation,
    tokens::{Duration, Easing, Spacing, corner_radius},
};
use criterion::Criterion;
use std::hint::black_box;

/// Benchmark design tokens
pub fn bench_tokens(c: &mut Criterion) {
    let mut group = c.benchmark_group("Design Tokens");

    group.bench_function("spacing_values", |b| {
        b.iter(|| {
            black_box((
                Spacing::EXTRA_SMALL,
                Spacing::SMALL,
                Spacing::MEDIUM,
                Spacing::LARGE,
                Spacing::EXTRA_LARGE,
            ))
        })
    });

    group.bench_function("corner_radius_values", |b| {
        b.iter(|| {
            black_box((
                corner_radius::NONE,
                corner_radius::EXTRA_SMALL,
                corner_radius::SMALL,
                corner_radius::MEDIUM,
                corner_radius::LARGE,
                corner_radius::EXTRA_LARGE,
                corner_radius::FULL,
            ))
        })
    });

    group.bench_function("elevation_values", |b| {
        b.iter(|| {
            black_box((
                Elevation::Level0,
                Elevation::Level1,
                Elevation::Level2,
                Elevation::Level3,
                Elevation::Level4,
                Elevation::Level5,
            ))
        })
    });

    group.bench_function("duration_values", |b| {
        b.iter(|| {
            black_box((
                Duration::SHORT1,
                Duration::SHORT2,
                Duration::MEDIUM1,
                Duration::MEDIUM2,
                Duration::LONG1,
                Duration::LONG2,
            ))
        })
    });

    group.bench_function("easing_control_points", |b| {
        b.iter(|| {
            black_box((
                Easing::Standard.control_points(),
                Easing::Emphasized.control_points(),
                Easing::EmphasizedDecelerate.control_points(),
            ))
        })
    });

    group.finish();
}
