use bevy_material_ui::{
    elevation::Elevation,
    tokens::{CornerRadius, Duration, Easing, Spacing},
};
use criterion::{black_box, Criterion};

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
                CornerRadius::NONE,
                CornerRadius::EXTRA_SMALL,
                CornerRadius::SMALL,
                CornerRadius::MEDIUM,
                CornerRadius::LARGE,
                CornerRadius::EXTRA_LARGE,
                CornerRadius::FULL,
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
