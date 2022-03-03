use crate::{
    assets::GameAssets, bullet::BulletEvent, bullet::BulletType, burro, collision, direction,
    game_state, AppState,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashMap;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<PlayerMoveEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_input.label("handle_input"))
                    .with_system(move_player.after("handle_input")),
            );
    }
}

trait ZeroSignum {
    fn zero_signum(&self) -> Vec3;
}

impl ZeroSignum for Vec3 {
    fn zero_signum(&self) -> Vec3 {
        let convert = |n| {
            if n < 0.1 && n > -0.1 {
                0.0
            } else if n > 0.0 {
                1.0
            } else {
                -1.0
            }
        };

        Vec3::new(convert(self.x), convert(self.y), convert(self.z)).normalize()
    }
}

fn move_player(
    time: Res<Time>,
    mut player_transform: Query<(Entity, &mut Transform, &mut Player, &burro::Burro)>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    collidables: Res<collision::Collidables>,
) {
    let mut move_events = HashMap::new();
    for move_event in player_move_event_reader.iter() {
        move_events.entry(move_event.entity).or_insert(move_event);
    }

    for (entity, mut transform, mut player, burro) in player_transform.iter_mut() {
        let speed: f32 = player.speed;
        let rotation_speed: f32 = player.rotation_speed;
        let friction: f32 = player.friction;

        player.velocity *= friction.powf(time.delta_seconds());
        if let Some(move_event) = move_events.get(&entity) {
            let acceleration = Vec3::from(move_event.direction);
            player.velocity += (acceleration.zero_signum() * speed) * time.delta_seconds();
        }

        player.velocity = player.velocity.clamp_length_max(speed);

        let mut new_translation = transform.translation + player.velocity;
        collidables.fit_in(
            &transform.translation,
            &mut new_translation,
            &mut player.velocity,
        );

        let angle = (-(new_translation.z - transform.translation.z))
            .atan2(new_translation.x - transform.translation.x);
        let rotation = Quat::from_axis_angle(Vec3::Y, angle);
        transform.translation = new_translation;

        if player.velocity.length() > 0.01 {
            let bobbing_velocity = (time.seconds_since_startup() as f32
                * (2.0 * std::f32::consts::PI)
                * 4.0
                * burro.random)
                .sin() as f32;
            transform.translation.y += bobbing_velocity * (time.delta_seconds() * 4.0);
        //          transform.rotate(Quat::from_rotation_x(
        //              bobbing_velocity * (time.delta_seconds() * 8.0),
        //          ));
        } else {
            transform.translation.y += -player.speed * time.delta_seconds(); // gravity
        }
        transform.translation.y = transform.translation.y.clamp(1.0, 1.5);

        let new_rotation = transform
            .rotation
            .lerp(rotation, time.delta_seconds() * rotation_speed);

        // don't rotate if we're not moving or if uhh rotation isnt a number?? why isn't it a number? who did this
        if !new_rotation.is_nan() && player.velocity.length() > 0.0001 && !player.is_firing {
            transform.rotation = rotation;
        }

        // make the burros all squishy like
        if transform.scale.x != 1.0 || transform.scale.y != 1.0 {
            let new_scale = transform
                .scale
                .lerp(Vec3::new(1.0, 1.0, 1.0), time.delta_seconds() * 4.0);
            if new_scale.is_nan() || transform.scale.distance(new_scale) < 0.0001 {
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
            } else {
                transform.scale = new_scale;
            }
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,

    ActionUp,
    ActionDown,
    ActionLeft,
    ActionRight,
    Pause,

    Debug,
}

impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> direction::Direction {
        match self {
            PlayerAction::Up => direction::Direction::UP,
            PlayerAction::Down => direction::Direction::DOWN,
            PlayerAction::Left => direction::Direction::LEFT,
            PlayerAction::Right => direction::Direction::RIGHT,
            _ => direction::Direction::NEUTRAL,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
    pub is_firing: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            velocity: Vec3::default(),
            speed: 0.8,
            rotation_speed: 1.0,
            friction: 0.01,
            is_firing: false,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    burro: burro::Burro,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(burro_skin: game_state::BurroSkin) -> Self {
        PlayerBundle {
            player: Player::new(),
            burro: burro::Burro::new(burro_skin),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
        }
    }
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.set_gamepad(Gamepad(0));

        // Movement
        input_map.insert(Up, KeyCode::Up);
        input_map.insert(Up, KeyCode::W);
        input_map.insert(Up, GamepadButtonType::DPadUp);

        input_map.insert(Down, KeyCode::Down);
        input_map.insert(Down, KeyCode::S);
        input_map.insert(Down, GamepadButtonType::DPadDown);

        input_map.insert(Left, KeyCode::Left);
        input_map.insert(Left, KeyCode::A);
        input_map.insert(Left, GamepadButtonType::DPadLeft);

        input_map.insert(Right, KeyCode::Right);
        input_map.insert(Right, KeyCode::D);
        input_map.insert(Right, GamepadButtonType::DPadRight);

        // Actions
        input_map.insert(ActionUp, KeyCode::I);
        input_map.insert(ActionUp, GamepadButtonType::North);

        input_map.insert(ActionDown, KeyCode::K);
        input_map.insert(ActionDown, GamepadButtonType::South);

        input_map.insert(ActionLeft, KeyCode::J);
        input_map.insert(ActionLeft, GamepadButtonType::West);

        input_map.insert(ActionRight, KeyCode::L);
        input_map.insert(ActionRight, GamepadButtonType::East);

        // Other
        input_map.insert(Pause, KeyCode::Escape);
        input_map.insert(Debug, KeyCode::X);

        input_map
    }
}

pub struct PlayerMoveEvent {
    pub entity: Entity,
    pub direction: direction::Direction,
}

fn handle_input(
    mut app_state: ResMut<State<AppState>>,
    mut player: Query<(
        Entity,
        &ActionState<PlayerAction>,
        &mut Transform,
        &mut burro::Burro,
        &mut Player,
        &mut Handle<StandardMaterial>,
    )>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut bullet_event_writer: EventWriter<BulletEvent>,
    //    game_assets: Res<GameAssets>,
    //    mut skin_index: Local<usize>,
    mut commands: Commands,
    mut burros: Query<Entity, With<burro::Burro>>,
    mut burro_death_event_writer: EventWriter<burro::BurroDeathEvent>,
) {
    for (entity, action_state, mut transform, mut burro, mut player, mut material) in
        player.iter_mut()
    {
        let mut direction = direction::Direction::NEUTRAL;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                direction += input_direction.direction();
            }
        }

        if direction != direction::Direction::NEUTRAL {
            player_move_event_writer.send(PlayerMoveEvent { entity, direction });
        }

        if action_state.just_pressed(&PlayerAction::Pause) {
            app_state.push(AppState::Pause).unwrap();
        }

        if action_state.just_pressed(&PlayerAction::Debug) {
            let entity = burros.iter().last().unwrap();
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Pinata,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Meow,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Salud,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Mexico,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Medianoche,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Morir,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Gators,
            });
            burro_death_event_writer.send(burro::BurroDeathEvent {
                entity,
                skin: game_state::BurroSkin::Aguas,
            });

            ///// changing skin
            //          *skin_index += 1;
            //          if *skin_index > 7 {
            //              *skin_index = 0;
            //          }

            //          let skin = match *skin_index {
            //              1 => &game_assets.meow_texture.material,
            //              2 => &game_assets.salud_texture.material,
            //              3 => &game_assets.mexico_texture.material,
            //              4 => &game_assets.medianoche_texture.material,
            //              5 => &game_assets.morir_texture.material,
            //              6 => &game_assets.gators_texture.material,
            //              7 => &game_assets.aguas_texture.material,
            //              8 => &game_assets.mechaburro_texture.material,
            //              _ => &game_assets.pinata_texture.material,
            //          };
            //          *material = skin.clone();
        }

        if burro.can_fire() {
            use std::f32::consts::PI;

            player.is_firing = false;
            if action_state.pressed(&PlayerAction::ActionUp) {
                bullet_event_writer.send(BulletEvent {
                    source: entity,
                    speed: burro.bullet_speed,
                    time_to_live: burro.bullet_time_alive,
                    position: transform.translation,
                    direction: Vec3::new(1.0, 0.0, 0.0),
                    bullet_type: if burro.is_mechaburro {
                        BulletType::Laser
                    } else {
                        BulletType::Candy
                    },
                });
                burro.fire();
                player.is_firing = true;
                transform.rotation = Quat::from_axis_angle(Vec3::Y, 0.0);
                transform.scale = Vec3::new(0.7, 1.4, 1.0);
            }

            if action_state.pressed(&PlayerAction::ActionDown) {
                bullet_event_writer.send(BulletEvent {
                    source: entity,
                    speed: burro.bullet_speed,
                    time_to_live: burro.bullet_time_alive,
                    position: transform.translation,
                    direction: Vec3::new(-1.0, 0.0, 0.0),
                    bullet_type: if burro.is_mechaburro {
                        BulletType::Laser
                    } else {
                        BulletType::Candy
                    },
                });
                burro.fire();
                player.is_firing = true;
                transform.rotation = Quat::from_axis_angle(Vec3::Y, PI);
                transform.scale = Vec3::new(0.7, 1.4, 1.0);
            }

            if action_state.pressed(&PlayerAction::ActionLeft) {
                bullet_event_writer.send(BulletEvent {
                    source: entity,
                    speed: burro.bullet_speed,
                    time_to_live: burro.bullet_time_alive,
                    position: transform.translation,
                    direction: Vec3::new(0.0, 0.0, -1.0),
                    bullet_type: if burro.is_mechaburro {
                        BulletType::Laser
                    } else {
                        BulletType::Candy
                    },
                });
                burro.fire();
                player.is_firing = true;
                transform.rotation = Quat::from_axis_angle(Vec3::Y, PI / 2.0);
                transform.scale = Vec3::new(0.7, 1.4, 1.0);
            }

            if action_state.pressed(&PlayerAction::ActionRight) {
                bullet_event_writer.send(BulletEvent {
                    source: entity,
                    speed: burro.bullet_speed,
                    time_to_live: burro.bullet_time_alive,
                    position: transform.translation,
                    direction: Vec3::new(0.0, 0.0, 1.0),
                    bullet_type: if burro.is_mechaburro {
                        BulletType::Laser
                    } else {
                        BulletType::Candy
                    },
                });
                burro.fire();
                player.is_firing = true;
                transform.rotation = Quat::from_axis_angle(Vec3::Y, (3.0 * PI) / 2.0);
                transform.scale = Vec3::new(0.7, 1.4, 1.0);
            }
        }
    }
}
