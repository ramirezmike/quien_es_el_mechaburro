use crate::AppState;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_event::<ScoreAddEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::ScoreDisplay).with_system(handle_score_add_event),
            );
    }
}

pub struct ScoreAddEvent;

fn handle_score_add_event(
    mut score_add_event_reader: EventReader<ScoreAddEvent>,
    mut game_state: ResMut<GameState>,
) {
    if score_add_event_reader.iter().count() > 0 {
        let burro_points: HashMap<BurroSkin, usize> = game_state
            .dead_burros
            .iter()
            .enumerate()
            .map(|(i, b)| (*b, i))
            .into_iter()
            .collect();

        for mut burro in game_state.burros.iter_mut() {
            burro.score += burro_points.get(&burro.skin).unwrap_or(&7);
        }
    }
}

#[derive(Default)]
pub struct GameState {
    pub burros: Vec<BurroState>,
    pub dead_burros: Vec<BurroSkin>,
    pub current_level: usize,
    pub current_level_over: bool,
}

impl GameState {
    pub fn initialize(burro_count: usize, bot_count: usize) -> Self {
        let burro_count = burro_count.max(bot_count + 1);
        let mut burros = vec![];
        let skins = [
            BurroSkin::Pinata,
            BurroSkin::Meow,
            BurroSkin::Salud,
            BurroSkin::Mexico,
            BurroSkin::Medianoche,
            BurroSkin::Morir,
            BurroSkin::Gators,
            BurroSkin::Aguas,
        ];

        // human players
        for i in 0..burro_count - bot_count {
            burros.push(BurroState {
                score: 0,
                skin: skins[i],
                is_bot: false,
                hearts: vec![],
            });
        }

        // bots
        for i in 0..bot_count {
            burros.push(BurroState {
                score: 0,
                skin: skins[burro_count - bot_count + i],
                is_bot: true,
                hearts: vec![],
            });
        }

        GameState {
            burros,
            dead_burros: vec![],
            current_level: 0,
            current_level_over: false,
        }
    }

    pub fn on_new_level(&mut self) {
        self.dead_burros = vec![];
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BurroSkin {
    Pinata,
    Meow,
    Salud,
    Mexico,
    Medianoche,
    Morir,
    Gators,
    Aguas,
}

impl Default for BurroSkin {
    fn default() -> Self {
        BurroSkin::Pinata
    }
}

#[derive(Default)]
pub struct BurroState {
    pub score: usize,
    pub skin: BurroSkin,
    pub is_bot: bool,
    pub hearts: Vec<Entity>,
}
