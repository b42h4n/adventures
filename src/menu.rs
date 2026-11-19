use crate::components::*;
use crate::data::monsters::{MonsterKind, MonsterRegistry};
use crate::AppState;
use bevy::prelude::*;

const NORMAL: Color = Color::srgb(0.2, 0.26, 0.36);
const HOVER: Color = Color::srgb(0.3, 0.4, 0.55);
const PRESSED: Color = Color::srgb(0.15, 0.55, 0.35);

#[derive(Component)]
struct ScreenRoot;
#[derive(Component)]
struct VolumeLabel;

#[derive(Component, Clone, Copy)]
enum Action {
    Goto(AppState),
    Kind(MonsterKind),
    Monster(&'static str),
    VolumeDown,
    VolumeUp,
    Quit,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu)
            .add_systems(OnEnter(AppState::Settings), settings_menu)
            .add_systems(OnEnter(AppState::Credits), credits_menu)
            .add_systems(OnEnter(AppState::TypeSelect), type_select)
            .add_systems(OnEnter(AppState::CharList), char_list)
            .add_systems(OnEnter(AppState::CharPreview), char_preview)
            .add_systems(
                Update,
                (button_visuals, button_actions, update_volume_label, apply_volume),
            );
        for s in [
            AppState::MainMenu,
            AppState::Settings,
            AppState::Credits,
            AppState::TypeSelect,
            AppState::CharList,
            AppState::CharPreview,
        ] {
            app.add_systems(OnExit(s), cleanup::<ScreenRoot>);
        }
    }
}

// ---------- helpers ----------

fn label(font: &UiFont, s: impl Into<String>, size: f32) -> impl Bundle {
    (Text::new(s), font.font(size), TextColor(Color::WHITE))
}

fn button_w(p: &mut ChildBuilder, font: &UiFont, text: &str, action: Action, width: f32) {
    p.spawn((
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(52.0),
            margin: UiRect::bottom(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL),
        action,
    ))
    .with_children(|b| {
        b.spawn(label(font, text, 24.0));
    });
}

fn button(p: &mut ChildBuilder, font: &UiFont, text: &str, action: Action) {
    button_w(p, font, text, action, 260.0);
}

/// Колонка в левом верхнем углу.
fn root_top_left() -> impl Bundle {
    (
        ScreenRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        },
    )
}

// ---------- screens ----------

fn main_menu(mut commands: Commands, font: Res<UiFont>) {
    commands.spawn(root_top_left()).with_children(|p| {
        p.spawn((label(&font, "MONSTERS", 44.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        button(p, &font, "Play", Action::Goto(AppState::TypeSelect));
        button(p, &font, "Settings", Action::Goto(AppState::Settings));
        button(p, &font, "Quit", Action::Quit);
    });
}

fn settings_menu(mut commands: Commands, font: Res<UiFont>, settings: Res<Settings>) {
    commands.spawn(root_top_left()).with_children(|p| {
        p.spawn((label(&font, "SETTINGS", 36.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        p.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        })
        .with_children(|row| {
            button_w(row, &font, "-", Action::VolumeDown, 52.0);
            row.spawn((
                VolumeLabel,
                label(&font, format!("Volume: {:.0}%", settings.volume * 100.0), 24.0),
                Node { width: Val::Px(160.0), ..default() },
            ));
            button_w(row, &font, "+", Action::VolumeUp, 52.0);
        });
        button(p, &font, "Credits", Action::Goto(AppState::Credits));
        button(p, &font, "Back", Action::Goto(AppState::MainMenu));
    });
}

fn read_credits() -> String {
    let data = crate::asset_root().join("data");
    for name in ["credits.txt", "credits.md", "credits"] {
        if let Ok(s) = std::fs::read_to_string(data.join(name)) {
            return s;
        }
    }
    "Credits file not found (data/credits.txt)".to_string()
}

fn credits_menu(mut commands: Commands, font: Res<UiFont>) {
    let text = read_credits();
    commands.spawn(root_top_left()).with_children(|p| {
        p.spawn((label(&font, "CREDITS", 36.0), Node { margin: UiRect::bottom(Val::Px(16.0)), ..default() }));
        p.spawn((label(&font, text, 20.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        button(p, &font, "Back", Action::Goto(AppState::Settings));
    });
}

fn type_select(mut commands: Commands, font: Res<UiFont>) {
    commands.spawn(root_top_left()).with_children(|p| {
        p.spawn((label(&font, "CHOOSE TYPE", 36.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        button(p, &font, "Natural", Action::Kind(MonsterKind::Natural));
        button(p, &font, "Primordial", Action::Kind(MonsterKind::Primordial));
        button(p, &font, "Back", Action::Goto(AppState::MainMenu));
    });
}

fn char_list(
    mut commands: Commands,
    font: Res<UiFont>,
    registry: Res<MonsterRegistry>,
    sel: Res<Selection>,
) {
    let monsters = registry.by_kind(sel.kind);
    commands.spawn(root_top_left()).with_children(|p| {
        p.spawn((label(&font, format!("{:?} MONSTERS", sel.kind), 36.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        if monsters.is_empty() {
            p.spawn((label(&font, "No monsters yet", 22.0), Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));
        }
        for m in monsters {
            button(p, &font, m.name, Action::Monster(m.id));
        }
        button(p, &font, "Back", Action::Goto(AppState::TypeSelect));
    });
}

fn char_preview(
    mut commands: Commands,
    font: Res<UiFont>,
    registry: Res<MonsterRegistry>,
    sel: Res<Selection>,
    assets: Res<AssetServer>,
) {
    let Some(def) = registry.get(sel.id).cloned() else { return };
    let tex: Handle<Image> = assets.load(def.texture);
    commands
        .spawn((
            ScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|top| {
                top.spawn(label(&font, def.name, 40.0));
                top.spawn((
                    Node {
                        width: Val::Px(240.0),
                        height: Val::Px(240.0),
                        ..default()
                    },
                    ImageNode::new(tex),
                ));
                top.spawn(label(
                    &font,
                    format!("Class: {:?}   HP: {}   Speed: {}", def.class, def.max_hp, def.speed),
                    22.0,
                ));
                for a in &def.abilities {
                    top.spawn(label(&font, a.describe(), 18.0));
                }
            });
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                button(row, &font, "Back", Action::Goto(AppState::CharList));
                button(row, &font, "Select", Action::Goto(AppState::Playing));
            });
        });
}

// ---------- systems ----------

fn button_visuals(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (i, mut bg) in &mut q {
        bg.0 = match *i {
            Interaction::Pressed => PRESSED,
            Interaction::Hovered => HOVER,
            Interaction::None => NORMAL,
        };
    }
}

fn button_actions(
    q: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mut next: ResMut<NextState<AppState>>,
    mut sel: ResMut<Selection>,
    mut settings: ResMut<Settings>,
    mut exit: EventWriter<AppExit>,
) {
    for (i, a) in &q {
        if *i != Interaction::Pressed {
            continue;
        }
        match *a {
            Action::Goto(s) => next.set(s),
            Action::Kind(k) => {
                sel.kind = k;
                next.set(AppState::CharList);
            }
            Action::Monster(id) => {
                sel.id = id;
                next.set(AppState::CharPreview);
            }
            Action::VolumeDown => settings.volume = (((settings.volume - 0.1) * 10.0).round() / 10.0).clamp(0.0, 1.0),
            Action::VolumeUp => settings.volume = (((settings.volume + 0.1) * 10.0).round() / 10.0).clamp(0.0, 1.0),
            Action::Quit => {
                exit.send(AppExit::Success);
            }
        }
    }
}

fn update_volume_label(settings: Res<Settings>, mut q: Query<&mut Text, With<VolumeLabel>>) {
    if !settings.is_changed() {
        return;
    }
    for mut t in &mut q {
        t.0 = format!("Volume: {:.0}%", settings.volume * 100.0);
    }
}

/// Громкость применяется ко всему звуку игры.
fn apply_volume(settings: Res<Settings>, mut commands: Commands) {
    if settings.is_changed() {
        commands.insert_resource(GlobalVolume::new(settings.volume));
    }
}
