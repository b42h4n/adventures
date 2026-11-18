use super::MonsterDef;

pub mod yeti;

/// Новый персонаж: создайте файл, объявите `pub mod ...;` и добавьте `...::def()` сюда.
pub fn all() -> Vec<MonsterDef> {
    vec![yeti::def()]
}
