#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![windows_subsystem = "windows"]

use bevy::ecs::component::Component;
use bevy::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod asset_loading;
mod assets;
mod audio;
mod bot;
mod bullet;
mod burro;
mod character_select;
mod collision;
mod direction;
mod follow_text;
mod game_camera;
mod game_controller;
mod game_state;
mod hit;
mod ingame_ui;
mod inspect;
mod levels;
mod mecha_picker;
mod mesh;
mod pause;
mod player;
mod score_display;
mod smoke;
mod title_screen;
mod ui;
mod winner;

fn main() {
    App::new()
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(burro::BurroPlugin)
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(bot::BotPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(character_select::CharacterSelectPlugin)
        .add_plugin(collision::WorldCollisionPlugin)
        .add_plugin(follow_text::FollowTextPlugin)
        .add_plugin(hit::HitPlugin)
        .add_plugin(game_camera::GameCameraPlugin)
        .add_plugin(game_controller::GameControllerPlugin)
        .add_plugin(game_state::GameStatePlugin)
        .add_plugin(mesh::MeshPlugin)
        .add_plugin(mecha_picker::MechaPickerPlugin)
        .add_plugin(title_screen::TitlePlugin)
        .add_plugin(ingame_ui::InGameUIPlugin)
        .add_plugin(inspect::InspectPlugin)
        .add_plugin(levels::debug::DebugRoomPlugin)
        .add_plugin(pause::PausePlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(score_display::ScoreDisplayPlugin)
        .add_plugin(smoke::SmokePlugin)
        .add_plugin(winner::WinnerPlugin)
        .add_state(AppState::Initial)
        .add_system_set(SystemSet::on_enter(AppState::Initial).with_system(bootstrap))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Initial,
    Pause,
    Debug,
    InGame,
    TitleScreen,
    CharacterSelect,
    ModelLoading,
    MechaPicker,
    ScoreDisplay,
    Loading,
    WinnerDisplay,
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
    game_state: ResMut<game_state::GameState>,
) {
    assets_handler.load(AppState::TitleScreen, &mut game_assets, &game_state);
}

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
