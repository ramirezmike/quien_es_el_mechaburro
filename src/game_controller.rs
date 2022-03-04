use bevy::prelude::*;
use std::collections::HashMap;

pub struct GameControllerPlugin;
impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections)
            .insert_resource(GameController::default())
            .add_system(gamepad_test);
    }
}

#[derive(Default)]
pub struct GameController {
    pub players: Vec<Gamepad>,
    pub pressed: HashMap<usize, Vec<GameButton>>,
}

fn gamepad_test(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut controllers: ResMut<GameController>,
) {
    let mut pressed: HashMap<usize, Vec<GameButton>> = HashMap::new();
    for mut gamepad in controllers.players.iter_mut() {
        let mut pressed_buttons = vec![];
        let gamepad = *gamepad;

        // The joysticks are represented using a separate axis for X and Y
        let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            // combine X and Y into one vector
            let left_stick_pos = Vec2::new(x, y);

            // implement a dead-zone to ignore small inputs
            if left_stick_pos.length() > 0.1 {
                // do something with the position of the left stick
                if x > 0.0 {
                    pressed_buttons.push(GameButton::Right);
                }
                if x < 0.0 {
                    pressed_buttons.push(GameButton::Left);
                }
                if y > 0.0 {
                    pressed_buttons.push(GameButton::Up);
                }
                if y < 0.0 {
                    pressed_buttons.push(GameButton::Down);
                }
            }
        }

        let axis_dx = GamepadAxis(gamepad, GamepadAxisType::DPadX);
        let axis_dy = GamepadAxis(gamepad, GamepadAxisType::DPadY);

        if let (Some(x), Some(y)) = (axes.get(axis_dx), axes.get(axis_dy)) {
            // combine X and Y into one vector
            let left_stick_pos = Vec2::new(x, y);

            // implement a dead-zone to ignore small inputs
            if left_stick_pos.length() > 0.1 {
                // do something with the position of the left stick
                if x > 0.0 {
                    pressed_buttons.push(GameButton::Right);
                }
                if x < 0.0 {
                    pressed_buttons.push(GameButton::Left);
                }
                if y > 0.0 {
                    pressed_buttons.push(GameButton::Up);
                }
                if y < 0.0 {
                    pressed_buttons.push(GameButton::Down);
                }
            }
        }

        let dpad_up = GamepadButton(gamepad, GamepadButtonType::DPadUp);
        let dpad_down = GamepadButton(gamepad, GamepadButtonType::DPadDown);
        let dpad_left = GamepadButton(gamepad, GamepadButtonType::DPadLeft);
        let dpad_right = GamepadButton(gamepad, GamepadButtonType::DPadRight);

        if buttons.pressed(dpad_up) {
            pressed_buttons.push(GameButton::Up);
        }

        if buttons.pressed(dpad_down) {
            pressed_buttons.push(GameButton::Down);
        }

        if buttons.pressed(dpad_left) {
            pressed_buttons.push(GameButton::Left);
        }

        if buttons.pressed(dpad_right) {
            pressed_buttons.push(GameButton::Right);
        }

        let south = GamepadButton(gamepad, GamepadButtonType::South);
        let east = GamepadButton(gamepad, GamepadButtonType::East);
        let west = GamepadButton(gamepad, GamepadButtonType::West);
        let north = GamepadButton(gamepad, GamepadButtonType::North);

        if buttons.just_pressed(south) {
            pressed_buttons.push(GameButton::ActionDown);
        }
        if buttons.just_pressed(north) {
            pressed_buttons.push(GameButton::ActionUp);
        }
        if buttons.just_pressed(west) {
            pressed_buttons.push(GameButton::ActionLeft);
        }
        if buttons.just_pressed(east) {
            pressed_buttons.push(GameButton::ActionRight);
        }

        let start_button = GamepadButton(gamepad, GamepadButtonType::Start);
        if buttons.just_pressed(start_button) {
            pressed_buttons.push(GameButton::Start);
        }

        let game_id = gamepad.0;
        pressed.insert(game_id, pressed_buttons);
    }

    controllers.pressed = pressed;
}

pub fn gamepad_connections(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut controllers: ResMut<GameController>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                controllers.players.push(*id);
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameButton {
    Up,
    Down,
    Left,
    Right,
    ActionUp,
    ActionLeft,
    ActionRight,
    ActionDown,
    Nothing,
    Start,
}
