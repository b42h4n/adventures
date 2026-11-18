mod abilities;
mod components;
mod enemy;
mod menu;
mod player;
mod ui;

#[path = "../data/mod.rs"]
mod data;

use bevy::prelude::*;
use bevy::sprite::SpriteImageMode;
use components::*;
use data::monsters::MonsterRegistry;

/// Пикселей на игровую единицу (скорости/дальности в данных заданы в единицах).
pub const PPU: f32 = 48.0;
/// Половина размера арены в единицах.
pub const ARENA: f32 = 19.0;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Settings,
    Credits,
    TypeSelect,
    CharList,
    CharPreview,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "data".into(), // текстуры: data/textures, шрифт: data/fonts
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Monsters".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.08, 0.12, 0.1)))
        .insert_resource(MonsterRegistry::load())
        .init_resource::<Selection>()
        .init_resource::<Settings>()
        .init_resource::<UiFont>()
        .init_resource::<Score>()
        .init_state::<AppState>()
        .add_plugins((
            menu::MenuPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            abilities::AbilityPlugin,
            ui::UiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Playing), spawn_arena)
        .add_systems(
            OnExit(AppState::Playing),
            (cleanup::<GameEntity>, reset_camera),
        )
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut font: ResMut<UiFont>) {
    commands.spawn(Camera2d);
    // Свой шрифт подхватывается, только если файл существует.
    if std::path::Path::new("data/fonts/ui.ttf").exists() {
        font.0 = Some(assets.load("fonts/ui.ttf"));
    }
}

fn spawn_arena(mut commands: Commands, assets: Res<AssetServer>) {
    let size = ARENA * PPU * 2.0;
    commands.spawn((
        GameEntity,
        Sprite {
            image: assets.load("textures/grass.png"),
            custom_size: Some(Vec2::splat(size)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn reset_camera(mut cam: Query<&mut Transform, With<Camera2d>>) {
    for mut t in &mut cam {
        t.translation.x = 0.0;
        t.translation.y = 0.0;
    }
}
