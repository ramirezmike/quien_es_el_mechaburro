use crate::{AppState, assets};
use bevy::prelude::*;
use std::collections::HashMap;
use rand::Rng;

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
        let burro_points: HashMap<usize, usize> = game_state
            .dead_burros
            .iter()
            .enumerate()
            .map(|(i, b)| (*b, i))
            .collect();
        let max_score = game_state.dead_burros.len();

        for burro in game_state.burros.iter_mut() {
            let new_score = burro_points.get(&burro.selected_burro).unwrap_or(&max_score);
            burro.score += new_score;
        }
    }
}

#[derive(Default, Resource)]
pub struct GameState {
    pub burros: Vec<BurroState>,
    pub dead_burros: Vec<usize>,
    pub current_level: usize,
    pub current_level_over: bool,
    pub players: Vec<BurroCharacter>,
    pub difficulty: f32,
}

impl GameState {
//  pub fn get_skin_player_map(&self) -> HashMap<BurroSkin, usize> {
//      let mut map: HashMap<BurroSkin, usize> = HashMap::new();
//      for player in self.players.iter() {
//          map.insert(player.selected_burro, player.player);
//      }
//      map
//  }

    pub fn initialize(
        players: Vec<BurroCharacter>,
        number_of_bots: usize,
        difficulty: f32,
        burro_assets: &Vec<assets::BurroAsset>,
    ) -> Self {
        let mut burros = vec![];
        let mut available_burros: Vec<usize> = (0..burro_assets.len()).collect();

        // human players
        for p in players.iter() {
            burros.push(BurroState {
                selected_burro: p.selected_burro,
                score: 0,
                is_bot: false,
                hearts: vec![],
            });
        }

        let claimed_burros: Vec<usize> = burros.iter().map(|x| x.selected_burro).collect();
        available_burros.retain(|x| !claimed_burros.contains(&x));

        // bots
        for _ in 0..number_of_bots {
            let index = rand::thread_rng().gen_range(0..available_burros.len());

            burros.push(BurroState {
                selected_burro: available_burros.remove(index),
                score: 0,
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

#[derive(Default)]
pub struct BurroState {
    pub selected_burro: usize,
    pub score: usize,
    pub is_bot: bool,
    pub hearts: Vec<Entity>,
}

#[derive(Component, Clone, Copy)]
pub struct BurroCharacter {
    pub player: usize,
    pub is_playing: bool,
    pub has_picked: bool,
    pub selected_burro: usize,
    pub action_cooldown: f32,
}
