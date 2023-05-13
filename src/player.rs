use crate::{
    direction,
    AppState,assets,
    ZeroSignum,
    bullet,
};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::{TAU, PI};
use std::collections::HashMap;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<PlayerMoveEvent>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    pub random: f32,
}

impl Player {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Player {
            speed: 60.0,
            rotation_speed: 1.0,
            friction: 0.01,
            velocity: Vec3::ZERO,
            random: rng.gen_range(0.5..1.0),
        }
    }
}

pub enum Movement {
    Normal(direction::Direction),
    Jump,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,

    ActionUp,
    ActionDown,
    ActionRight,
    ActionLeft,
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

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn new() -> Self {
        PlayerBundle {
            player: Player::new(),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
        }
    }

    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.set_gamepad(Gamepad { id: 0 });

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(KeyCode::Z, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::Q, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Actions
        input_map.insert(KeyCode::J, ActionLeft);
        input_map.insert(GamepadButtonType::West, ActionLeft);

        input_map.insert(KeyCode::L, ActionRight);
        input_map.insert(GamepadButtonType::East, ActionRight);

        input_map.insert(KeyCode::I, ActionUp);
        input_map.insert(GamepadButtonType::North, ActionUp);

        input_map.insert(KeyCode::K, ActionDown);
        input_map.insert(GamepadButtonType::South, ActionDown);

//      input_map.insert(KeyCode::Space, Action);
//      input_map.insert(KeyCode::Return, Action);

        input_map
    }
}

pub struct PlayerMoveEvent {
    pub entity: Entity,
    pub movement: Movement,
    pub facing: Option::<Quat>,
}

pub fn handle_input(
    mut app_state: ResMut<State<AppState>>,
    mut players: Query<(Entity, &ActionState<PlayerAction>, &Transform, &mut Player, &mut Velocity)>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut bullet_event_writer: EventWriter<bullet::BulletEvent>,
) {
    for (entity, action_state, transform, mut player, mut velocity) in &mut players {
        //println!("T: {:?}", transform.translation);
        let mut direction = direction::Direction::NEUTRAL;
        let mut facing = None;

        if action_state.pressed(PlayerAction::ActionUp) {
            facing = Some(Quat::from_axis_angle(Vec3::Y, 0.0));
            bullet_event_writer.send(bullet::BulletEvent {
                source: entity,
                speed: 12.0, //burro.bullet_speed,
                time_to_live: 3.0,//burro.bullet_time_alive,
                position: transform.translation,
                direction: Vec3::new(1.0, 0.0, 0.0),
                bullet_type: if false { //burro.is_mechaburro {
                    bullet::BulletType::Laser
                } else {
                    bullet::BulletType::Candy
                },
            });
        } else if action_state.pressed(PlayerAction::ActionDown) {
            facing = Some(Quat::from_axis_angle(Vec3::Y, PI));
        } else if action_state.pressed(PlayerAction::ActionLeft) {
            facing = Some(Quat::from_axis_angle(Vec3::Y, PI / 2.0));
        } else if action_state.pressed(PlayerAction::ActionRight) {
            facing = Some(Quat::from_axis_angle(Vec3::Y, (3.0 * PI) / 2.0));
        }

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(input_direction) {
                direction += input_direction.direction();
            }
        }

        player_move_event_writer.send(PlayerMoveEvent {
            entity,
            movement: Movement::Normal(direction),
            facing,
        });
    }
}

pub fn move_player(
    time: Res<Time>,
    mut players: Query<(Entity, &mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut Transform, &mut Player, &mut Velocity)>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    game_assets: Res<assets::GameAssets>,
//    mut audio: audio::GameAudio,
) {
    let mut move_events = HashMap::new();
    let mut facing = None;
    for move_event in player_move_event_reader.iter() {
        facing = move_event.facing;
        move_events.entry(move_event.entity).or_insert(move_event);
    }

    for (entity, mut controller, controller_output, mut transform, mut player, p_velocity) in players.iter_mut() {
        //transform.rotate_z(time.delta_seconds());

        let speed: f32 = player.speed;
        let rotation_speed: f32 = player.rotation_speed;
        let friction: f32 = player.friction;
        let mut gravity: Vec3 = 3.0 * Vec3::new(0.0, -1.0, 0.0);

        player.velocity *= friction.powf(time.delta_seconds());
//        player.velocity += (Vec3::X * speed) * time.delta_seconds();

        if let Some(move_event) = move_events.get(&entity) {
            match move_event.movement {
                Movement::Normal(direction) => {
                    let mut acceleration = Vec3::from(direction).zero_signum();
                    if !controller_output.grounded {
                        acceleration.z *= 0.5;
                    }
                    player.velocity += (acceleration * speed) * time.delta_seconds();
                },
                _ => ()
            }
        }

        player.velocity = player.velocity.clamp_length_max(speed);

        let new_translation = (gravity + player.velocity) * time.delta_seconds();
        let new_position = new_translation + transform.translation;

        let angle = (-(new_position.z - transform.translation.z))
            .atan2(new_position.x - transform.translation.x);
        let rotation = Quat::from_axis_angle(Vec3::Y, angle);
//       velocity.angvel = rotation.to_scaled_axis();
        controller.translation = Some(new_translation);
//        velocity.linvel = player.velocity * time.delta_seconds();

//        transform.translation.x = 0.0; // hardcoding for now

        let new_rotation = transform
            .rotation
            .lerp(Quat::from_axis_angle(Vec3::Y, TAU * 0.75), time.delta_seconds() * rotation_speed);

        // don't rotate if we're not moving or if rotation isnt a number
        if let Some(facing) = facing { 
            println!("ahp");
            transform.rotation = facing;
        } else if !rotation.is_nan() && player.velocity.length() > 0.1 {
            transform.rotation = rotation;
        }
    }
}

