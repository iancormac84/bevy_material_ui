//! Layouts Demo
//!
//! Demonstrates a simple Material 3 scaffold layout (content + bottom navigation).

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_HOME, ICON_SEARCH, ICON_SETTINGS};
use bevy_material_ui::layout::NavigationBarScaffold;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("layouts_demo/root", &telemetry)
        .with_children(|root| {
            let scaffold = NavigationBarScaffold::default();

            bevy_material_ui::layout::spawn_navigation_bar_scaffold(
                root,
                &theme,
                &scaffold,
                |content| {
                    content
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme.surface),
                        ))
                        .with_children(|center| {
                            center.spawn((
                                Text::new("Main content"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                },
                |nav| {
                    nav.spawn(IconButtonBuilder::new(ICON_HOME).standard().build(&theme))
                        .insert_test_id("layouts_demo/nav/home", &telemetry);
                    nav.spawn(IconButtonBuilder::new(ICON_SEARCH).standard().build(&theme))
                        .insert_test_id("layouts_demo/nav/search", &telemetry);
                    nav.spawn(IconButtonBuilder::new(ICON_SETTINGS).standard().build(&theme))
                        .insert_test_id("layouts_demo/nav/settings", &telemetry);
                },
            );
        });
}
