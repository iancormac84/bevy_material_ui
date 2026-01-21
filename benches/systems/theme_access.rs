use bevy_material_ui::theme::MaterialTheme;
use criterion::{black_box, Criterion};

use super::setup_app;

/// Benchmark theme resource access
pub fn bench_theme_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("Theme Access");

    group.bench_function("get_theme_colors", |b| {
        let mut app = setup_app();
        app.insert_resource(MaterialTheme::default());

        b.iter(|| {
            let theme = app.world().resource::<MaterialTheme>();
            black_box((
                theme.primary,
                theme.secondary,
                theme.tertiary,
                theme.surface,
                theme.on_surface,
            ))
        })
    });

    group.finish();
}
