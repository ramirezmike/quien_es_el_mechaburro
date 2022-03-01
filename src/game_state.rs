use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default());
    }
}

#[derive(Default)]
pub struct GameState {
    pub burros: Vec<BurroState>,
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
            });
        }

        // bots
        for i in 0..bot_count {
            burros.push(BurroState {
                score: 0,
                skin: skins[burro_count - bot_count + i],
                is_bot: true,
            });
        }

        GameState { burros }
    }
}

#[derive(Clone, Copy, PartialEq)]
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
}
