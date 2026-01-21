use bevy_material_ui::{checkbox::CheckboxState, elevation::Elevation, slider::MaterialSlider};
use criterion::{black_box, Criterion};

/// Benchmark checkbox state transitions and related state changes.
pub fn bench_state_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("State Changes");

    // Checkbox state toggles
    group.bench_function("checkbox_state_toggle_100", |b| {
        let mut states: Vec<CheckboxState> = (0..100).map(|_| CheckboxState::Unchecked).collect();
        b.iter(|| {
            for state in states.iter_mut() {
                *state = state.toggle();
            }
            black_box(states.len())
        })
    });

    // Slider normalized value calculations
    group.bench_function("slider_normalize_100", |b| {
        let sliders: Vec<_> = (0..100)
            .map(|i| MaterialSlider::new(0.0, 100.0).with_value(i as f32))
            .collect();
        b.iter(|| {
            for slider in &sliders {
                black_box(slider.normalized_value());
            }
        })
    });

    // Elevation calculations
    group.bench_function("elevation_calculations_100", |b| {
        let elevations: Vec<_> = (0..100)
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
                black_box((
                    elevation.dp(),
                    elevation.shadow_opacity(),
                    elevation.shadow_blur(),
                ));
            }
        })
    });

    group.finish();
}
