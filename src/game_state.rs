use crate::{assets, config, AppState};
use bevy::prelude::*;
use rand::Rng;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default());
    }
}

#[derive(Default, Resource)]
pub struct GameState {
    pub burros: Vec<BurroState>,
    pub dead_burros: Vec<usize>,
    pub current_level: usize,
    pub current_level_over: bool,
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
        mut burros: Vec<BurroState>,
        number_of_bots: usize,
        difficulty: f32,
        burro_assets: &Vec<assets::BurroAsset>,
    ) -> Self {
        let mut available_burros: Vec<usize> = (0..burro_assets.len()).collect();

        let claimed_burros: Vec<usize> = burros.iter().map(|x| x.selected_burro).collect();
        available_burros.retain(|x| !claimed_burros.contains(&x));

        // bots
        for i in 0..number_of_bots {
            let index = rand::thread_rng().gen_range(0..available_burros.len());

            burros.push(BurroState {
                player: config::MAX_NUMBER_OF_PLAYERS as usize + i,
                selected_burro: available_burros.remove(index),
                outline_color: Color::BLACK,
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
            difficulty,
        }
    }

    pub fn on_new_level(&mut self) {
        self.dead_burros = vec![];
    }
}

#[derive(Default, Clone)]
pub struct BurroState {
    pub player: usize,
    pub selected_burro: usize,
    pub outline_color: Color,
    pub score: usize,
    pub is_bot: bool,
    pub hearts: Vec<Entity>,
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct PlayerMarker(pub usize);
