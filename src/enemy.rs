use crate::components::*;
use crate::{AppState, ARENA, PPU};
use bevy::prelude::*;

const NORMAL: Color = Color::srgb(0.8, 0.2, 0.2);
const ICE: Color = Color::srgb(0.45, 0.8, 1.0);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawner(Timer::from_seconds(1.5, TimerMode::Repeating)))
            .add_systems(
                Update,
                (spawn_enemies, chase_player, tint_enemies).run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Resource)]
struct EnemySpawner(Timer);

fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawner>,
    mut commands: Commands,
    enemies: Query<(), With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() || enemies.iter().count() >= 12 {
        return;
    }
    let Ok(p) = player.get_single() else { return };
    let a = time.elapsed_secs() * 91.7;
    let pos = p.translation.truncate() + Vec2::new(a.cos(), a.sin()) * 15.0 * PPU;
    let lim = ARENA * PPU;
    commands.spawn((
        GameEntity,
        Enemy,
        Health { current: 100.0, max: 100.0 },
        Speed(2.4 * PPU),
        Sprite {
            color: NORMAL,
            custom_size: Some(Vec2::splat(PPU)),
            ..default()
        },
        Transform::from_xyz(pos.x.clamp(-lim, lim), pos.y.clamp(-lim, lim), 1.0),
    ));
}

fn chase_player(
    time: Res<Time>,
    mut enemies: Query<
        (&mut Transform, &Speed),
        (With<Enemy>, Without<Player>, Without<Frozen>, Without<Knockback>),
    >,
    mut player: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
) {
    let Ok((ptf, mut php)) = player.get_single_mut() else { return };
    if php.current <= 0.0 {
        return;
    }
    for (mut tf, speed) in &mut enemies {
        let to = ptf.translation.truncate() - tf.translation.truncate();
        let dist = to.length();
        if dist > 1.5 * PPU {
            tf.translation += (to / dist * speed.0 * time.delta_secs()).extend(0.0);
        } else {
            php.current -= 8.0 * time.delta_secs(); // урон при контакте
        }
    }
}

/// Замороженные враги становятся голубыми.
fn tint_enemies(mut q: Query<(&mut Sprite, Option<&Frozen>), With<Enemy>>) {
    for (mut s, f) in &mut q {
        s.color = if f.is_some() { ICE } else { NORMAL };
    }
}
