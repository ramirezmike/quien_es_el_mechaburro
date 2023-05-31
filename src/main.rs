#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![windows_subsystem = "windows"]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::{bevy_egui, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use bevy_toon_shader::ToonShaderPlugin;

mod asset_loading;
mod assets;
mod bullet;
mod burro;
mod config;
mod direction;
mod game_camera;
mod game_state;
mod ingame;
mod player;
mod scene_hook;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_state::<AppState>();

    #[cfg(feature = "inspect")]
    app.add_plugin(WorldInspectorPlugin::new());

    #[cfg(feature = "lines")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    #[cfg(feature = "fps")]
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.insert_resource(bevy_egui::EguiSettings {
        scale_factor: 1.8,
        ..default()
    })
    .insert_resource(config::GameConfiguration::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(asset_loading::AssetLoadingPlugin)
    .add_plugin(assets::AssetsPlugin)
    .add_plugin(bullet::BulletPlugin)
    .add_plugin(burro::BurroPlugin)
    .add_plugin(game_state::GameStatePlugin)
    .add_plugin(game_camera::GameCameraPlugin)
    .add_plugin(ToonShaderPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(ingame::InGamePlugin)
    .add_plugin(scene_hook::HookPlugin)
    .add_system(debug)
    .add_system(bootstrap.in_set(OnUpdate(AppState::Initial)))
    .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Initial,
    Pause,
    Debug,
    Options,
    InGame,
    TitleScreen,
    CharacterSelect,
    ModelLoading,
    MechaPicker,
    ScoreDisplay,
    Loading,
    WinnerDisplay,
    Splash,
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
    game_state: ResMut<game_state::GameState>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::hex("000000").unwrap();
    assets_handler.load(AppState::InGame, &mut game_assets, &game_state);
}

fn debug(
    //    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut exit: ResMut<Events<AppExit>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub trait ZeroSignum {
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
