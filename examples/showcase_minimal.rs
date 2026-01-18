//! Minimal-Bevy, UI-only demo.
//!
//! This intentionally avoids Bevy 3D / PBR types so it can be compiled with
//! `--no-default-features --features bevy_minimal`.
//!
//! Run with:
//!   cargo run --no-default-features --features bevy_minimal --example showcase_minimal

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy_material_ui::i18n::MaterialTranslations;
use bevy_material_ui::prelude::*;
use std::path::PathBuf;

#[derive(Resource)]
struct ShowcaseI18nHandles(Vec<Handle<MaterialTranslations>>);

fn main() {
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    App::new()
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: asset_root.to_string_lossy().to_string(),
                ..default()
            }),
        )
        .add_plugins(MaterialUiPlugin)
        // Pick a known language so LocalizedText updates are deterministic.
        .insert_resource(MaterialLanguage {
            tag: "en-US".to_string(),
        })
        .add_systems(Startup, (load_showcase_i18n_assets, setup_ui))
        .add_systems(Update, toggle_language_system)
        .run();
}

fn load_showcase_i18n_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Keep strong handles alive so the assets remain loaded.
    let handles = vec![
        asset_server.load::<MaterialTranslations>("i18n/en-US.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/es-ES.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/fr-FR.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/de-DE.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/ja-JP.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/zh-CN.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/he-IL.mui_lang"),
    ];
    commands.insert_resource(ShowcaseI18nHandles(handles));
}

fn toggle_language_system(keys: Res<ButtonInput<KeyCode>>, mut language: ResMut<MaterialLanguage>) {
    if !keys.just_pressed(KeyCode::KeyL) {
        return;
    }

    language.tag = match language.tag.as_str() {
        "en-US" => "es-ES",
        "es-ES" => "fr-FR",
        "fr-FR" => "de-DE",
        "de-DE" => "ja-JP",
        "ja-JP" => "zh-CN",
        "zh-CN" => "he-IL",
        _ => "en-US",
    }
    .to_string();

    info!("MaterialLanguage.tag set to '{}'", language.tag);
}

fn setup_ui(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .with_children(|root| {
            // Header showing that i18n assets are loaded and can be toggled.
            root.spawn((
                Text::new(""),
                LocalizedText::new("showcase.app.title").with_default("Material UI"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn((
                Text::new(""),
                LocalizedText::new("showcase.translations.help")
                    .with_default("Press L to cycle languages"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            // Scroll container (wheel scrolling should work here).
            root.spawn((
                ScrollContainer::vertical(),
                ScrollPosition::default(),
                Node {
                    height: Val::Px(320.0),
                    width: Val::Percent(100.0),
                    overflow: Overflow::scroll_y(),
                    padding: UiRect::all(Val::Px(12.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
                BorderRadius::all(Val::Px(12.0)),
            ))
            .with_children(|list| {
                for i in 1..=40 {
                    list.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::vertical(Val::Px(6.0)),
                            ..default()
                        },
                    ))
                    .with_children(|row| {
                        row.spawn((
                            Text::new(format!("Row {i}: ")),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));

                        // Some localized strings to validate runtime translation updates.
                        row.spawn((
                            Text::new(""),
                            LocalizedText::new("showcase.common.result_prefix")
                                .with_default("Result:"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                        ));
                    });
                }
            });
        });
}
