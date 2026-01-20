//! Menu Demo
//!
//! Demonstrates Material Design 3 menus.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_EXPAND_MORE, ICON_MORE_VERT};
use bevy_material_ui::list::{
    ListItemBody, ListItemBuilder, ListItemHeadline, ListItemSupportingText,
};
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct MenuTrigger;

#[derive(Component)]
struct MenuDropdown;

#[derive(Component)]
struct MenuItemMarker(pub String);

#[derive(Component)]
struct MenuSelectedText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, menu_demo_system)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
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
        .insert_test_id("menu_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|container| {
                // Trigger button
                let trigger_button =
                    MaterialButton::new("Options").with_variant(ButtonVariant::Outlined);
                let trigger_bg = trigger_button.background_color(&theme);
                let trigger_border = trigger_button.border_color(&theme);

                container
                    .spawn((
                        MenuTrigger,
                        trigger_button,
                        Button,
                        Interaction::None,
                        RippleHost::new(),
                        Node {
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(8.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(trigger_bg),
                        BorderColor::all(trigger_border),
                        BorderRadius::all(Val::Px(8.0)),
                    ))
                    .insert_test_id("menu_demo/trigger", &telemetry)
                    .with_children(|btn| {
                        if let Some(icon) = MaterialIcon::from_name(ICON_MORE_VERT) {
                            btn.spawn(icon.with_size(20.0).with_color(theme.on_surface));
                        }
                        btn.spawn((
                            MenuSelectedText,
                            Text::new("Options"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));
                        if let Some(icon) = MaterialIcon::from_name(ICON_EXPAND_MORE) {
                            btn.spawn(icon.with_size(20.0).with_color(theme.on_surface));
                        }
                    });

                // Dropdown (hidden by default)
                container
                    .spawn((
                        MenuDropdown,
                        Visibility::Hidden,
                        Node {
                            width: Val::Px(200.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::vertical(Val::Px(8.0)),
                            margin: UiRect::top(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container),
                        BorderRadius::all(Val::Px(4.0)),
                        BoxShadow::from(ShadowStyle {
                            color: Color::BLACK.with_alpha(0.2),
                            x_offset: Val::Px(0.0),
                            y_offset: Val::Px(4.0),
                            spread_radius: Val::Px(0.0),
                            blur_radius: Val::Px(8.0),
                        }),
                    ))
                    .insert_test_id("menu_demo/dropdown", &telemetry)
                    .with_children(|menu| {
                        spawn_menu_item(menu, &theme, "Cut", "Ctrl+X", false, &telemetry);
                        spawn_menu_item(menu, &theme, "Copy", "Ctrl+C", false, &telemetry);
                        spawn_menu_item(menu, &theme, "Paste", "Ctrl+V", false, &telemetry);

                        menu.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(1.0),
                                margin: UiRect::vertical(Val::Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(theme.outline_variant),
                        ));

                        spawn_menu_item(menu, &theme, "Delete", "", true, &telemetry);
                    });
            });
        });
}

fn spawn_menu_item(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    shortcut: &str,
    is_destructive: bool,
    telemetry: &TelemetryConfig,
) {
    let headline_color = if is_destructive {
        theme.error
    } else {
        theme.on_surface
    };
    let supporting_color = theme.on_surface_variant;
    let has_supporting = !shortcut.is_empty();

    let builder = if has_supporting {
        ListItemBuilder::new(label).two_line()
    } else {
        ListItemBuilder::new(label)
    };

    let test_suffix = label.to_ascii_lowercase().replace(' ', "_");

    parent
        .spawn((
            MenuItemMarker(label.to_string()),
            Interaction::None,
            builder.build(theme),
        ))
        .insert_test_id(format!("menu_demo/item/{test_suffix}"), telemetry)
        .with_children(|item| {
            item.spawn((
                ListItemBody,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
            ))
            .with_children(|body| {
                body.spawn((
                    ListItemHeadline,
                    Text::new(label),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                if has_supporting {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(shortcut),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(supporting_color),
                    ));
                }
            });
        });
}

#[allow(clippy::type_complexity)]
fn menu_demo_system(
    mut triggers: Query<(&ChildOf, &Interaction), (With<MenuTrigger>, Changed<Interaction>)>,
    mut dropdowns: Query<(&ChildOf, &mut Visibility), With<MenuDropdown>>,
    mut items: Query<(&ChildOf, &Interaction, &MenuItemMarker), Changed<Interaction>>,
    triggers_all: Query<(Entity, &ChildOf), With<MenuTrigger>>,
    mut selected_text: Query<(&ChildOf, &mut Text), With<MenuSelectedText>>,
    parents: Query<&ChildOf>,
) {
    // Build lookup: container -> trigger entity
    let mut trigger_by_container: std::collections::HashMap<Entity, Entity> =
        std::collections::HashMap::new();
    for (trigger_entity, parent) in triggers_all.iter() {
        trigger_by_container.insert(parent.0, trigger_entity);
    }

    // Toggle dropdown on trigger press
    for (parent, interaction) in triggers.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let container = parent.0;
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = match *vis {
                    Visibility::Hidden => Visibility::Inherited,
                    _ => Visibility::Hidden,
                };
            }
        }
    }

    // Select item
    for (parent, interaction, label) in items.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Item parent is the dropdown; dropdown parent is the container.
        let dropdown_entity = parent.0;
        let Ok(container_parent) = parents.get(dropdown_entity) else {
            continue;
        };
        let container = container_parent.0;

        // Update selected text on trigger button
        if let Some(trigger_entity) = trigger_by_container.get(&container).copied() {
            for (text_parent, mut text) in selected_text.iter_mut() {
                if text_parent.0 == trigger_entity {
                    *text = Text::new(label.0.as_str());
                }
            }
        }

        // Close dropdown
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = Visibility::Hidden;
            }
        }
    }
}
