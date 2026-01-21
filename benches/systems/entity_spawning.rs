use bevy::prelude::*;
use bevy_material_ui::{
    button::MaterialButton,
    card::MaterialCard,
    checkbox::{CheckboxState, MaterialCheckbox},
    divider::MaterialDivider,
    fab::MaterialFab,
    icon_button::MaterialIconButton,
    list::MaterialListItem,
    loading_indicator::MaterialLoadingIndicator,
    radio::MaterialRadio,
    search::MaterialSearchBar,
    slider::MaterialSlider,
    switch::MaterialSwitch,
};
use criterion::{black_box, BenchmarkId, Criterion};

use super::setup_app;

/// Benchmark entity spawning with UI components
pub fn bench_entity_spawning(c: &mut Criterion) {
    let mut group = c.benchmark_group("Entity Spawning");

    // Spawn button entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("buttons", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn checkbox entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("checkboxes", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn switch entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("switches", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialSwitch::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn slider entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("sliders", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        MaterialSlider::new(0.0, 100.0).with_value(50.0),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn radio entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("radios", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialRadio::new()
                            .selected(i % 2 == 0)
                            .group("test_group"),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn FAB entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("fabs", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialFab::new("add"), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn icon button entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("icon_buttons", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialIconButton::new("home"), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn card entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("cards", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialCard::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn list item entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("list_items", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialListItem::new(format!("Item {}", i)),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn loading indicator entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("loading_indicators", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialLoadingIndicator::new(), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn search bar entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("search_bars", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialSearchBar::new("Search..."), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn divider entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("dividers", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialDivider::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    group.finish();
}
