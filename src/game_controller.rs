use bevy::prelude::*;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent, RustArcadePlugin};

pub struct GameControllerPlugin;
impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RustArcadePlugin)
            .insert_resource(GameController::default())
            .add_system(store_from_arcade_event_system.label("store_controller_inputs"));
    }
}

#[derive(Default)]
pub struct GameController {
    pub pressed: Vec<GameButton>,
    pub just_pressed: Vec<GameButton>,
}

impl GameController {
    fn clear_presses(&mut self) {
        self.pressed = Vec::<GameButton>::new();
        self.just_pressed = Vec::<GameButton>::new();
    }
}

fn store_from_arcade_event_system(
    mut arcade_input_events: EventReader<ArcadeInputEvent>,
    mut controllers: ResMut<GameController>,
) {
    let pressed = &mut controllers.pressed;
    let already_pressed = pressed.clone();
    let mut released_buttons = vec![];
    for event in arcade_input_events.iter() {
        if let Some(mapped_button) = map_arcade_button(&event.arcade_input) {
            if event.value >= 0.8 {
                pressed.push(mapped_button);
            } else {
                released_buttons.push(mapped_button);
            }
        }
    }

    pressed.retain(|button| !released_buttons.contains(button));
    pressed.sort();
    pressed.dedup();

    let mut just_pressed = pressed.clone();
    just_pressed.retain(|button| !already_pressed.contains(button));
    controllers.just_pressed = just_pressed;
}

fn map_arcade_button(input: &ArcadeInput) -> Option<GameButton> {
    println!("{:?}", input);
    match input {
        ArcadeInput::JoyUp => Some(GameButton::Up),
        ArcadeInput::JoyDown => Some(GameButton::Down),
        ArcadeInput::JoyLeft => Some(GameButton::Left),
        ArcadeInput::JoyRight => Some(GameButton::Right),
        ArcadeInput::ButtonTop2 => Some(GameButton::ActionUp),
        ArcadeInput::ButtonTop4 => Some(GameButton::ActionLeft),
        ArcadeInput::ButtonTop5 | 
        ArcadeInput::JoyButton => Some(GameButton::ActionDown),
        ArcadeInput::ButtonTop6 => Some(GameButton::ActionRight),
        ArcadeInput::ButtonFront2 => Some(GameButton::Start),
        _ => None,
    }
}

pub fn clear_presses(mut controllers: ResMut<GameController>) {
    controllers.clear_presses();
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
