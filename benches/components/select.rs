use bevy_material_ui::select::{MaterialSelect, SelectOption, SelectVariant};
use criterion::{black_box, Criterion};

/// Benchmark select component
pub fn bench_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("Select Component");

    group.bench_function("create_small_10", |b| {
        b.iter(|| {
            let options: Vec<_> = (0..10)
                .map(|i| SelectOption::new(format!("Option {i}")))
                .collect();
            black_box(MaterialSelect::new(options))
        })
    });

    group.bench_function("create_large_1000", |b| {
        b.iter(|| {
            let options: Vec<_> = (0..1000)
                .map(|i| SelectOption::new(format!("Option {i}")))
                .collect();
            black_box(MaterialSelect::new(options))
        })
    });

    group.bench_function("create_full_config", |b| {
        b.iter(|| {
            let options: Vec<_> = (0..50)
                .map(|i| SelectOption::new(format!("Option {i}")))
                .collect();
            black_box(
                MaterialSelect::new(options)
                    .with_variant(black_box(SelectVariant::Outlined))
                    .label(black_box("Label"))
                    .supporting_text(black_box("Supporting"))
                    .selected(black_box(10))
                    .disabled(black_box(false))
                    .error(black_box(false)),
            )
        })
    });

    group.bench_function("display_text", |b| {
        let options: Vec<_> = (0..100)
            .map(|i| SelectOption::new(format!("Option {i}")))
            .collect();
        let select = MaterialSelect::new(options).selected(42);
        b.iter(|| black_box(select.display_text()))
    });

    group.finish();
}
