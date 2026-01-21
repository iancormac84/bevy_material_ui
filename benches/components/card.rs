use bevy_material_ui::card::MaterialCard;
use criterion::{black_box, Criterion};

/// Benchmark card component
pub fn bench_card(c: &mut Criterion) {
    let mut group = c.benchmark_group("Card Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialCard::new()))
    });

    group.bench_function("create_clickable", |b| {
        b.iter(|| black_box(MaterialCard::new().clickable()))
    });

    group.bench_function("create_draggable", |b| {
        b.iter(|| black_box(MaterialCard::new().draggable()))
    });

    group.finish();
}
