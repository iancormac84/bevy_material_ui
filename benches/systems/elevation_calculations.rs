use bevy_material_ui::elevation::{Elevation, ElevationShadow};
use criterion::{black_box, BenchmarkId, Criterion};

/// Benchmark elevation shadow calculations
pub fn bench_elevation_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Elevation Calculations");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("calculate_shadows", count),
            count,
            |b, &count| {
                let elevations: Vec<Elevation> = (0..count)
                    .map(|i| match i % 6 {
                        0 => Elevation::Level0,
                        1 => Elevation::Level1,
                        2 => Elevation::Level2,
                        3 => Elevation::Level3,
                        4 => Elevation::Level4,
                        _ => Elevation::Level5,
                    })
                    .collect();

                b.iter(|| {
                    for elevation in &elevations {
                        let shadow = ElevationShadow::from_elevation(*elevation);
                        black_box(shadow);
                    }
                })
            },
        );
    }

    group.finish();
}
