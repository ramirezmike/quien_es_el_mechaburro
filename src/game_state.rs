use crate::{AppState};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .add_event::<ScoreAddEvent>()
            .add_system(handle_score_add_event.in_set(OnUpdate(AppState::ScoreDisplay)));
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
        let max_score = game_state.dead_burros.len();

        for burro in game_state.burros.iter_mut() {
            let new_score = burro_points.get(&burro.skin).unwrap_or(&max_score);
            burro.score += new_score;
        }
    }
}

#[derive(Default, Resource)]
pub struct GameState {
    pub burros: Vec<BurroState>,
    pub dead_burros: Vec<BurroSkin>,
    pub current_level: usize,
    pub current_level_over: bool,
    pub players: Vec<BurroCharacter>,
    pub difficulty: f32,
}

impl GameState {
    pub fn get_skin_player_map(&self) -> HashMap<BurroSkin, usize> {
        let mut map: HashMap<BurroSkin, usize> = HashMap::new();
        for player in self.players.iter() {
            map.insert(player.selected_burro, player.player);
        }
        map
    }

    pub fn initialize(players: Vec<BurroCharacter>, number_of_bots: usize, difficulty: f32) -> Self {
        let mut burros = vec![];
        let skins = vec![
            BurroSkin::Pinata,
            BurroSkin::Meow,
            BurroSkin::Salud,
            BurroSkin::Mexico,
            BurroSkin::Medianoche,
            BurroSkin::Morir,
            BurroSkin::Gators,
            BurroSkin::Aguas,
        ];
        let picked_skins = players.iter().map(|b| b.selected_burro).collect::<Vec<_>>();
        let skins = skins
            .iter()
            .filter(|s| !picked_skins.contains(s))
            .collect::<Vec<_>>();

        // human players
        for p in players.iter() {
            burros.push(BurroState {
                score: 0,
                skin: p.selected_burro,
                is_bot: false,
                hearts: vec![],
            });
        }

        // bots
        for skin in skins.iter().take(number_of_bots) {
            burros.push(BurroState {
                score: 0,
                skin: **skin,
                is_bot: true,
                hearts: vec![],
            });
        }

        GameState {
            burros,
            dead_burros: vec![],
            current_level: 0,
            current_level_over: false,
            players,
            difficulty,
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

#[derive(Component, Clone, Copy)]
pub struct BurroCharacter {
    pub player: usize,
    pub is_playing: bool,
    pub has_picked: bool,
    pub selected_burro: BurroSkin,
    pub action_cooldown: f32,
}
