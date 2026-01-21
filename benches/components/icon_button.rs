use bevy_material_ui::icon_button::{IconButtonVariant, MaterialIconButton};
use criterion::{black_box, Criterion};

/// Benchmark icon button component
pub fn bench_icon_button(c: &mut Criterion) {
    let mut group = c.benchmark_group("IconButton Component");

    group.bench_function("create_standard", |b| {
        b.iter(|| black_box(MaterialIconButton::new(black_box("home"))))
    });

    group.bench_function("create_filled", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("favorite"))
                    .with_variant(IconButtonVariant::Filled),
            )
        })
    });

    group.bench_function("create_outlined", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("settings"))
                    .with_variant(IconButtonVariant::Outlined),
            )
        })
    });

    group.bench_function("create_tonal", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("share"))
                    .with_variant(IconButtonVariant::FilledTonal),
            )
        })
    });

    group.finish();
}
