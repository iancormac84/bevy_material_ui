use bevy::prelude::*;
use bevy_material_ui::{
    button::MaterialButton,
    checkbox::{CheckboxState, MaterialCheckbox},
    focus::Focusable,
    slider::MaterialSlider,
    switch::MaterialSwitch,
};
use criterion::{black_box, Criterion};

use super::setup_app;

/// Benchmark mixed component workload
pub fn bench_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mixed Workload");

    // Simulate a realistic UI with various components
    group.bench_function("realistic_ui_100_components", |b| {
        b.iter(|| {
            let mut app = setup_app();

            // Spawn a mix of components (typical form)
            // 20 buttons
            app.world_mut().spawn_batch((0..20).map(|i| {
                (
                    MaterialButton::new(format!("Button {}", i)),
                    Node::default(),
                )
            }));

            // 30 checkboxes
            app.world_mut().spawn_batch((0..30).map(|_| {
                (
                    MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                    Node::default(),
                )
            }));

            // 10 switches
            app.world_mut()
                .spawn_batch((0..10).map(|_| (MaterialSwitch::new(), Node::default())));

            // 20 sliders
            app.world_mut().spawn_batch((0..20).map(|i| {
                (
                    MaterialSlider::new(0.0, 100.0).with_value((i as f32 * 5.0) % 100.0),
                    Node::default(),
                )
            }));

            // 20 focusables
            app.world_mut()
                .spawn_batch((0..20).map(|_| (Focusable::new(), Node::default())));

            black_box(app.world().entities().len())
        })
    });

    // Simulate querying all interactive components
    group.bench_function("query_all_interactive", |b| {
        let mut app = setup_app();

        // Setup entities
        app.world_mut().spawn_batch((0..50).map(|i| {
            (
                MaterialButton::new(format!("Button {}", i)),
                Node::default(),
                Interaction::None,
            )
        }));

        app.world_mut().spawn_batch((0..50).map(|_| {
            (
                MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                Node::default(),
                Interaction::None,
            )
        }));

        b.iter(|| {
            let mut count = 0;
            let world = app.world_mut();

            // Query buttons
            let mut button_query = world.query::<(&MaterialButton, &Interaction)>();
            for (button, interaction) in button_query.iter(world) {
                black_box((button, interaction));
                count += 1;
            }

            // Query checkboxes
            let mut checkbox_query = world.query::<(&MaterialCheckbox, &Interaction)>();
            for (checkbox, interaction) in checkbox_query.iter(world) {
                black_box((checkbox, interaction));
                count += 1;
            }

            black_box(count)
        })
    });

    group.finish();
}
