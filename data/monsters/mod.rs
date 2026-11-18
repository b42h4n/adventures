//! Общие типы для описания монстров + реестр.
//! Новый персонаж = одна функция `def()` в natural/ или primordial/.

use bevy::prelude::*;
use std::collections::HashMap;

pub mod natural;
pub mod primordial;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterKind {
    Natural,
    Primordial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Attack,
    Control,
    Defense,
    Tank,
}

impl Class {
    /// Здоровье определяется классом.
    pub const fn base_hp(self) -> f32 {
        match self {
            Class::Attack => 150.0,
            Class::Control => 300.0,
            Class::Defense => 450.0,
            Class::Tank => 650.0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Effect {
    /// Враг не двигается `seconds` секунд.
    Freeze { seconds: f32 },
    /// Отбрасывание от кастера на `distance` единиц.
    Knockback { distance: f32 },
}

/// Способность, бьющая по конусу перед персонажем.
#[derive(Clone, Debug)]
pub struct AbilityDef {
    pub name: &'static str,
    pub key: KeyCode,
    pub cooldown: f32,
    pub damage: f32,
    pub range: f32,
    pub angle_deg: f32,
    pub effects: Vec<Effect>,
    pub fx_color: Color,
}

impl AbilityDef {
    pub fn cone(
        name: &'static str,
        key: KeyCode,
        cooldown: f32,
        damage: f32,
        range: f32,
        angle_deg: f32,
    ) -> Self {
        Self {
            name,
            key,
            cooldown,
            damage,
            range,
            angle_deg,
            effects: Vec::new(),
            fx_color: Color::WHITE,
        }
    }
    pub fn effect(mut self, e: Effect) -> Self {
        self.effects.push(e);
        self
    }
    pub fn fx_color(mut self, c: Color) -> Self {
        self.fx_color = c;
        self
    }

    /// Текст описания, генерируется из данных способности.
    pub fn describe(&self) -> String {
        let key = format!("{:?}", self.key).replace("Key", "");
        let mut s = format!(
            "[{key}] {}: {} damage, cooldown {}s, cone {} deg, range {}",
            self.name, self.damage, self.cooldown, self.angle_deg, self.range
        );
        for e in &self.effects {
            match e {
                Effect::Freeze { seconds } => s.push_str(&format!(", freezes {seconds}s")),
                Effect::Knockback { .. } => s.push_str(", knocks back"),
            }
        }
        s
    }
}

#[derive(Clone, Debug)]
pub struct MonsterDef {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: MonsterKind,
    pub class: Class,
    pub max_hp: f32,
    pub speed: f32,
    pub radius: f32,
    /// Путь относительно папки data, например "textures/mammott.png".
    pub texture: &'static str,
    pub abilities: Vec<AbilityDef>,
}

impl MonsterDef {
    /// HP берётся из класса, остальное настраивается билдером.
    pub fn new(id: &'static str, name: &'static str, kind: MonsterKind, class: Class) -> Self {
        Self {
            id,
            name,
            kind,
            class,
            max_hp: class.base_hp(),
            speed: 5.0,
            radius: 0.6,
            texture: "",
            abilities: Vec::new(),
        }
    }
    pub fn speed(mut self, v: f32) -> Self {
        self.speed = v;
        self
    }
    pub fn radius(mut self, v: f32) -> Self {
        self.radius = v;
        self
    }
    pub fn texture(mut self, path: &'static str) -> Self {
        self.texture = path;
        self
    }
    pub fn ability(mut self, a: AbilityDef) -> Self {
        self.abilities.push(a);
        self
    }
}

#[derive(Resource)]
pub struct MonsterRegistry(HashMap<&'static str, MonsterDef>);

impl MonsterRegistry {
    pub fn load() -> Self {
        let mut map = HashMap::new();
        for d in natural::all().into_iter().chain(primordial::all()) {
            map.insert(d.id, d);
        }
        Self(map)
    }
    pub fn get(&self, id: &str) -> Option<&MonsterDef> {
        self.0.get(id)
    }
    pub fn by_kind(&self, kind: MonsterKind) -> Vec<&MonsterDef> {
        let mut v: Vec<_> = self.0.values().filter(|d| d.kind == kind).collect();
        v.sort_by_key(|d| d.name);
        v
    }
}
