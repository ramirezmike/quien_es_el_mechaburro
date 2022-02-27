use crate::{burro, player, player::PlayerAction};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bot_ai.label("ai")).add_system(
            update_virtual_controllers
                .after("ai")
                .before("handle_input"),
        );
    }
}

#[derive(Component)]
pub struct Bot {
    heading: Option<Cardinal>,
    shooting: Option<Cardinal>,
    mind_cooldown: f32,
    target: Option<Vec2>,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            heading: None,
            shooting: None,
            mind_cooldown: 1.0,
            target: None,
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

fn update_bot_ai(
    time: Res<Time>,
    mut bots: Query<(Entity, &mut Bot, &burro::Burro, &Transform)>,
    other_burros: Query<(Entity, &Transform), With<burro::Burro>>,
) {
    let mut rng = thread_rng();
    let cardinals = [
        Cardinal::N,
        Cardinal::S,
        Cardinal::E,
        Cardinal::W,
        Cardinal::NE,
        Cardinal::NW,
        Cardinal::SE,
        Cardinal::SW,
    ];

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

        if let Some(target) = bot.target {
            if burro_position.distance(target) < 0.5 {
                bot.target = None;
                bot.mind_cooldown = 1.0;
                continue;
            }

            let x_diff = burro_position.x - target.x;
            let z_diff = burro_position.y - target.y;

            if x_diff.abs() - z_diff.abs() < 0.1 {
                // go diagonal
                if burro_position.x > target.x {
                    if burro_position.y > target.y {
                        bot.heading = Some(Cardinal::SW);
                    } else {
                        bot.heading = Some(Cardinal::SE);
                    }
                } else {
                    if burro_position.y > target.y {
                        bot.heading = Some(Cardinal::NW);
                    } else {
                        bot.heading = Some(Cardinal::NE);
                    }
                }
            } else if x_diff.abs() > z_diff.abs() {
                // go vertical
                if burro_position.x > target.x {
                    bot.heading = Some(Cardinal::S);
                } else {
                    bot.heading = Some(Cardinal::N);
                }
            } else {
                // go horizontal
                if burro_position.y > target.y {
                    bot.heading = Some(Cardinal::W);
                } else {
                    bot.heading = Some(Cardinal::E);
                }
            }
        } else {
            bot.target = Some(get_random_position());
            //println!("New target: {:?}", bot.target);
        }

        let mut other_burros: Vec<_> = other_burros
            .iter()
            .filter(|(other_entity, _)| entity != *other_entity) // skip yourself
            .map(|other| {
                let position = Vec2::new(other.1.translation.x, other.1.translation.z);
                (position.distance(burro_position), other)
            })
            .collect();

        other_burros.sort_by_key(|o| o.0 as usize); // sort by distance to self

        for (_, (other_entity, other_burro_transform)) in other_burros.iter().rev() {
            if entity == *other_entity {
                continue;
            }

            let other_x = other_burro_transform.translation.x as isize;
            let other_z = other_burro_transform.translation.z as isize;
            let other_fx = other_burro_transform.translation.x;
            let other_fz = other_burro_transform.translation.z;

            bot.shooting = None;

            // shoot left or right to try to hit a burro
            if burro_x == other_x {
                if burro_z > other_z {
                    bot.shooting = Some(Cardinal::W);
                } else if burro_z < other_z {
                    bot.shooting = Some(Cardinal::E);
                }
            }

            // shoot up or down to try to hit a burro
            if burro_z == other_z {
                if burro_x > other_x {
                    bot.shooting = Some(Cardinal::S);
                } else if burro_x < other_x {
                    bot.shooting = Some(Cardinal::N);
                }
            }

            //          println!(
            //              "Shooting {:?} because Me{} {} You {} {}",
            //              bot.shooting, burro_x, burro_z, other_x, other_z
            //          );
        }
    }
}

fn get_random_position() -> Vec2 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let x: f32 = rng.gen_range(-15.0..15.0);
    let z: f32 = rng.gen_range(-15.0..15.0);

    Vec2::new(x, z)
}

/*
        let mut other_burros: Vec<_> = other_burros
            .iter()
            .filter(|(other_entity, _)| entity != *other_entity) // skip yourself
            .map(|other| {
                let position = Vec2::new(other.1.translation.x, other.1.translation.z);
                (position.distance(burro_position), other)
            })
            .collect();

        other_burros.sort_by_key(|o| o.0 as usize); // sort by distance to self

        for (_, (other_entity, other_burro_transform)) in other_burros.iter().rev() {
            if entity == *other_entity {
                continue;
            }

            let other_x = other_burro_transform.translation.x as isize;
            let other_z = other_burro_transform.translation.z as isize;
            let other_fx = other_burro_transform.translation.x;
            let other_fz = other_burro_transform.translation.z;

            bot.shooting = None;

            // shoot left or right to try to hit a burro
            if burro_x == other_x {
                if burro_z > other_z {
                    bot.shooting = Some(Cardinal::W);
                } else if burro_z < other_z {
                    bot.shooting = Some(Cardinal::E);
                }
            }

            // shoot up or down to try to hit a burro
            if burro_z == other_z {
                if burro_x > other_x {
                    bot.shooting = Some(Cardinal::S);
                } else if burro_x < other_x {
                    bot.shooting = Some(Cardinal::N);
                }
            }

            //          println!(
            //              "Shooting {:?} because Me{} {} You {} {}",
            //              bot.shooting, burro_x, burro_z, other_x, other_z
            //          );

            if burro.full_health() {
                // try to hunt?
                if burro_fx <= other_fx && burro_z == other_z {
                    bot.heading = Some(Cardinal::N);
                } else if burro_fx > other_fx && burro_z == other_z {
                    bot.heading = Some(Cardinal::S);
                } else if burro_fz <= other_fz && burro_x == other_x {
                    bot.heading = Some(Cardinal::E);
                } else if burro_fz > other_fz && burro_x == other_x {
                    bot.heading = Some(Cardinal::W);
                } else if burro_fx <= other_fx && burro_fz > other_fz {
                    bot.heading = Some(Cardinal::NW);
                } else if burro_fx <= other_fx && burro_fz < other_fz {
                    bot.heading = Some(Cardinal::NE);
                } else if burro_fx > other_fx && burro_fz > other_fz {
                    bot.heading = Some(Cardinal::SW);
                } else if burro_fx > other_fx && burro_fz < other_fz {
                    bot.heading = Some(Cardinal::SE);
                }
            } else if burro.almost_dead() {
                // try to avoid?
                if burro_fx <= other_fx && burro_z == other_z {
                    bot.heading = Some(Cardinal::S);
                } else if burro_fx > other_fx && burro_z == other_z {
                    bot.heading = Some(Cardinal::N);
                } else if burro_fz <= other_fz && burro_x == other_x {
                    bot.heading = Some(Cardinal::W);
                } else if burro_fz > other_fz && burro_x == other_x {
                    bot.heading = Some(Cardinal::E);
                } else if burro_fx <= other_fx && burro_fz > other_fz {
                    bot.heading = Some(Cardinal::SE);
                } else if burro_fx <= other_fx && burro_fz < other_fz {
                    bot.heading = Some(Cardinal::SW);
                } else if burro_fx > other_fx && burro_fz > other_fz {
                    bot.heading = Some(Cardinal::NE);
                } else if burro_fx > other_fx && burro_fz < other_fz {
                    bot.heading = Some(Cardinal::NW);
                }
            } else {
                // random?
                bot.heading = cardinals.choose(&mut rng).copied();
            }
        }
    }

*/

fn update_virtual_controllers(mut bots: Query<(Entity, &Bot, &mut ActionState<PlayerAction>)>) {
    for (_, bot, mut action_state) in bots.iter_mut() {
        // release all buttons
        // this probably affects durations but for
        // this game it might not be a big deal
        action_state.release(&PlayerAction::Up);
        action_state.release(&PlayerAction::Down);
        action_state.release(&PlayerAction::Left);
        action_state.release(&PlayerAction::Right);

        action_state.release(&PlayerAction::ActionUp);
        action_state.release(&PlayerAction::ActionDown);
        action_state.release(&PlayerAction::ActionLeft);
        action_state.release(&PlayerAction::ActionRight);

        if let Some(cardinal) = &bot.heading {
            match cardinal {
                Cardinal::N => action_state.press(&PlayerAction::Up),
                Cardinal::S => action_state.press(&PlayerAction::Down),
                Cardinal::W => action_state.press(&PlayerAction::Left),
                Cardinal::E => action_state.press(&PlayerAction::Right),
                Cardinal::NE => {
                    action_state.press(&PlayerAction::Up);
                    action_state.press(&PlayerAction::Right);
                }
                Cardinal::NW => {
                    action_state.press(&PlayerAction::Up);
                    action_state.press(&PlayerAction::Left);
                }
                Cardinal::SE => {
                    action_state.press(&PlayerAction::Down);
                    action_state.press(&PlayerAction::Right);
                }
                Cardinal::SW => {
                    action_state.press(&PlayerAction::Down);
                    action_state.press(&PlayerAction::Left);
                }
            }
        }

        if let Some(cardinal) = &bot.shooting {
            match cardinal {
                Cardinal::N => action_state.press(&PlayerAction::ActionUp),
                Cardinal::S => action_state.press(&PlayerAction::ActionDown),
                Cardinal::W => action_state.press(&PlayerAction::ActionLeft),
                Cardinal::E => action_state.press(&PlayerAction::ActionRight),
                _ => (),
            }
        }
    }
}