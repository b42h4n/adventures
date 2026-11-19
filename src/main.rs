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

/// Папка, в которой лежит `data` (корень ассетов).
///
/// Bevy резолвит относительный `AssetPlugin::file_path` не от текущей
/// директории, а от BEVY_ASSET_ROOT / CARGO_MANIFEST_DIR (последний задаётся
/// только при `cargo run`) или от папки с исполняемым файлом. Поэтому при
/// запуске `target/release/game` напрямую ассеты искались в
/// `target/release/data`. Ищем папку `data` сами: сначала в текущей
/// директории, затем рядом с бинарником (для сборок «binary + data»).
pub fn asset_root() -> std::path::PathBuf {
    if let Ok(root) = std::env::var("BEVY_ASSET_ROOT") {
        return root.into();
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()));
    for dir in [Some(cwd.clone()), exe_dir].into_iter().flatten() {
        if dir.join("data").is_dir() {
            return dir;
        }
    }
    cwd
}

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
                    // Абсолютный путь: иначе Bevy откладывает его от папки
                    // с бинарником (target/release), а не от текущей директории.
                    file_path: asset_root().join("data").display().to_string(),
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
    info!("Папка ассетов: {}", asset_root().join("data").display());
    // Свой шрифт подхватывается, только если файл существует.
    if asset_root().join("data/fonts/ui.ttf").exists() {
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
