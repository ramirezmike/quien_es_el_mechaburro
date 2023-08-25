use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    Back,
}

pub(crate) trait InputCommandsExt {
    fn spawn_menu_input(&mut self, cleanup_marker: impl Component);
}

impl<'w, 's> InputCommandsExt for Commands<'w, 's> {
    fn spawn_menu_input(&mut self, cleanup_marker: impl Component) {
        self.spawn((
            InputManagerBundle::<MenuAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new(KEYBOARD_INPUTS).insert_multiple(GAMEPAD_INPUTS).build(),
            },
            cleanup_marker,
        ));
    }
}

pub fn create_menu_input_for_player(player: usize) -> impl Bundle {
    let mut input_map = InputMap::new(GAMEPAD_INPUTS);
    if player == 0 || cfg!(feature = "debug") {
        input_map.insert_multiple(KEYBOARD_INPUTS);
    }
    InputManagerBundle::<MenuAction> {
        action_state: ActionState::default(),
        input_map: input_map
            .set_gamepad(Gamepad { id: player })
            .build(),
    }
}

const KEYBOARD_INPUTS: [(KeyCode, MenuAction); 12] = [
    (KeyCode::Space, MenuAction::Select),
    (KeyCode::Return, MenuAction::Select),
    (KeyCode::Return, MenuAction::Start),
    (KeyCode::X, MenuAction::Back),
    (KeyCode::Up, MenuAction::Up),
    (KeyCode::Down, MenuAction::Down),
    (KeyCode::Right, MenuAction::Right),
    (KeyCode::Left, MenuAction::Left),
    (KeyCode::W, MenuAction::Up),
    (KeyCode::S, MenuAction::Down),
    (KeyCode::A, MenuAction::Right),
    (KeyCode::D, MenuAction::Left),
];

const GAMEPAD_INPUTS: [(GamepadButtonType, MenuAction); 7] = [
    (GamepadButtonType::DPadUp, MenuAction::Up),
    (GamepadButtonType::DPadDown, MenuAction::Down),
    (GamepadButtonType::DPadRight, MenuAction::Right),
    (GamepadButtonType::DPadLeft, MenuAction::Left),

    (GamepadButtonType::South, MenuAction::Select),
    (GamepadButtonType::East, MenuAction::Back),
    (GamepadButtonType::Start, MenuAction::Start),
];
