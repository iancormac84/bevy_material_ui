use bevy_material_ui::search::MaterialSearchBar;
use criterion::{black_box, Criterion};

/// Benchmark search bar component
pub fn bench_search_bar(c: &mut Criterion) {
    let mut group = c.benchmark_group("SearchBar Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialSearchBar::new(black_box("Search..."))))
    });

    group.bench_function("create_with_text", |b| {
        b.iter(|| {
            black_box(MaterialSearchBar::new(black_box("Search...")).with_text(black_box("test")))
        })
    });

    group.finish();
}
