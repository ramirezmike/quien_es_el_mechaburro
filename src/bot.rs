use crate::{burro, player, player::PlayerAction};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bots);
    }
}

#[derive(Component, Default)]
pub struct Bot {}

#[derive(Bundle)]
pub struct BotBundle {
    player: player::Player,
    burro: burro::Burro,
    bot: Bot,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for BotBundle {
    fn default() -> Self {
        BotBundle {
            player: player::Player::new(),
            burro: burro::Burro::default(),
            bot: Bot::default(),
            input_manager: InputManagerBundle {
                input_map: InputMap::default(),
                action_state: ActionState::default(),
            },
        }
    }
}

fn move_bots(mut bots: Query<(Entity, &Bot, &mut ActionState<PlayerAction>)>) {
    for (entity, bot, mut action_state) in bots.iter_mut() {
        action_state.press(&PlayerAction::Right);
    }
}
