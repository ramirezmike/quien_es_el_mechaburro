use crate::{cleanup, AppState};
use bevy::prelude::*;

pub mod loader;
mod setup;
mod state;
mod update;

use self::{
    setup::setup,
    state::SettingsMenuState,
    update::{handle_input, highlight_selection, update_values},
};

pub struct SettingsMenuPlugin;
impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup)
            .init_resource::<SettingsMenuState>()
            .add_systems(
                Update,
                (highlight_selection, handle_input, update_values)
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;
#[derive(Component)]
pub struct SettingDisplayMarker;
