use bevy::prelude::*;

use bevy_material_ui::{
    select::SpawnSelectChild,
    select::{SelectBuilder, SelectOption},
    theme::MaterialTheme,
};

#[derive(Resource, Clone, Debug)]
pub struct SelectSpawnBenchConfig {
    pub options_count: usize,
    pub virtualize: bool,
}

pub fn spawn_select_for_bench(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    config: Res<SelectSpawnBenchConfig>,
) {
    let options = (0..config.options_count)
        .map(|i| SelectOption::new(format!("Option {i}")))
        .collect::<Vec<_>>();

    commands.spawn(Node::default()).with_children(|root| {
        root.spawn_select_with(
            &theme,
            SelectBuilder::new(options)
                .label("Bench")
                .filled()
                .dropdown_max_height(Val::Px(240.0))
                .virtualize(config.virtualize)
                .width(Val::Px(320.0)),
        );
    });
}

/// Setup a minimal Bevy App for benchmarking
pub fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

pub mod component_queries;
pub mod elevation_calculations;
pub mod entity_spawning;
pub mod focus_updates;
pub mod mixed_workload;
pub mod ripple_updates;
pub mod scroll_operations;
pub mod select_dropdown_spawning;
pub mod theme_access;
