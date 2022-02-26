use crate::{bullet::BulletEvent, collision, direction, AppState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<PlayerMoveEvent>()
            .add_system(handle_input.label("input"))
            .add_system(move_player.after("input"));
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

        Vec3::new(convert(self.x), convert(self.y), convert(self.z))
    }
}

fn move_player(
    time: Res<Time>,
    mut player_transform: Query<(&mut Transform, &mut Player)>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    collidables: Res<collision::Collidables>,
) {
    if let Ok((mut transform, mut player)) = player_transform.get_single_mut() {
        let speed: f32 = player.speed;
        let rotation_speed: f32 = player.rotation_speed;
        let friction: f32 = player.friction;

        player.velocity *= friction.powf(time.delta_seconds());
        if let Some(move_event) = player_move_event_reader.iter().last() {
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
            let bobbing_velocity =
                (time.seconds_since_startup() as f32 * (2.0 * std::f32::consts::PI) * 4.0).sin()
                    as f32;
            transform.translation.y += bobbing_velocity * (time.delta_seconds() * 4.0);
            transform.rotate(Quat::from_rotation_x(
                bobbing_velocity * (time.delta_seconds() * 8.0),
            ));
        } else {
            transform.translation.y += -player.speed * time.delta_seconds(); // gravity
        }
        transform.translation.y = transform.translation.y.clamp(0.5, 1.0);

        let new_rotation = transform
            .rotation
            .lerp(rotation, time.delta_seconds() * rotation_speed);

        // don't rotate if we're not moving or if uhh rotation isnt a number?? why isn't it a number? who did this
        if !new_rotation.is_nan() && player.velocity.length() > 0.0001 {
            transform.rotation = rotation;
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Up,
    Down,
    Left,
    Right,

    ActionUp,
    ActionDown,
    ActionLeft,
    ActionRight,
    Pause,
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
}

impl Player {
    fn new() -> Self {
        Player {
            velocity: Vec3::default(),
            speed: 0.8,
            rotation_speed: 1.0,
            friction: 0.01,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player::new(),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
        }
    }
}

impl PlayerBundle {
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

        input_map
    }
}

pub struct PlayerMoveEvent {
    pub direction: direction::Direction,
}

fn handle_input(
    mut app_state: ResMut<State<AppState>>,
    player: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut bullet_event_writer: EventWriter<BulletEvent>,
) {
    if let Ok((action_state, transform)) = player.get_single() {
        let mut direction = direction::Direction::NEUTRAL;

        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                direction += input_direction.direction();
            }
        }

        if direction != direction::Direction::NEUTRAL {
            player_move_event_writer.send(PlayerMoveEvent { direction });
        }

        if action_state.just_pressed(&PlayerAction::Pause) {
            app_state.push(AppState::Pause).unwrap();
        }

        if action_state.just_pressed(&PlayerAction::ActionUp) {
            bullet_event_writer.send(BulletEvent {
                position: transform.translation,
                direction: Vec3::new(1.0, 0.0, 0.0),
            });
        }

        if action_state.just_pressed(&PlayerAction::ActionDown) {
            bullet_event_writer.send(BulletEvent {
                position: transform.translation,
                direction: Vec3::new(-1.0, 0.0, 0.0),
            });
        }

        if action_state.just_pressed(&PlayerAction::ActionLeft) {
            bullet_event_writer.send(BulletEvent {
                position: transform.translation,
                direction: Vec3::new(0.0, 0.0, -1.0),
            });
        }

        if action_state.just_pressed(&PlayerAction::ActionRight) {
            bullet_event_writer.send(BulletEvent {
                position: transform.translation,
                direction: Vec3::new(0.0, 0.0, 1.0),
            });
        }
    }
}
