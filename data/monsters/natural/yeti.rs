use crate::data::monsters::*;
use bevy::prelude::{Color, KeyCode};

pub fn def() -> MonsterDef {
    MonsterDef::new("yeti", "Yeti", MonsterKind::Natural, Class::Control) // 300 HP
        .speed(5.0)
        .radius(0.7)
        // Если у файла другое расширение (.jpg и т.п.) — поменяйте здесь.
        .texture("textures/mammott.png")
        // Холодное дыхание: 25 урона, заморозка 3 с, отталкивание, КД 20 с
        .ability(
            AbilityDef::cone("Frost Breath", KeyCode::KeyQ, 20.0, 25.0, 7.0, 60.0)
                .effect(Effect::Freeze { seconds: 3.0 })
                .effect(Effect::Knockback { distance: 3.0 })
                .fx_color(Color::srgb(0.4, 0.85, 1.0)),
        )
        // Крик: 35 урона, КД 12 с
        .ability(
            AbilityDef::cone("Roar", KeyCode::KeyE, 12.0, 35.0, 6.0, 90.0)
                .fx_color(Color::srgb(1.0, 0.6, 0.2)),
        )
}
