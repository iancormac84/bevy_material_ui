//! Lists view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::icons::ICON_EMAIL;
use bevy_material_ui::list::ListItemBuilder;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

#[derive(Resource, Debug, Clone)]
pub struct ListsViewState {
    pub virtualize_large_list: bool,
}

impl Default for ListsViewState {
    fn default() -> Self {
        Self {
            virtualize_large_list: true,
        }
    }
}

#[derive(Component)]
pub(crate) struct ListVirtualizeToggle;

#[derive(Component)]
pub(crate) struct ListVirtualDemoHost;

/// Spawn the list section content
pub fn spawn_list_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
    state: &ListsViewState,
) {
    let theme_clone = theme.clone();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                &theme_clone,
                "showcase.section.lists.title",
                "Lists (with Selection)",
                "showcase.section.lists.description",
                "Scrollable list with single or multi-select - click items to select",
            );

            // Selection mode options
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.lists.selection_mode")
                            .with_default("Selection Mode:"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme_clone.on_surface),
                        Node {
                            margin: UiRect::right(Val::Px(8.0)),
                            ..default()
                        },
                        NeedsInternationalFont,
                    ));
                    spawn_list_mode_option(
                        row,
                        &theme_clone,
                        "Single",
                        ListSelectionMode::Single,
                        true,
                    );
                    spawn_list_mode_option(
                        row,
                        &theme_clone,
                        "Multi",
                        ListSelectionMode::Multi,
                        false,
                    );
                });
            // Container for list with scrollbar
            section
                .spawn(Node {
                    width: Val::Px(420.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|container| {
                    // Scrollable list using the new API
                    let scroll_area_id = container
                        .spawn((
                            ListDemoRoot,
                            TestId::new("list_scroll_area"),
                            bevy_material_ui::list::ListBuilder::new()
                                .max_visible_items_variant(
                                    4,
                                    bevy_material_ui::list::ListItemVariant::TwoLine,
                                )
                                .selection_mode(ListSelectionMode::Single)
                                .build_scrollable(),
                            BackgroundColor(theme_clone.surface),
                            BorderRadius::all(Val::Px(12.0)),
                            Interaction::None, // Enable hover detection
                        ))
                        .with_children(|list| {
                            // 10 list items with translation keys
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
                                i,
                                (
                                    headline_key,
                                    headline_default,
                                    supporting_key,
                                    supporting_default,
                                ),
                            ) in items.iter().enumerate()
                            {
                                list.spawn((
                                    SelectableListItem,
                                    TestId::new(format!("list_item_{}", i)),
                                    ListItemBuilder::new(*headline_default)
                                        .two_line()
                                        .supporting_text(*supporting_default)
                                        .build(&theme_clone),
                                    Interaction::None,
                                ))
                                .with_children(|item| {
                                    // Leading (match library default list item layout)
                                    item.spawn((
                                        bevy_material_ui::list::ListItemLeading,
                                        Node {
                                            width: Val::Px(56.0),
                                            height: Val::Px(56.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                    ))
                                    .with_children(
                                        |leading| {
                                            if let Some(icon) = MaterialIcon::from_name(ICON_EMAIL)
                                            {
                                                leading
                                                    .spawn(icon.with_size(24.0).with_color(
                                                        theme_clone.on_surface_variant,
                                                    ));
                                            }
                                        },
                                    );

                                    // Body (match library markers so styling/selection systems can work)
                                    item.spawn((
                                        bevy_material_ui::list::ListItemBody,
                                        Node {
                                            flex_direction: FlexDirection::Column,
                                            flex_grow: 1.0,
                                            ..default()
                                        },
                                    ))
                                    .with_children(|body| {
                                        body.spawn((
                                            bevy_material_ui::list::ListItemHeadline,
                                            Text::new(""),
                                            LocalizedText::new(*headline_key)
                                                .with_default(*headline_default),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(theme_clone.on_surface),
                                            NeedsInternationalFont,
                                        ));
                                        body.spawn((
                                            bevy_material_ui::list::ListItemSupportingText,
                                            Text::new(""),
                                            LocalizedText::new(*supporting_key)
                                                .with_default(*supporting_default),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(theme_clone.on_surface_variant),
                                            NeedsInternationalFont,
                                        ));
                                    });
                                });
                            }
                            // Note: Scrollbars spawn automatically via ScrollPlugin's ensure_scrollbars_system
                            // because ScrollContainerBuilder defaults to show_scrollbars=true.
                            // No manual spawn_scrollbars() call needed!
                        })
                        .id();

                    // Keep the entity id around for future selection/scroll interactions.
                    let _ = scroll_area_id;

                    // Virtualized list demo (large item count) to showcase performance.
                    container.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.lists.virtualized_demo")
                            .with_default("Virtualized list (500 items)"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme_clone.on_surface),
                        Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },
                        NeedsInternationalFont,
                    ));

                    // Runtime toggle for virtualization.
                    let switch = MaterialSwitch::new().selected(state.virtualize_large_list);
                    let switch_track_color = switch.track_color(&theme_clone);
                    let switch_outline_color = switch.track_outline_color(&theme_clone);
                    let switch_handle_color = switch.handle_color(&theme_clone);
                    let switch_handle_size = switch.handle_size();
                    let switch_label = if state.virtualize_large_list {
                        "Virtualize: ON"
                    } else {
                        "Virtualize: OFF"
                    };
                    let label_color = theme_clone.on_surface;

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
                                TestId::new("list_virtualize_toggle"),
                                switch,
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    width: Val::Px(bevy_material_ui::switch::SWITCH_TRACK_WIDTH),
                                    height: Val::Px(bevy_material_ui::switch::SWITCH_TRACK_HEIGHT),
                                    justify_content: if state.virtualize_large_list {
                                        JustifyContent::FlexEnd
                                    } else {
                                        JustifyContent::FlexStart
                                    },
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(Val::Px(2.0)),
                                    border: UiRect::all(Val::Px(if state.virtualize_large_list {
                                        0.0
                                    } else {
                                        2.0
                                    })),
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
                                Text::new(switch_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(label_color),
                                NeedsInternationalFont,
                            ));
                        });

                    // Host used so we can rebuild the large list on toggle.
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
                            spawn_large_list_demo(host, &theme_clone, state.virtualize_large_list);
                        });
                });

            // Explicit scrollbar orientation demos (vertical/horizontal/both)
            section
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
                        TextColor(theme_clone.on_surface),
                    ));

                    demo.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    })
                    .with_children(|row| {
                        // Horizontal scrollbar
                        row.spawn((
                            TestId::new("scroll_demo_horizontal"),
                            ScrollContainerBuilder::new().horizontal().build(),
                            ScrollPosition::default(),
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Px(120.0),
                                // Both axes must be Scroll (direction controlled by ScrollContainer)
                                overflow: Overflow::scroll(),
                                padding: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(12.0),
                                ..default()
                            },
                            BackgroundColor(theme_clone.surface_container_low),
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
                                        theme_clone.secondary_container
                                    } else {
                                        theme_clone.primary_container
                                    }),
                                    BorderRadius::all(Val::Px(12.0)),
                                ));
                            }
                            // Scrollbars spawn automatically (show_scrollbars=true by default)
                        });

                        // Both directions
                        row.spawn((
                            TestId::new("scroll_demo_both"),
                            ScrollContainerBuilder::new().both().build(),
                            ScrollPosition::default(),
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Px(180.0),
                                // Both axes must be Scroll
                                overflow: Overflow::scroll(),
                                padding: UiRect::all(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(theme_clone.surface_container_low),
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
                                                theme_clone.tertiary_container
                                            } else if i % 2 == 0 {
                                                theme_clone.secondary_container
                                            } else {
                                                theme_clone.primary_container
                                            }),
                                            BorderRadius::all(Val::Px(12.0)),
                                        ));
                                    }
                                });
                            // Scrollbars spawn automatically
                        });
                    });
                });

            spawn_code_block(
                section,
                &theme_clone,
                include_str!("../../list_demo.rs"),
            );
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
            ListItemBuilder::new(format!("Item {i}")).one_line().leading_icon(ICON_EMAIL)
        };
        items.push(builder);
    }

    if virtualize {
        parent.spawn((
            TestId::new("list_virtual_scroll_area"),
            bevy_material_ui::list::ListBuilder::new()
                .max_visible_items_variant(6, bevy_material_ui::list::ListItemVariant::TwoLine)
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

    // Non-virtualized: spawn actual child entities for all items.
    parent
        .spawn((
            TestId::new("list_virtual_scroll_area"),
            bevy_material_ui::list::ListBuilder::new()
                .max_visible_items_variant(6, bevy_material_ui::list::ListItemVariant::TwoLine)
                .selection_mode(ListSelectionMode::Single)
                .build_scrollable(),
            BackgroundColor(theme.surface),
            BorderRadius::all(Val::Px(12.0)),
            Interaction::None,
        ))
        .with_children(|list| {
            for builder in items {
                list.spawn((
                    SelectableListItem,
                    builder.build(theme),
                    Interaction::None,
                ));
            }
        });
}

pub(crate) fn handle_list_virtualize_toggle(
    mut commands: Commands,
    mut state: ResMut<ListsViewState>,
    mut switch_events: MessageReader<bevy_material_ui::switch::SwitchChangeEvent>,
    toggles: Query<(), With<ListVirtualizeToggle>>,
    hosts: Query<Entity, With<ListVirtualDemoHost>>,
    children_q: Query<&Children>,
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

    for host in hosts.iter() {
        clear_children_recursive_local(&mut commands, &children_q, host);
        commands.entity(host).with_children(|parent| {
            spawn_large_list_demo(parent, &theme, new_value);
        });
    }
}

fn clear_children_recursive_local(commands: &mut Commands, children_q: &Query<&Children>, entity: Entity) {
    let Ok(children) = children_q.get(entity) else {
        return;
    };

    for child in children.iter() {
        clear_children_recursive_local(commands, children_q, child);
        commands.entity(child).despawn();
    }
}

fn spawn_list_mode_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    mode: ListSelectionMode,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            ListSelectionModeOption(mode),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
        ))
        .with_children(|chip| {
            chip.spawn((
                ChipLabel,
                Text::new(""),
                LocalizedText::new(match label {
                    "Single" => "showcase.lists.mode_single",
                    "Multi" => "showcase.lists.mode_multi",
                    _ => label,
                })
                .with_default(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(label_color),
                NeedsInternationalFont,
            ));
        });
}
