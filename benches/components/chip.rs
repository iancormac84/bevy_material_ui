use bevy_material_ui::chip::{ChipVariant, MaterialChip};
use criterion::{black_box, Criterion};

/// Benchmark chip component
pub fn bench_chip(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chip Component");

    group.bench_function("create_assist", |b| {
        b.iter(|| black_box(MaterialChip::new(black_box("Help")).with_variant(ChipVariant::Assist)))
    });

    group.bench_function("create_filter", |b| {
        b.iter(|| {
            black_box(MaterialChip::new(black_box("Category")).with_variant(ChipVariant::Filter))
        })
    });

    group.bench_function("create_input", |b| {
        b.iter(|| black_box(MaterialChip::new(black_box("Tag")).with_variant(ChipVariant::Input)))
    });

    group.bench_function("create_suggestion", |b| {
        b.iter(|| {
            black_box(
                MaterialChip::new(black_box("Recommended")).with_variant(ChipVariant::Suggestion),
            )
        })
    });

    // Chip group
    group.bench_function("create_filter_group_8", |b| {
        let labels = [
            "All", "Recent", "Starred", "Shared", "Archived", "Drafts", "Sent", "Trash",
        ];
        b.iter(|| {
            let chips: Vec<_> = labels
                .iter()
                .map(|&label| MaterialChip::new(label).with_variant(ChipVariant::Filter))
                .collect();
            black_box(chips)
        })
    });

    group.finish();
}
