use super::MonsterDef;

// Шаблон нового персонажа (например primordial/titan.rs):
//
//   pub fn def() -> MonsterDef {
//       MonsterDef::new("titan", "Titan", MonsterKind::Primordial, Class::Tank)
//           .speed(3.5).texture("textures/titan.png")
//           .ability(AbilityDef::cone("Slam", KeyCode::KeyQ, 8.0, 40.0, 4.0, 120.0))
//   }

pub fn all() -> Vec<MonsterDef> {
    vec![]
}
