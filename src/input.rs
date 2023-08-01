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
    Select,
}

pub(crate) trait InputCommandsExt {
    fn spawn_menu_input(&mut self, cleanup_marker: impl Component);
}

impl<'w, 's> InputCommandsExt for Commands<'w, 's> {
    fn spawn_menu_input(&mut self, cleanup_marker: impl Component) {
        self.spawn((
            InputManagerBundle::<MenuAction> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::Space, MenuAction::Select),
                    (KeyCode::Return, MenuAction::Select),
                    (KeyCode::Up, MenuAction::Up),
                    (KeyCode::Down, MenuAction::Down),
                    (KeyCode::Right, MenuAction::Right),
                    (KeyCode::Left, MenuAction::Left),
                    (KeyCode::W, MenuAction::Up),
                    (KeyCode::S, MenuAction::Down),
                    (KeyCode::A, MenuAction::Right),
                    (KeyCode::D, MenuAction::Left),
                ]),
            },
            cleanup_marker,
        ));
    }
}
