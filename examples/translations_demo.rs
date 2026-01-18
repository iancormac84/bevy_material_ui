//! Translations (i18n) Demo
//!
//! Demonstrates loading a `.mui_lang` file and using `LocalizedText`.

use bevy::prelude::*;
use bevy_material_ui::i18n::MaterialTranslations;
use bevy_material_ui::prelude::*;

#[derive(Resource)]
struct I18nHandles {
    _handles: Vec<Handle<MaterialTranslations>>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        // Pick a known language so the demo is deterministic.
        .insert_resource(MaterialLanguage {
            tag: "en-US".to_string(),
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Load one of the shipped language files (assets/i18n/en-US.mui_lang).
    // The i18n plugin will ingest these assets into the translations system.
    // Keep strong handles alive so the assets remain loaded.
    commands.insert_resource(I18nHandles {
        _handles: vec![asset_server.load::<MaterialTranslations>("i18n/en-US.mui_lang")],
    });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("translations_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new(""),
                // `LocalizedText` uses keys from the loaded .mui_lang file.
                LocalizedText::new("showcase.translations.greeting").with_default("Hello!"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn((
                Text::new(""),
                LocalizedText::new("showcase.translations.instructions")
                    .with_default("Load assets/i18n/*.mui_lang and attach LocalizedText"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
