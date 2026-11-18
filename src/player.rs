use crate::components::*;
use crate::data::monsters::MonsterRegistry;
use crate::{AppState, ARENA, PPU};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player)
            .add_systems(
                Update,
                (move_player, aim_player, follow_camera, back_to_menu)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    registry: Res<MonsterRegistry>,
    sel: Res<Selection>,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    let def = registry.get(sel.id).expect("monster id not found");
    let size = def.radius * 2.0 * PPU * 1.6;
    commands.spawn((
        GameEntity,
        Player,
        Health { current: def.max_hp, max: def.max_hp },
        Speed(def.speed * PPU),
        Facing(Vec2::X),
        Abilities(def.abilities.iter().cloned().map(AbilitySlot::new).collect()),
        Sprite {
            image: assets.load(def.texture),
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Speed, &Health), With<Player>>,
) {
    let Ok((mut tf, speed, hp)) = q.get_single_mut() else { return };
    if hp.current <= 0.0 {
        return;
    }
    let mut d = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { d.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { d.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { d.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { d.x += 1.0; }
    if d != Vec2::ZERO {
        let step = d.normalize() * speed.0 * time.delta_secs();
        tf.translation.x = (tf.translation.x + step.x).clamp(-ARENA * PPU, ARENA * PPU);
        tf.translation.y = (tf.translation.y + step.y).clamp(-ARENA * PPU, ARENA * PPU);
    }
}

/// Направление к курсору мыши; спрайт отражается по горизонтали.
fn aim_player(
    windows: Query<&Window, With<PrimaryWindow>>,
    cams: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut q: Query<(&Transform, &mut Facing, &mut Sprite), With<Player>>,
) {
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    let Ok((cam, cam_tf)) = cams.get_single() else { return };
    let Ok(point) = cam.viewport_to_world_2d(cam_tf, cursor) else { return };
    let Ok((tf, mut facing, mut sprite)) = q.get_single_mut() else { return };
    if let Some(d) = (point - tf.translation.truncate()).try_normalize() {
        facing.0 = d;
        sprite.flip_x = d.x < 0.0;
    }
}

fn follow_camera(
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut cam: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let (Ok(p), Ok(mut c)) = (player.get_single(), cam.get_single_mut()) else { return };
    c.translation.x = p.translation.x;
    c.translation.y = p.translation.y;
}

fn back_to_menu(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(AppState::MainMenu);
    }
}
