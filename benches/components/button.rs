use bevy_material_ui::button::{ButtonVariant, MaterialButton};
use criterion::{black_box, Criterion};

/// Benchmark button component creation
pub fn bench_button_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Button Component");

    group.bench_function("create_filled", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Filled),
            )
        })
    });

    group.bench_function("create_outlined", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Outlined),
            )
        })
    });

    group.bench_function("create_elevated", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Elevated),
            )
        })
    });

    group.bench_function("create_tonal", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::FilledTonal),
            )
        })
    });

    group.bench_function("create_text", |b| {
        b.iter(|| {
            black_box(MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Text))
        })
    });

    // Full configuration
    group.bench_function("create_full_config", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Submit"))
                    .with_variant(ButtonVariant::Filled)
                    .disabled(black_box(false))
                    .with_icon(black_box("send")),
            )
        })
    });

    // Batch creation
    group.bench_function("create_batch_10", |b| {
        b.iter(|| {
            let buttons: Vec<_> = (0..10)
                .map(|i| MaterialButton::new(format!("Button {}", i)))
                .collect();
            black_box(buttons)
        })
    });

    group.finish();
}
