//! List Demo
//!
//! Demonstrates Material Design 3 lists with various item configurations.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::icons::ICON_EMAIL;
use bevy_material_ui::prelude::*;

#[derive(Resource, Debug, Clone)]
struct ListsDemoState {
    virtualize_large_list: bool,
    selection_mode: ListSelectionMode,
}

impl Default for ListsDemoState {
    fn default() -> Self {
        Self {
            virtualize_large_list: true,
            selection_mode: ListSelectionMode::Single,
        }
    }
}

#[derive(Component)]
struct ListDemoRoot;

#[derive(Component)]
struct ListSelectionModeChip(ListSelectionMode);

#[derive(Component)]
struct ListVirtualizeToggle;

#[derive(Component)]
struct ListVirtualDemoHost;

#[derive(Component)]
struct ListVirtualizeLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_selection_mode_chips, handle_list_virtualize_toggle),
        )
        .init_resource::<ListsDemoState>()
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    language: Option<Res<MaterialLanguage>>,
    i18n: Option<Res<MaterialI18n>>,
    state: Res<ListsDemoState>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("list_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(420.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                BackgroundColor(theme.surface),
            ))
            .insert_test_id("list_demo/panel", &telemetry)
            .with_children(|panel| {
                let language_tag = language.as_ref().map(|l| l.tag.as_str()).unwrap_or("en-US");

                // Selection mode options
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Selection Mode:"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                            Node {
                                margin: UiRect::right(Val::Px(8.0)),
                                ..default()
                            },
                        ));

                        for (label, mode) in [
                            ("Single", ListSelectionMode::Single),
                            ("Multi", ListSelectionMode::Multi),
                        ] {
                            let is_selected = state.selection_mode == mode;
                            let chip_for_color =
                                MaterialChip::filter(label).with_selected(is_selected);
                            let label_color = chip_for_color.label_color(&theme);

                            row.spawn((
                                ListSelectionModeChip(mode),
                                Interaction::None,
                                ChipBuilder::filter(label)
                                    .selected(is_selected)
                                    .build(&theme),
                            ))
                            .with_children(|chip| {
                                chip.spawn((
                                    ChipLabel,
                                    Text::new(label),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(label_color),
                                ));
                            });
                        }
                    });

                // Small scrollable list
                panel
                    .spawn(Node {
                        width: Val::Px(420.0),
                        max_width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|container| {
                        container
                            .spawn((
                                ListDemoRoot,
                                ListBuilder::new()
                                    .max_visible_items_variant(4, ListItemVariant::TwoLine)
                                    .selection_mode(state.selection_mode)
                                    .build_scrollable(),
                                BackgroundColor(theme.surface),
                                BorderRadius::all(Val::Px(12.0)),
                                Interaction::None,
                            ))
                            .insert_test_id("list_demo/list", &telemetry)
                            .with_children(|list| {
                                let items = [
                                    (
                                        "list_demo.item_1.headline",
                                        "Inbox",
                                        "list_demo.item_1.supporting",
                                        "Primary inbox for emails",
                                    ),
                                    (
                                        "list_demo.item_2.headline",
                                        "Starred",
                                        "list_demo.item_2.supporting",
                                        "Important messages",
                                    ),
                                    (
                                        "list_demo.item_4.headline",
                                        "Sent",
                                        "list_demo.item_4.supporting",
                                        "Outgoing messages",
                                    ),
                                    (
                                        "list_demo.item_3.headline",
                                        "Drafts",
                                        "list_demo.item_3.supporting",
                                        "Unfinished messages",
                                    ),
                                    (
                                        "list_demo.item_6.headline",
                                        "Spam",
                                        "list_demo.item_6.supporting",
                                        "Filtered junk mail",
                                    ),
                                    (
                                        "list_demo.item_5.headline",
                                        "Trash",
                                        "list_demo.item_5.supporting",
                                        "Deleted items",
                                    ),
                                    (
                                        "list_demo.item_17.headline",
                                        "Archive",
                                        "list_demo.item_17.supporting",
                                        "Stored messages",
                                    ),
                                    (
                                        "list_demo.item_19.headline",
                                        "Labels",
                                        "list_demo.item_19.supporting",
                                        "Organized categories",
                                    ),
                                    (
                                        "list_demo.item_20.headline",
                                        "Settings",
                                        "list_demo.item_20.supporting",
                                        "Configuration options",
                                    ),
                                    (
                                        "list_demo.item_7.headline",
                                        "Help",
                                        "list_demo.item_7.supporting",
                                        "Support and documentation",
                                    ),
                                ];

                                for (
                                    headline_key,
                                    headline_default,
                                    supporting_key,
                                    supporting_default,
                                ) in items
                                {
                                    let headline = i18n
                                        .as_ref()
                                        .and_then(|i18n| i18n.translate(language_tag, headline_key))
                                        .map(str::to_string)
                                        .unwrap_or_else(|| headline_default.to_string());

                                    let supporting = i18n
                                        .as_ref()
                                        .and_then(|i18n| {
                                            i18n.translate(language_tag, supporting_key)
                                        })
                                        .map(str::to_string)
                                        .unwrap_or_else(|| supporting_default.to_string());

                                    list.spawn((
                                        ListItemBuilder::new(headline)
                                            .two_line()
                                            .supporting_text(supporting)
                                            .leading_icon(ICON_EMAIL)
                                            .build(&theme),
                                        Interaction::None,
                                    ));
                                }
                            });

                        // Virtualized list demo
                        container.spawn((
                            Text::new("Virtualized list (500 items)"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                            Node {
                                margin: UiRect::top(Val::Px(16.0)),
                                ..default()
                            },
                        ));

                        let switch = MaterialSwitch::new().selected(state.virtualize_large_list);
                        let switch_track_color = switch.track_color(&theme);
                        let switch_outline_color = switch.track_outline_color(&theme);
                        let switch_handle_color = switch.handle_color(&theme);
                        let switch_handle_size = switch.handle_size();
                        let switch_label = if state.virtualize_large_list {
                            "Virtualize: ON"
                        } else {
                            "Virtualize: OFF"
                        };

                        container
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(12.0),
                                ..default()
                            })
                            .with_children(|row| {
                                row.spawn((
                                    ListVirtualizeToggle,
                                    switch,
                                    Button,
                                    Interaction::None,
                                    RippleHost::new(),
                                    Node {
                                        width: Val::Px(
                                            bevy_material_ui::switch::SWITCH_TRACK_WIDTH,
                                        ),
                                        height: Val::Px(
                                            bevy_material_ui::switch::SWITCH_TRACK_HEIGHT,
                                        ),
                                        justify_content: if state.virtualize_large_list {
                                            JustifyContent::FlexEnd
                                        } else {
                                            JustifyContent::FlexStart
                                        },
                                        align_items: AlignItems::Center,
                                        padding: UiRect::horizontal(Val::Px(2.0)),
                                        border: UiRect::all(Val::Px(
                                            if state.virtualize_large_list {
                                                0.0
                                            } else {
                                                2.0
                                            },
                                        )),
                                        ..default()
                                    },
                                    BackgroundColor(switch_track_color),
                                    BorderColor::all(switch_outline_color),
                                    BorderRadius::all(Val::Px(16.0)),
                                ))
                                .with_children(|track| {
                                    let handle_size = switch_handle_size;
                                    track.spawn((
                                        SwitchHandle,
                                        Node {
                                            width: Val::Px(handle_size),
                                            height: Val::Px(handle_size),
                                            ..default()
                                        },
                                        BackgroundColor(switch_handle_color),
                                        BorderRadius::all(Val::Px(handle_size / 2.0)),
                                    ));
                                });

                                row.spawn((
                                    ListVirtualizeLabel,
                                    Text::new(switch_label),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                            });

                        container
                            .spawn((
                                ListVirtualDemoHost,
                                Node {
                                    width: Val::Px(420.0),
                                    max_width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                            ))
                            .with_children(|host| {
                                spawn_large_list_demo(host, &theme, state.virtualize_large_list);
                            });
                    });

                // Scrollbar orientation demos (horizontal + both)
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        margin: UiRect::top(Val::Px(16.0)),
                        ..default()
                    })
                    .with_children(|demo| {
                        demo.spawn((
                            Text::new("Scrollbar orientations"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));

                        demo.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(12.0),
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                ScrollContainerBuilder::new().horizontal().build(),
                                ScrollPosition::default(),
                                Node {
                                    width: Val::Px(400.0),
                                    height: Val::Px(120.0),
                                    overflow: Overflow::scroll(),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(12.0),
                                    ..default()
                                },
                                BackgroundColor(theme.surface_container_low),
                                BorderRadius::all(Val::Px(12.0)),
                                Interaction::None,
                            ))
                            .with_children(|scroller| {
                                for i in 1..=18 {
                                    scroller.spawn((
                                        Node {
                                            width: Val::Px(84.0),
                                            height: Val::Px(72.0),
                                            ..default()
                                        },
                                        BackgroundColor(if i % 2 == 0 {
                                            theme.secondary_container
                                        } else {
                                            theme.primary_container
                                        }),
                                        BorderRadius::all(Val::Px(12.0)),
                                    ));
                                }
                            });

                            row.spawn((
                                ScrollContainerBuilder::new().both().build(),
                                ScrollPosition::default(),
                                Node {
                                    width: Val::Px(400.0),
                                    height: Val::Px(180.0),
                                    overflow: Overflow::scroll(),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    ..default()
                                },
                                BackgroundColor(theme.surface_container_low),
                                BorderRadius::all(Val::Px(12.0)),
                                Interaction::None,
                            ))
                            .with_children(|scroller| {
                                scroller
                                    .spawn(Node {
                                        width: Val::Px(760.0),
                                        height: Val::Px(380.0),
                                        flex_direction: FlexDirection::Row,
                                        flex_wrap: FlexWrap::Wrap,
                                        row_gap: Val::Px(12.0),
                                        column_gap: Val::Px(12.0),
                                        ..default()
                                    })
                                    .with_children(|content| {
                                        for i in 1..=30 {
                                            content.spawn((
                                                Node {
                                                    width: Val::Px(120.0),
                                                    height: Val::Px(72.0),
                                                    ..default()
                                                },
                                                BackgroundColor(if i % 3 == 0 {
                                                    theme.tertiary_container
                                                } else if i % 2 == 0 {
                                                    theme.secondary_container
                                                } else {
                                                    theme.primary_container
                                                }),
                                                BorderRadius::all(Val::Px(12.0)),
                                            ));
                                        }
                                    });
                            });
                        });
                    });
            });
        });
}

fn spawn_large_list_demo(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    virtualize: bool,
) {
    let mut items: Vec<ListItemBuilder> = Vec::new();
    for i in 1..=500 {
        let builder = if i % 3 == 0 {
            ListItemBuilder::new(format!("Item {i}"))
                .two_line()
                .supporting_text("Supporting text")
                .leading_icon(ICON_EMAIL)
        } else {
            ListItemBuilder::new(format!("Item {i}"))
                .one_line()
                .leading_icon(ICON_EMAIL)
        };
        items.push(builder);
    }

    if virtualize {
        parent.spawn((
            ListBuilder::new()
                .max_visible_items_variant(6, ListItemVariant::TwoLine)
                .selection_mode(ListSelectionMode::Single)
                .items_from_builders(items)
                .virtualize(true)
                .overscan_rows(3)
                .build_scrollable(),
            BackgroundColor(theme.surface),
            BorderRadius::all(Val::Px(12.0)),
            Interaction::None,
        ));
        return;
    }

    parent
        .spawn((
            ListBuilder::new()
                .max_visible_items_variant(6, ListItemVariant::TwoLine)
                .selection_mode(ListSelectionMode::Single)
                .build_scrollable(),
            BackgroundColor(theme.surface),
            BorderRadius::all(Val::Px(12.0)),
            Interaction::None,
        ))
        .with_children(|list| {
            for builder in items {
                list.spawn((builder.build(theme), Interaction::None));
            }
        });
}

fn handle_selection_mode_chips(
    mut state: ResMut<ListsDemoState>,
    mut clicks: Query<
        (&Interaction, &ListSelectionModeChip),
        (Changed<Interaction>, With<MaterialChip>),
    >,
    mut chips: Query<(&ListSelectionModeChip, &mut MaterialChip)>,
    mut lists: Query<&mut MaterialList, With<ListDemoRoot>>,
) {
    let mut new_mode = None;
    for (interaction, chip) in clicks.iter_mut() {
        if *interaction == Interaction::Pressed {
            new_mode = Some(chip.0);
        }
    }

    let Some(new_mode) = new_mode else {
        return;
    };

    if state.selection_mode == new_mode {
        return;
    }
    state.selection_mode = new_mode;

    for mut list in lists.iter_mut() {
        list.selection_mode = new_mode;
    }

    for (mode, mut chip) in chips.iter_mut() {
        chip.selected = mode.0 == new_mode;
    }
}

fn handle_list_virtualize_toggle(
    mut commands: Commands,
    mut state: ResMut<ListsDemoState>,
    mut switch_events: MessageReader<bevy_material_ui::switch::SwitchChangeEvent>,
    toggles: Query<(), With<ListVirtualizeToggle>>,
    hosts: Query<Entity, With<ListVirtualDemoHost>>,
    children_q: Query<&Children>,
    mut labels: Query<&mut Text, With<ListVirtualizeLabel>>,
    theme: Res<MaterialTheme>,
) {
    let mut should_update = None;
    for event in switch_events.read() {
        if toggles.get(event.entity).is_ok() {
            should_update = Some(event.selected);
        }
    }

    let Some(new_value) = should_update else {
        return;
    };

    if state.virtualize_large_list == new_value {
        return;
    }
    state.virtualize_large_list = new_value;

    let label_text = if new_value {
        "Virtualize: ON"
    } else {
        "Virtualize: OFF"
    };
    for mut text in labels.iter_mut() {
        *text = Text::new(label_text);
    }

    for host in hosts.iter() {
        clear_children_recursive_local(&mut commands, &children_q, host);
        commands.entity(host).with_children(|parent| {
            spawn_large_list_demo(parent, &theme, new_value);
        });
    }
}

fn clear_children_recursive_local(
    commands: &mut Commands,
    children_q: &Query<&Children>,
    entity: Entity,
) {
    let Ok(children) = children_q.get(entity) else {
        return;
    };

    for child in children.iter() {
        clear_children_recursive_local(commands, children_q, child);
        commands.entity(child).despawn();
    }
}
