use crate::data::monsters::{AbilityDef, MonsterKind};
use bevy::prelude::*;
use std::time::Duration;

/// Всё, что создаётся на время игры; удаляется при выходе в меню.
#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
/// Скорость в пикселях в секунду.
#[derive(Component)]
pub struct Speed(pub f32);
/// Направление взгляда (нормализованное).
#[derive(Component)]
pub struct Facing(pub Vec2);

#[derive(Component)]
pub struct Frozen(pub Timer);
#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}

pub struct AbilitySlot {
    pub def: AbilityDef,
    pub cooldown: Timer,
}
impl AbilitySlot {
    pub fn new(def: AbilityDef) -> Self {
        let mut cooldown = Timer::from_seconds(def.cooldown, TimerMode::Once);
        cooldown.tick(Duration::from_secs_f32(def.cooldown)); // готова сразу
        Self { def, cooldown }
    }
}
#[derive(Component)]
pub struct Abilities(pub Vec<AbilitySlot>);

/// Визуальный эффект конуса (рисуется гизмо), размеры в пикселях.
#[derive(Component)]
pub struct ConeFx {
    pub origin: Vec2,
    pub dir: Vec2,
    pub range: f32,
    pub angle_deg: f32,
    pub color: Color,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct Settings {
    pub volume: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self { volume: 0.5 }
    }
}

/// Выбор игрока в меню.
#[derive(Resource)]
pub struct Selection {
    pub kind: MonsterKind,
    pub id: &'static str,
}
impl Default for Selection {
    fn default() -> Self {
        Self { kind: MonsterKind::Natural, id: "yeti" }
    }
}

/// Пользовательский шрифт (если лежит data/fonts/ui.ttf), иначе шрифт Bevy.
#[derive(Resource, Default)]
pub struct UiFont(pub Option<Handle<Font>>);
impl UiFont {
    pub fn font(&self, size: f32) -> TextFont {
        TextFont {
            font: self.0.clone().unwrap_or_default(),
            font_size: size,
            ..default()
        }
    }
}

pub fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
