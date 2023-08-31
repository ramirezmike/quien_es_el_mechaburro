use crate::{burro, floor, game_state, player, player::PlayerAction, AppState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::cmp::Ordering;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Bot {
    heading: Option<Cardinal>,
    shooting: Option<Cardinal>,
    mind_cooldown: f32,
    target: Option<Vec2>,
    previous_distance: f32,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            heading: None,
            shooting: None,
            mind_cooldown: 0.0,
            target: None,
            previous_distance: 0.,
        }
    }
}

impl Bot {
    pub fn can_think(&self) -> bool {
        self.mind_cooldown <= 0.0
    }
}

#[derive(Copy, Clone, Debug)]
enum Cardinal {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Bundle)]
pub struct BotBundle {
    bot: Bot,
    input_manager: InputManagerBundle<PlayerAction>,
}

impl BotBundle {
    pub fn new() -> Self {
        BotBundle {
            bot: Bot::default(),
            input_manager: InputManagerBundle {
                input_map: InputMap::default(),
                action_state: ActionState::default(),
            },
        }
    }
}

pub fn update_bot_ai(
    time: Res<Time>,
    mut bots: Query<(Entity, &mut Bot, &burro::Burro, &Transform)>,
    other_burros: Query<(Entity, &Transform, &burro::Burro)>,
    floor_manager: Res<floor::FloorManager>,
) {
    for (entity, mut bot, burro, transform) in bots.iter_mut() {
        // handling mind cool down
        bot.mind_cooldown -= time.delta_seconds();
        bot.mind_cooldown = bot.mind_cooldown.clamp(-10.0, 30.0);

        if !bot.can_think() {
            continue;
        }

        let burro_position = Vec2::new(transform.translation.x, transform.translation.z);
        let burro_x = transform.translation.x as isize;
        let burro_z = transform.translation.z as isize;
        let burro_fx = transform.translation.x;
        let burro_fz = transform.translation.z;

        // uh this works sorta so I'm ok with it

        if let Some(target) = bot.target {
            let distance = burro_position.distance(target);
            let is_bot_stuck = (bot.previous_distance - distance).abs() < 0.001;
            if distance < 0.5 || is_bot_stuck {
                bot.target = None;
                bot.mind_cooldown = 0.0;
                continue;
            }

            bot.previous_distance = distance;

            let x_diff = burro_position.x - target.x;
            let z_diff = burro_position.y - target.y;
            let buffer = 0.5;

            if x_diff.abs() - z_diff.abs() < 0.1 {
                // go diagonal
                if burro_position.x > target.x
                    && floor_manager.is_walkable(burro_fx - buffer, burro_fz)
                {
                    if burro_position.y > target.y {
                        if floor_manager.is_walkable(burro_fx, burro_fz - buffer) {
                            bot.heading = Some(Cardinal::SW);
                        } else {
                            bot.target = None;
                            bot.mind_cooldown = 0.0;
                        }
                    } else if floor_manager.is_walkable(burro_fx, burro_fz + buffer) {
                        bot.heading = Some(Cardinal::SE);
                    } else {
                        bot.target = None;
                        bot.mind_cooldown = 0.0;
                    }
                } else if floor_manager.is_walkable(burro_fx + buffer, burro_fz) {
                    if burro_position.y > target.y {
                        if floor_manager.is_walkable(burro_fx, burro_fz - buffer) {
                            bot.heading = Some(Cardinal::NW);
                        } else {
                            bot.target = None;
                            bot.mind_cooldown = 0.0;
                        }
                    } else if floor_manager.is_walkable(burro_fx, burro_fz + buffer) {
                        bot.heading = Some(Cardinal::NE);
                    } else {
                        bot.target = None;
                        bot.mind_cooldown = 0.0;
                    }
                } else {
                    bot.target = None;
                    bot.mind_cooldown = 0.0;
                }
            } else if x_diff.abs() > z_diff.abs() {
                // go vertical
                if burro_position.x > target.x {
                    if floor_manager.is_walkable(burro_fx - buffer, burro_fz) {
                        bot.heading = Some(Cardinal::S);
                    } else {
                        bot.target = None;
                        bot.mind_cooldown = 0.0;
                    }
                } else if floor_manager.is_walkable(burro_fx + buffer, burro_fz) {
                    bot.heading = Some(Cardinal::N);
                } else {
                    bot.target = None;
                    bot.mind_cooldown = 0.0;
                }
            } else {
                // go horizontal
                if burro_position.y > target.y {
                    if floor_manager.is_walkable(burro_fx, burro_fz - buffer) {
                        bot.heading = Some(Cardinal::W);
                    } else {
                        bot.target = None;
                        bot.mind_cooldown = 0.0;
                    }
                } else if floor_manager.is_walkable(burro_fx, burro_fz + buffer) {
                    bot.heading = Some(Cardinal::E);
                } else {
                    bot.target = None;
                    bot.mind_cooldown = 0.0;
                }
            }
        } else {
            bot.target = floor_manager.get_random_spot();
        }

        let mut other_burros: Vec<_> = other_burros
            .iter()
            .filter(|(other_entity, _, other_burro)| {
                entity != *other_entity && other_burro.can_be_hit()
            }) // skip yourself and burros that can't be hit
            .map(|other| {
                let position = Vec2::new(other.1.translation.x, other.1.translation.z);
                (position.distance(burro_position), other)
            })
            .collect();

        other_burros.sort_by_key(|o| o.0 as usize); // sort by distance to self

        for (_, (other_entity, other_burro_transform, _)) in other_burros.iter().rev() {
            if entity == *other_entity {
                continue;
            }

            let other_x = other_burro_transform.translation.x as isize;
            let other_z = other_burro_transform.translation.z as isize;

            bot.shooting = None;
            if burro.can_fire() {
                // shoot left or right to try to hit a burro
                if burro_x == other_x {
                    bot.shooting = match burro_z.cmp(&other_z) {
                        Ordering::Greater => Some(Cardinal::W),
                        Ordering::Less => Some(Cardinal::E),
                        Ordering::Equal => bot.shooting,
                    };
                }

                // shoot up or down to try to hit a burro
                if burro_z == other_z {
                    bot.shooting = match burro_x.cmp(&other_x) {
                        Ordering::Greater => Some(Cardinal::S),
                        Ordering::Less => Some(Cardinal::N),
                        Ordering::Equal => bot.shooting,
                    };
                }
            }
        }
    }
}

pub fn update_virtual_controllers(mut bots: Query<(Entity, &Bot, &mut ActionState<PlayerAction>)>) {
    for (_, bot, mut action_state) in bots.iter_mut() {
        // release all buttons
        // this probably affects durations but for
        // this game it might not be a big deal
        action_state.release(PlayerAction::Up);
        action_state.release(PlayerAction::Down);
        action_state.release(PlayerAction::Left);
        action_state.release(PlayerAction::Right);

        action_state.release(PlayerAction::ActionUp);
        action_state.release(PlayerAction::ActionDown);
        action_state.release(PlayerAction::ActionLeft);
        action_state.release(PlayerAction::ActionRight);

        if let Some(cardinal) = &bot.heading {
            match cardinal {
                Cardinal::N => action_state.press(PlayerAction::Up),
                Cardinal::S => action_state.press(PlayerAction::Down),
                Cardinal::W => action_state.press(PlayerAction::Left),
                Cardinal::E => action_state.press(PlayerAction::Right),
                Cardinal::NE => {
                    action_state.press(PlayerAction::Up);
                    action_state.press(PlayerAction::Right);
                }
                Cardinal::NW => {
                    action_state.press(PlayerAction::Up);
                    action_state.press(PlayerAction::Left);
                }
                Cardinal::SE => {
                    action_state.press(PlayerAction::Down);
                    action_state.press(PlayerAction::Right);
                }
                Cardinal::SW => {
                    action_state.press(PlayerAction::Down);
                    action_state.press(PlayerAction::Left);
                }
            }
        }

        if let Some(cardinal) = &bot.shooting {
            match cardinal {
                Cardinal::N => action_state.press(PlayerAction::ActionUp),
                Cardinal::S => action_state.press(PlayerAction::ActionDown),
                Cardinal::W => action_state.press(PlayerAction::ActionLeft),
                Cardinal::E => action_state.press(PlayerAction::ActionRight),
                _ => (),
            }
        }
    }
}
