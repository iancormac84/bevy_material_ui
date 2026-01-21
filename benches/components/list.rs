use bevy_material_ui::list::MaterialListItem;
use criterion::{black_box, Criterion};

/// Benchmark list component
pub fn bench_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("List Component");

    group.bench_function("create_list_item", |b| {
        b.iter(|| black_box(MaterialListItem::new(black_box("Item"))))
    });

    group.bench_function("create_list_item_with_icon", |b| {
        b.iter(|| {
            black_box(MaterialListItem::new(black_box("Item")).leading_icon(black_box("person")))
        })
    });

    group.bench_function("create_list_item_full", |b| {
        b.iter(|| {
            black_box(
                MaterialListItem::new(black_box("Title"))
                    .leading_icon(black_box("mail"))
                    .supporting_text(black_box("Supporting text"))
                    .trailing_icon(black_box("chevron_right")),
            )
        })
    });

    group.finish();
}
