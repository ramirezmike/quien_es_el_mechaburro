use crate::game_state;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default)]
pub struct PlayerSelectionState {
    pub burro: usize,
    pub outline_color: usize,
    pub state: SelectionState,
}

impl PlayerSelectionState {
    pub fn get_outline_color(&self) -> Color {
        super::OUTLINE_COLORS[self.outline_color]
    }
}

impl From<(PlayerSelectionState, game_state::PlayerMarker)> for game_state::BurroState {
    fn from(item: (PlayerSelectionState, game_state::PlayerMarker)) -> Self {
        game_state::BurroState {
            player: item.1 .0,
            selected_burro: item.0.burro,
            outline_color: item.0.get_outline_color(),
            score: 0,
            is_bot: false,
            hearts: vec![],
        }
    }
}

#[derive(Default, Resource)]
pub struct PlayerSelection {
    pub players: Vec<(PlayerSelectionState, game_state::PlayerMarker)>,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum SelectionState {
    #[default]
    NotPlaying,
    Burro,
    OutlineColor,
    Ready,
}

impl SelectionState {
    pub fn has_selected_burro(&self) -> bool {
        match self {
            SelectionState::NotPlaying | SelectionState::Burro => false,
            _ => true,
        }
    }
}
