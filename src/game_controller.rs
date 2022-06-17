use bevy::prelude::*;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin};
use std::collections::HashMap;

pub struct GameControllerPlugin;
impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections)
            .add_plugin(RustArcadePlugin)
            .insert_resource(GameController::default())
            .add_system(store_from_arcade_event_system.label("store_controller_inputs"));
    }
}

#[derive(Default)]
pub struct GameController {
    pub players: Vec<Gamepad>,
    pub pressed: HashMap<usize, Vec<GameButton>>,
    pub just_pressed: HashMap<usize, Vec<GameButton>>,
}

impl GameController {
    fn clear_presses(&mut self) {
        self.pressed = HashMap::<usize, Vec<GameButton>>::new();
        self.just_pressed = HashMap::<usize, Vec<GameButton>>::new();
    }
}

fn store_from_arcade_event_system(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut controllers: ResMut<GameController>,
) {
    let players = controllers.players.clone();
    for gamepad in players.iter() {
        let gamepad_id = gamepad.0;

        controllers.pressed.entry(gamepad_id).or_insert_with(Vec::new);
        controllers.just_pressed.entry(gamepad_id).or_insert_with(Vec::new);

        let pressed = controllers.pressed.get_mut(&gamepad_id).unwrap();
        let already_pressed = pressed.clone();
        let mut released_buttons = vec![];
        for event in arcade_input_events.iter() {
            let mapped_button = map_arcade_button(&event.arcade_input);
            if event.value >= 0.8 {
                pressed.push(mapped_button);
            } else {
                released_buttons.push(mapped_button);
            }
        }

        pressed.retain(|button| !released_buttons.contains(button));
        pressed.sort();
        pressed.dedup();

        let mut just_pressed = pressed.clone();
        just_pressed.retain(|button| !already_pressed.contains(button));
        controllers.just_pressed.insert(gamepad_id, just_pressed);
    }
}

fn map_arcade_button(input: &ArcadeInput) -> GameButton {
    match input {
        ArcadeInput::JoyUp => GameButton::Up,
        ArcadeInput::JoyDown => GameButton::Down,
        ArcadeInput::JoyLeft => GameButton::Left,
        ArcadeInput::JoyRight => GameButton::Right,
        ArcadeInput::ButtonTop2 => GameButton::ActionUp,
        ArcadeInput::ButtonTop4 => GameButton::ActionLeft,
        ArcadeInput::ButtonTop5 => GameButton::ActionDown,
        ArcadeInput::ButtonTop6 => GameButton::ActionRight,
        _ => GameButton::Start,
    }
}

pub fn clear_presses(mut controllers: ResMut<GameController>) {
    controllers.clear_presses();
}

pub fn gamepad_connections(
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut controllers: ResMut<GameController>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        if *kind == GamepadEventType::Connected {
            println!("New gamepad connected with ID: {:?}", id);
            controllers.players.push(*id);
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum GameButton {
    Up,
    Down,
    Left,
    Right,
    ActionUp,
    ActionLeft,
    ActionRight,
    ActionDown,
    Start,
}
