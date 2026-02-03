//! Layout scaffolds showcase demonstrating navigation and pane patterns.

use bevy::prelude::*;
use bevy_material_ui::icons::icon_by_name;
use bevy_material_ui::layout::{self, PaneEntities};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 240.0;

pub fn spawn_layouts_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.layouts.title",
                "Layouts",
                "showcase.section.layouts.description",
                "Canonical Material 3 scaffolds: navigation bar, rail, standard/modal/permanent drawers, list-detail, and supporting panes.",
            );

            spawn_navigation_examples(section, theme.clone(), icon_font.clone());
            spawn_adaptive_examples(section, theme.clone(), icon_font.clone());
            spawn_panes_examples(section, theme.clone());

            spawn_code_block(
                section,
                theme,
                include_str!("../../layouts_demo.rs"),
            );
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
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|section| {
            section.spawn((
                Text::new(title),
                TextFont {
                    font_size: FontSize::Px(16.0),
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
                    font_size: FontSize::Px(13.0),
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

fn spawn_navigation_examples(
    parent: &mut ChildSpawnerCommands,
    theme: MaterialTheme,
    _icon_font: Handle<Font>,
) {
    spawn_layout_section(parent, &theme, "Navigation scaffolds", |section| {
        spawn_layout_entry(section, &theme, "Navigation Bar (Bottom)", |root| {
            spawn_bottom_nav_card(root, &theme, _icon_font.clone());
        });
        spawn_layout_entry(section, &theme, "Navigation Rail", |root| {
            spawn_nav_rail_card(root, &theme, _icon_font.clone());
        });
        spawn_layout_entry(section, &theme, "Standard Drawer", |root| {
            spawn_standard_drawer_card(root, &theme);
        });
        spawn_layout_entry(section, &theme, "Permanent Drawer", |root| {
            spawn_permanent_drawer_card(root, &theme);
        });
        spawn_layout_entry(section, &theme, "Modal Drawer", |root| {
            spawn_modal_drawer_example(root, theme.clone());
        });
    });
}

fn spawn_adaptive_examples(
    parent: &mut ChildSpawnerCommands,
    theme: MaterialTheme,
    _icon_font: Handle<Font>,
) {
    spawn_layout_section(parent, &theme, "Adaptive navigation (by window class)", |section| {
        let phone = WindowSizeClass::new(480.0, 800.0);
        let tablet = WindowSizeClass::new(900.0, 900.0);
        let desktop = WindowSizeClass::new(1400.0, 900.0);

        spawn_layout_entry(section, &theme, "Adaptive: Phone (Compact)", |root| {
            spawn_adaptive_card(
                root,
                &theme,
                _icon_font.clone(),
                phone,
                "layout_adaptive_phone",
                spawn_primary_actions,
            );
        });
        spawn_layout_entry(section, &theme, "Adaptive: Tablet (Rail)", |root| {
            spawn_adaptive_card(
                root,
                &theme,
                _icon_font.clone(),
                tablet,
                "layout_adaptive_tablet",
                spawn_toggle_stack,
            );
        });
        spawn_layout_entry(section, &theme, "Adaptive: Desktop (Drawer)", |root| {
            spawn_adaptive_card(
                root,
                &theme,
                _icon_font.clone(),
                desktop,
                "layout_adaptive_desktop",
                spawn_navigation_stack,
            );
        });
    });
}

fn spawn_adaptive_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
    size_class: WindowSizeClass,
    test_prefix: &str,
    content_builder: fn(&mut ChildSpawnerCommands, &MaterialTheme),
) {
    let config = layout::NavigationSuiteScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(PREVIEW_WIDTH),
            height: Val::Px(PREVIEW_HEIGHT),
            ..default()
        })
        .with_children(|root| {
            let _entities = layout::spawn_navigation_suite_scaffold(
                root,
                theme,
                &size_class,
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
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                        ))
                        .with_children(|c| {
                            content_builder(c, theme);
                        });
                },
            );
        });
}

fn spawn_bottom_nav_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
) {
    let config = layout::NavigationBarScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(PREVIEW_WIDTH),
            height: Val::Px(PREVIEW_HEIGHT),
            ..default()
        })
        .with_children(|card| {
            let _entities = layout::spawn_navigation_bar_scaffold(
                card,
                theme,
                &config,
                |content| {
                    content
                        .spawn((
                            TestId::new("layout_bottom_content"),
                            Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Stretch,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                        ))
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
        });
}

fn spawn_nav_rail_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
) {
    let config = layout::NavigationRailScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(PREVIEW_WIDTH),
            height: Val::Px(PREVIEW_HEIGHT),
            ..default()
        })
        .with_children(|root| {
            let _entities = layout::spawn_navigation_rail_scaffold(
                root,
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
                        .spawn((
                            TestId::new("layout_rail_content"),
                            Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Stretch,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                        ))
                        .with_children(|c| {
                            spawn_toggle_stack(c, theme);
                        });
                },
            );
        });
}

fn spawn_standard_drawer_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::StandardDrawerScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(PREVIEW_WIDTH),
            height: Val::Px(PREVIEW_HEIGHT),
            ..default()
        })
        .with_children(|root| {
            let _entities = layout::spawn_standard_drawer_scaffold(
                root,
                theme,
                &config,
                |drawer| {
                    drawer
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|list| {
                            for label in ["Inbox", "Starred", "Archive"].iter() {
                                list.spawn((
                                    Button,
                                    Interaction::None,
                                    RippleHost::new(),
                                    Node {
                                        height: Val::Px(36.0),
                                        align_items: AlignItems::Center,
                                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                        border_radius: BorderRadius::all(Val::Px(6.0)),
                                        ..default()
                                    },
                                    BackgroundColor(theme.surface_container_high),
                                ))
                                .with_children(|item| {
                                    item.spawn((
                                        Text::new((*label).to_string()),
                                        TextFont {
                                            font_size: FontSize::Px(13.0),
                                            ..default()
                                        },
                                        TextColor(theme.on_surface),
                                    ));
                                });
                            }
                        });
                },
                |content| {
                    content
                        .spawn(Node {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Stretch,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|c| {
                            spawn_input_stack(c, theme);
                        });
                },
            );

            let _ = _entities;
        });
}

fn spawn_permanent_drawer_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::PermanentDrawerScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(PREVIEW_WIDTH),
            height: Val::Px(PREVIEW_HEIGHT),
            ..default()
        })
        .with_children(|root| {
            let _entities = layout::spawn_permanent_drawer_scaffold(
                root,
                theme,
                &config,
                |drawer| {
                    drawer
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|list| {
                            for label in ["Home", "Bookmarks", "Settings"].iter() {
                                list.spawn((
                                    Button,
                                    Interaction::None,
                                    RippleHost::new(),
                                    Node {
                                        height: Val::Px(36.0),
                                        align_items: AlignItems::Center,
                                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                        border_radius: BorderRadius::all(Val::Px(6.0)),
                                        ..default()
                                    },
                                    BackgroundColor(theme.surface_container_high),
                                ))
                                .with_children(|item| {
                                    item.spawn((
                                        Text::new((*label).to_string()),
                                        TextFont {
                                            font_size: FontSize::Px(13.0),
                                            ..default()
                                        },
                                        TextColor(theme.on_surface),
                                    ));
                                });
                            }
                        });
                },
                |content| {
                    content
                        .spawn(Node {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Stretch,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|c| {
                            spawn_filter_stack(c, theme);
                        });
                },
            );

            let _ = _entities;
        });
}

fn spawn_modal_drawer_example(parent: &mut ChildSpawnerCommands, theme: MaterialTheme) {
    let config = layout::ModalDrawerScaffold::default();
    let _entities = layout::spawn_modal_drawer_scaffold(
        parent,
        &theme,
        &config,
        |drawer| {
            drawer
                .spawn((
                    TestId::new("layout_drawer_list"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|list| {
                    for (i, label) in ["Inbox", "Starred", "Archive"].iter().enumerate() {
                        list.spawn((
                            TestId::new(format!("layout_drawer_item_{}", i)),
                            Button,
                            Interaction::None,
                            RippleHost::new(),
                            Node {
                                height: Val::Px(40.0),
                                align_items: AlignItems::Center,
                                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container_high),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new((*label).to_string()),
                                TextFont {
                                    font_size: FontSize::Px(14.0),
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                    }
                });
        },
        |content| {
            content
                .spawn((
                    TestId::new("layout_drawer_content"),
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
                    spawn_status_stack(c, &theme);
                });
        },
    );

    let _ = _entities;
}

fn spawn_panes_examples(parent: &mut ChildSpawnerCommands, theme: MaterialTheme) {
    spawn_layout_section(parent, &theme, "Pane scaffolds", |section| {
        spawn_layout_entry(section, &theme, "List-detail", |root| {
            spawn_list_detail_card(root, &theme);
        });
        spawn_layout_entry(section, &theme, "Supporting panes", |root| {
            spawn_supporting_panes_card(root, &theme);
        });
    });
}

fn spawn_list_detail_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::ListDetailScaffold::default();
    let _entities: PaneEntities = layout::spawn_list_detail_scaffold(
        parent,
        theme,
        &config,
        |primary| {
            primary
                .spawn((
                    TestId::new("layout_list_primary"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                ))
                .with_children(|list| {
                    for (i, label) in ["Email A", "Email B", "Email C"].iter().enumerate() {
                        list.spawn((
                            TestId::new(format!("layout_list_item_{}", i)),
                            Node {
                                height: Val::Px(32.0),
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container_high),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new((*label).to_string()),
                                TextFont {
                                    font_size: FontSize::Px(13.0),
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
                .spawn((
                    TestId::new("layout_list_detail"),
                    Node {
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                ))
                .with_children(|detail| {
                    spawn_navigation_stack(detail, theme);
                });
        },
    );

    let _ = _entities;
}

fn spawn_supporting_panes_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::SupportingPanesScaffold::default();
    let _entities: PaneEntities = layout::spawn_supporting_panes_scaffold(
        parent,
        theme,
        &config,
        |primary| {
            primary
                .spawn((
                    TestId::new("layout_support_primary"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                ))
                .with_children(|list| {
                    for (i, label) in ["Thread A", "Thread B", "Thread C"].iter().enumerate() {
                        list.spawn((
                            TestId::new(format!("layout_support_item_{}", i)),
                            Node {
                                height: Val::Px(32.0),
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container_high),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new((*label).to_string()),
                                TextFont {
                                    font_size: FontSize::Px(13.0),
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
                .spawn((
                    TestId::new("layout_support_secondary"),
                    Node {
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                ))
                .with_children(|detail| {
                    spawn_input_stack(detail, theme);
                });
        },
        |supporting| {
            supporting
                .spawn((
                    TestId::new("layout_supporting"),
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|support| {
                    spawn_filter_stack(support, theme);
                });
        },
    );

    let _ = _entities;
}

// ---------------------------------------------------------------------------
// Content stack helpers
// ---------------------------------------------------------------------------

fn spawn_primary_actions(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_filled_button(theme, "Save");
            col.spawn_outlined_button(theme, "Cancel");
            col.spawn_text_button(theme, "More");
        });
}

fn spawn_toggle_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_checkbox(theme, CheckboxState::Checked, "Checkbox");
            col.spawn_switch(theme, true, "Switch");
        });
}

fn spawn_input_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_filled_text_field(theme, "Username", "");
            col.spawn_filled_text_field(theme, "Password", "");
        });
}

fn spawn_filter_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        })
        .with_children(|row| {
            row.spawn_filter_chip(theme, "Filter A", false);
            row.spawn_filter_chip(theme, "Filter B", true);
            row.spawn_filter_chip(theme, "Filter C", false);
        });
}

fn spawn_status_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Status: OK"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
            col.spawn_filled_button(theme, "Refresh");
        });
}

fn spawn_navigation_stack(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn_text_button(theme, "Home");
            col.spawn_text_button(theme, "Settings");
            col.spawn_text_button(theme, "Help");
        });
}
