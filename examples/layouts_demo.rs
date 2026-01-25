//! Layouts Demo
//!
//! Demonstrates canonical Material 3 scaffold patterns.

use bevy::prelude::*;
use bevy_material_ui::icons::icon_by_name;
use bevy_material_ui::layout::{self, PaneEntities};
use bevy_material_ui::prelude::*;
use bevy_material_ui::select::{SelectBuilder, SelectOption, SpawnSelectChild};

const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 240.0;

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
            ScrollContainerBuilder::new().vertical().build(),
            ScrollPosition::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                overflow: Overflow::scroll(),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("layouts_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Percent(100.0),
                min_height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(24.0),
                ..default()
            })
            .with_children(|page| {
                spawn_layout_section(page, &theme, "Navigation scaffolds", |section| {
                    spawn_layout_entry(section, &theme, "Navigation Bar (Bottom)", |root| {
                        spawn_bottom_navigation_scaffold(root, &theme)
                    });
                    spawn_layout_entry(section, &theme, "Navigation Rail", |root| {
                        spawn_navigation_rail_scaffold(root, &theme)
                    });
                    spawn_layout_entry(section, &theme, "Standard Drawer", |root| {
                        spawn_standard_drawer_scaffold(root, &theme)
                    });
                    spawn_layout_entry(section, &theme, "Permanent Drawer", |root| {
                        spawn_permanent_drawer_scaffold(root, &theme)
                    });
                    spawn_layout_entry(section, &theme, "Modal Drawer", |root| {
                        spawn_modal_drawer_scaffold(root, &theme)
                    });
                });

                spawn_layout_section(page, &theme, "Adaptive navigation (by window class)", |section| {
                    let phone = WindowSizeClass::new(480.0, 800.0);
                    let tablet = WindowSizeClass::new(900.0, 900.0);
                    let desktop = WindowSizeClass::new(1400.0, 900.0);

                    spawn_layout_entry(section, &theme, "Adaptive: Phone (Compact)", |root| {
                        spawn_navigation_suite_scaffold(
                            root,
                            &theme,
                            &phone,
                            "layout_adaptive_phone",
                            spawn_primary_actions,
                        )
                    });
                    spawn_layout_entry(section, &theme, "Adaptive: Tablet (Rail)", |root| {
                        spawn_navigation_suite_scaffold(
                            root,
                            &theme,
                            &tablet,
                            "layout_adaptive_tablet",
                            spawn_toggle_stack,
                        )
                    });
                    spawn_layout_entry(section, &theme, "Adaptive: Desktop (Drawer)", |root| {
                        spawn_navigation_suite_scaffold(
                            root,
                            &theme,
                            &desktop,
                            "layout_adaptive_desktop",
                            spawn_navigation_stack,
                        )
                    });
                });

                spawn_layout_section(page, &theme, "Pane scaffolds", |section| {
                    spawn_layout_entry(section, &theme, "List Detail (2 panes)", |root| {
                        spawn_list_detail_scaffold(root, &theme)
                    });
                    spawn_layout_entry(section, &theme, "Supporting Panes (3 panes)", |root| {
                        spawn_supporting_panes_scaffold(root, &theme)
                    });
                });
            });
        });
}

fn spawn_layout_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    title: &str,
    build: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            max_width: Val::Px(980.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|section| {
            section.spawn((
                Text::new(title),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            build(section);
        });
}

fn spawn_layout_entry(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    build_scaffold: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|entry| {
            entry.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            entry
                .spawn((
                    Node {
                        width: Val::Px(PREVIEW_WIDTH),
                        height: Val::Px(PREVIEW_HEIGHT),
                        border: UiRect::all(Val::Px(1.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    BorderColor::all(theme.outline_variant),
                ))
                .with_children(|root| {
                    build_scaffold(root);
                });
        });
}

fn spawn_primary_actions(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_button(theme, "Primary", ButtonVariant::Filled);
            col.spawn_checkbox(theme, CheckboxState::Checked, "Remember");
        });
}

fn spawn_toggle_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_switch(theme, true, "Notifications");
            col.spawn_checkbox(theme, CheckboxState::Unchecked, "Use cellular");
        });
}

fn spawn_input_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let options = vec![
        SelectOption::new("Option A"),
        SelectOption::new("Option B"),
        SelectOption::new("Option C"),
    ];

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_select_with(
                theme,
                SelectBuilder::new(options)
                    .label("Mode")
                    .filled()
                    .selected(0)
                    .width(Val::Px(200.0)),
            );
            col.spawn_button(theme, "Submit", ButtonVariant::Tonal);
        });
}

fn spawn_filter_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_chip_with(theme, ChipBuilder::filter("Filter A").selected(true));
            col.spawn_chip_with(theme, ChipBuilder::filter("Filter B"));
        });
}

fn spawn_status_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_button(theme, "Refresh", ButtonVariant::Outlined);
            col.spawn_switch(theme, false, "Sync");
        });
}

fn spawn_navigation_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_button(theme, "Open", ButtonVariant::Text);
            col.spawn_checkbox(theme, CheckboxState::Indeterminate, "Select all");
        });
}

trait LayoutDemoButtonExt {
    fn spawn_button(&mut self, theme: &MaterialTheme, label: &str, variant: ButtonVariant);
}

impl LayoutDemoButtonExt for ChildSpawnerCommands<'_> {
    fn spawn_button(&mut self, theme: &MaterialTheme, label: &str, variant: ButtonVariant) {
        let button = MaterialButton::new(label).with_variant(variant);
        let text_color = button.text_color(theme);

        self.spawn(MaterialButtonBuilder::new(label).with_variant(variant).build(theme))
            .with_children(|btn| {
                btn.spawn((
                    ButtonLabel,
                    Text::new(label),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(text_color),
                ));
            });
    }
}

fn spawn_bottom_navigation_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::NavigationBarScaffold::default();
    let _ = layout::spawn_navigation_bar_scaffold(
        parent,
        theme,
        &config,
        |content| {
            content
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|c| {
                    spawn_primary_actions(c, theme);
                });
        },
        |nav| {
            for (i, icon_name) in ["home", "search", "person"].iter().enumerate() {
                nav.spawn((
                    TestId::new(format!("layout_bottom_nav_{}", i)),
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    if let Some(id) = icon_by_name(icon_name) {
                        btn.spawn(
                            bevy_material_ui::icons::MaterialIcon::new(id)
                                .with_size(18.0)
                                .with_color(theme.on_surface),
                        );
                    }
                });
            }
        },
    );
}

fn spawn_navigation_rail_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::NavigationRailScaffold::default();
    let _ = layout::spawn_navigation_rail_scaffold(
        parent,
        theme,
        &config,
        |nav| {
            for (i, icon_name) in ["menu", "favorite", "more_vert"].iter().enumerate() {
                nav.spawn((
                    TestId::new(format!("layout_rail_nav_{}", i)),
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(56.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    if let Some(id) = icon_by_name(icon_name) {
                        btn.spawn(
                            bevy_material_ui::icons::MaterialIcon::new(id)
                                .with_size(20.0)
                                .with_color(theme.on_surface),
                        );
                    }
                });
            }
        },
        |content| {
            content
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|c| {
                    spawn_toggle_stack(c, theme);
                });
        },
    );
}

fn spawn_standard_drawer_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::StandardDrawerScaffold::default();
    let _ = layout::spawn_standard_drawer_scaffold(
        parent,
        theme,
        &config,
        |drawer| {
            spawn_drawer_list(drawer, theme, "layout_standard_drawer");
        },
        |content| {
            content
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|c| {
                    spawn_input_stack(c, theme);
                });
        },
    );
}

fn spawn_permanent_drawer_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::PermanentDrawerScaffold::default();
    let _ = layout::spawn_permanent_drawer_scaffold(
        parent,
        theme,
        &config,
        |drawer| {
            spawn_drawer_list(drawer, theme, "layout_permanent_drawer");
        },
        |content| {
            content
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|c| {
                    spawn_filter_stack(c, theme);
                });
        },
    );
}

fn spawn_modal_drawer_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::ModalDrawerScaffold::default();
    let _ = layout::spawn_modal_drawer_scaffold(
        parent,
        theme,
        &config,
        |drawer| {
            spawn_drawer_list(drawer, theme, "layout_modal_drawer");
        },
        |content| {
            content
                .spawn(Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|c| {
                    spawn_status_stack(c, theme);
                });
        },
    );
}

fn spawn_navigation_suite_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    size_class: &WindowSizeClass,
    test_prefix: &str,
    content_builder: fn(&mut ChildSpawnerCommands, &MaterialTheme),
) {
    let config = layout::NavigationSuiteScaffold::default();
    let _ = layout::spawn_navigation_suite_scaffold(
        parent,
        theme,
        size_class,
        &config,
        |nav| {
            for (i, icon_name) in ["home", "search", "person"].iter().enumerate() {
                nav.spawn((
                    TestId::new(format!("{}_nav_{}", test_prefix, i)),
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|btn| {
                    if let Some(id) = icon_by_name(icon_name) {
                        btn.spawn(
                            bevy_material_ui::icons::MaterialIcon::new(id)
                                .with_size(18.0)
                                .with_color(theme.on_surface),
                        );
                    }
                });
            }
        },
        |content| {
            content
                .spawn((
                    TestId::new(format!("{}_content", test_prefix)),
                    Node {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Stretch,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                ))
                .with_children(|c| {
                    content_builder(c, theme);
                });
        },
    );
}

fn spawn_list_detail_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::ListDetailScaffold::default();
    let _panes: PaneEntities = layout::spawn_list_detail_scaffold(
        parent,
        theme,
        &config,
        |primary| {
            primary
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|col| {
                    for label in ["Drafts", "Sent", "Archive"].iter() {
                        col.spawn((
                            Button,
                            Interaction::None,
                            RippleHost::new(),
                            Node {
                                height: Val::Px(36.0),
                                align_items: AlignItems::Center,
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container_high),
                            BorderRadius::all(Val::Px(6.0)),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new((*label).to_string()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                    }
                });
        },
        |secondary| {
            secondary
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_navigation_stack(col, theme);
                });
        },
    );
}

fn spawn_supporting_panes_scaffold(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::SupportingPanesScaffold::default();
    let _panes: PaneEntities = layout::spawn_supporting_panes_scaffold(
        parent,
        theme,
        &config,
        |primary| {
            primary
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_input_stack(col, theme);
                });
        },
        |secondary| {
            secondary
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_status_stack(col, theme);
                });
        },
        |supporting| {
            supporting
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .with_children(|col| {
                    spawn_filter_stack(col, theme);
                });
        },
    );
}

fn spawn_drawer_list(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    test_prefix: &str,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            TestId::new(format!("{}_list", test_prefix)),
        ))
        .with_children(|list| {
            for (i, label) in ["Inbox", "Starred", "Archive"].iter().enumerate() {
                list.spawn((
                    TestId::new(format!("{}_item_{}", test_prefix, i)),
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        height: Val::Px(36.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_high),
                    BorderRadius::all(Val::Px(6.0)),
                ))
                .with_children(|item| {
                    item.spawn((
                        Text::new((*label).to_string()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));
                });
            }
        });
}
