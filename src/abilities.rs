use crate::components::*;
use crate::data::monsters::Effect;
use crate::{AppState, PPU};
use bevy::prelude::*;

const KNOCK_TIME: f32 = 0.25;

pub struct AbilityPlugin;
impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_cooldowns,
                cast_abilities,
                tick_frozen,
                apply_knockback,
                reap_dead,
                draw_effects,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn tick_cooldowns(time: Res<Time>, mut q: Query<&mut Abilities>) {
    for mut a in &mut q {
        for slot in &mut a.0 {
            slot.cooldown.tick(time.delta());
        }
    }
}

fn cast_abilities(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut players: Query<(&Transform, &Facing, &mut Abilities, &Health), With<Player>>,
    mut enemies: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    let Ok((tf, facing, mut abilities, hp)) = players.get_single_mut() else { return };
    if hp.current <= 0.0 {
        return;
    }
    let origin = tf.translation.truncate();
    let fwd = facing.0;

    for slot in abilities.0.iter_mut() {
        if !keys.just_pressed(slot.def.key) || !slot.cooldown.finished() {
            continue;
        }
        slot.cooldown.reset();
        let range = slot.def.range * PPU;
        let cos_half = (slot.def.angle_deg * 0.5).to_radians().cos();

        for (e, etf, mut ehp) in &mut enemies {
            let to = etf.translation.truncate() - origin;
            let dist = to.length();
            if dist > range {
                continue;
            }
            let dir = if dist > 1e-3 { to / dist } else { fwd };
            if fwd.dot(dir) < cos_half {
                continue;
            }
            ehp.current -= slot.def.damage;
            for fx in &slot.def.effects {
                match fx {
                    Effect::Freeze { seconds } => {
                        commands
                            .entity(e)
                            .insert(Frozen(Timer::from_seconds(*seconds, TimerMode::Once)));
                    }
                    Effect::Knockback { distance } => {
                        commands.entity(e).insert(Knockback {
                            velocity: dir * (distance * PPU / KNOCK_TIME),
                            timer: Timer::from_seconds(KNOCK_TIME, TimerMode::Once),
                        });
                    }
                }
            }
        }

        commands.spawn((
            GameEntity,
            ConeFx {
                origin,
                dir: fwd,
                range,
                angle_deg: slot.def.angle_deg,
                color: slot.def.fx_color,
                timer: Timer::from_seconds(0.35, TimerMode::Once),
            },
        ));
    }
}

fn tick_frozen(time: Res<Time>, mut commands: Commands, mut q: Query<(Entity, &mut Frozen)>) {
    for (e, mut f) in &mut q {
        f.0.tick(time.delta());
        if f.0.finished() {
            commands.entity(e).remove::<Frozen>();
        }
    }
}

fn apply_knockback(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut Knockback)>,
) {
    for (e, mut tf, mut k) in &mut q {
        tf.translation += (k.velocity * time.delta_secs()).extend(0.0);
        k.timer.tick(time.delta());
        if k.timer.finished() {
            commands.entity(e).remove::<Knockback>();
        }
    }
}

fn reap_dead(
    mut commands: Commands,
    mut score: ResMut<Score>,
    q: Query<(Entity, &Health), With<Enemy>>,
) {
    for (e, hp) in &q {
        if hp.current <= 0.0 {
            commands.entity(e).despawn_recursive();
            score.0 += 1;
        }
    }
}

/// Конусы способностей и полоски HP — через гизмо.
fn draw_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut fx: Query<(Entity, &mut ConeFx)>,
    units: Query<(&Transform, &Health, &Sprite)>,
) {
    for (e, mut c) in &mut fx {
        c.timer.tick(time.delta());
        if c.timer.finished() {
            commands.entity(e).despawn();
            continue;
        }
        let half = (c.angle_deg * 0.5).to_radians();
        let (o, dir, range, color) = (c.origin, c.dir, c.range, c.color);
        let at = |a: f32| o + Vec2::from_angle(a).rotate(dir) * range;
        gizmos.line_2d(o, at(half), color);
        gizmos.line_2d(o, at(-half), color);
        const N: i32 = 20;
        let mut prev = at(-half);
        for i in 1..=N {
            let a = -half + 2.0 * half * (i as f32 / N as f32);
            let p = at(a);
            gizmos.line_2d(prev, p, color);
            prev = p;
        }
    }

    for (tf, hp, sprite) in &units {
        let offset = sprite.custom_size.map(|s| s.y / 2.0 + 8.0).unwrap_or(40.0);
        let pos = tf.translation.truncate() + Vec2::Y * offset;
        let w = 60.0;
        let start = pos - Vec2::X * (w / 2.0);
        let frac = (hp.current / hp.max).clamp(0.0, 1.0);
        gizmos.line_2d(start, start + Vec2::X * w, Color::srgb(0.5, 0.1, 0.1));
        gizmos.line_2d(start, start + Vec2::X * w * frac, Color::srgb(0.2, 0.9, 0.3));
    }
}
