#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![windows_subsystem = "windows"]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::{bevy_egui, quick::WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use bevy_toon_shader::{ToonShaderMaterial, ToonShaderPlugin};

mod asset_loading;
mod assets;
mod audio;
mod bot;
mod bullet;
mod burro;
mod config;
mod direction;
mod floor;
mod game_camera;
mod game_state;
mod hit;
mod ingame;
mod input;
mod loading;
mod mecha_picker;
mod menu;
mod player;
mod scene_hook;
mod shaders;
mod smoke;
mod ui;
mod util;

#[cfg(feature = "fps")]
mod debug;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_state::<AppState>();

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());

    #[cfg(feature = "lines")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    #[cfg(feature = "fps")]
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        debug::DebugPlugin,
    ));


    app.insert_resource(bevy_egui::EguiSettings {
        scale_factor: 1.8,
        ..default()
    })
    .insert_resource(config::GameConfiguration::default())
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins((
        shaders::ShaderPlugin,
        menu::character_select::CharacterSelectPlugin,
        menu::settings::SettingsMenuPlugin,
        menu::splash::SplashPlugin,
        menu::title_screen::TitlePlugin,
    ))
    .add_plugins((
        audio::GameAudioPlugin,
        bullet::BulletPlugin,
        burro::BurroPlugin,
        bot::BotPlugin,
        hit::HitPlugin,
        game_state::GameStatePlugin,
        game_camera::GameCameraPlugin,
        mecha_picker::MechaPickerPlugin,
        floor::FloorPlugin,
        player::PlayerPlugin,
        ingame::InGamePlugin,
        smoke::SmokePlugin,
    ))
    .add_plugins((
        asset_loading::AssetLoadingPlugin,
        assets::AssetsPlugin,
        ToonShaderPlugin,
        input::InputPlugin,
        scene_hook::HookPlugin,
        ui::text_size::TextSizePlugin,
    ))
    .add_systems(Update, debug)
    .add_systems(Update, bootstrap.run_if(in_state(AppState::Initial)))
    .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Initial,
    Pause,
    Debug,
    Settings,
    LoadInGame,
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

use loading::command_ext::*;
fn bootstrap(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::hex("000000").unwrap();

    #[cfg(feature = "debug")]
    {
        commands.load_state(AppState::CharacterSelect);
    }

    #[cfg(not(feature = "debug"))]
    commands.load_state(AppState::Splash);
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
