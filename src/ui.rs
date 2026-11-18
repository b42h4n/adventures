use crate::components::*;
use crate::AppState;
use bevy::prelude::*;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Hud;

fn setup_hud(mut commands: Commands, font: Res<UiFont>) {
    commands.spawn((
        GameEntity,
        Hud,
        Text::new(""),
        font.font(22.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn update_hud(
    player: Query<(&Health, &Abilities), With<Player>>,
    score: Res<Score>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    let (Ok(mut text), Ok((hp, ab))) = (hud.get_single_mut(), player.get_single()) else { return };
    let mut s = format!("HP {:.0}/{:.0}   Kills {}   (Esc - menu)\n", hp.current.max(0.0), hp.max, score.0);
    if hp.current <= 0.0 {
        s.push_str("YOU DIED\n");
    }
    for slot in &ab.0 {
        let key = format!("{:?}", slot.def.key).replace("Key", "");
        if slot.cooldown.finished() {
            s.push_str(&format!("[{key}] {} - READY\n", slot.def.name));
        } else {
            s.push_str(&format!("[{key}] {} - {:.1}s\n", slot.def.name, slot.cooldown.remaining_secs()));
        }
    }
    text.0 = s;
}
