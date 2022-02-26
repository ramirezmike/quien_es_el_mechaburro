#![windows_subsystem = "windows"]

use bevy::ecs::component::Component;
use bevy::prelude::*;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod asset_loading;
mod assets;
mod audio;
mod bullet;
mod bot;
mod burro;
mod collision;
mod direction;
mod game_camera;
//mod ingame;
mod inspect;
mod levels;
mod mesh;
mod player;
mod title_screen;

fn main() {
    App::new()
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(bot::BotPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(game_camera::GameCameraPlugin)
        .add_plugin(mesh::MeshPlugin)
        .add_plugin(title_screen::TitlePlugin)
        //.add_plugin(ingame::InGamePlugin)
        .add_plugin(inspect::InspectPlugin)
        .add_plugin(levels::debug::DebugRoomPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(collision::WorldCollisionPlugin)
        .add_state(AppState::Initial)
        .add_system_set(SystemSet::on_enter(AppState::Initial).with_system(bootstrap))
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Initial,
    Pause,
    Debug,
    //InGame,
    TitleScreen,
    ModelLoading,
    Loading,
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
) {
    assets_handler.load(AppState::TitleScreen, &mut game_assets);
}

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
