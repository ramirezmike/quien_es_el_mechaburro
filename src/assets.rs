use crate::asset_loading;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

#[derive(Default)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub bgm_1: Handle<AudioSource>,
    pub sfx_1: Handle<AudioSource>,
    pub sfx_2: Handle<AudioSource>,
    pub level: Handle<Gltf>,
    pub burro: Handle<Gltf>,
    pub title_screen_background: asset_loading::GameTexture,
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: asset_loading::GameTexture,
}
