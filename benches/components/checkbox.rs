use bevy_material_ui::checkbox::{CheckboxState, MaterialCheckbox};
use criterion::{black_box, Criterion};

/// Benchmark checkbox component
pub fn bench_checkbox(c: &mut Criterion) {
    let mut group = c.benchmark_group("Checkbox Component");

    group.bench_function("create_unchecked", |b| {
        b.iter(|| {
            black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Unchecked)))
        })
    });

    group.bench_function("create_checked", |b| {
        b.iter(|| black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Checked))))
    });

    group.bench_function("create_indeterminate", |b| {
        b.iter(|| {
            black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Indeterminate)))
        })
    });

    group.bench_function("state_toggle", |b| {
        b.iter(|| {
            let state = black_box(CheckboxState::Unchecked);
            let toggled = state.toggle();
            black_box(toggled)
        })
    });

    group.finish();
}
