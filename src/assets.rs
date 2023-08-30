use crate::asset_loading;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_toon_shader::ToonShaderMaterial;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

#[derive(Component)]
pub struct AnimationLink {
    pub entity: Entity,
}

#[derive(Default)]
pub struct BurroAsset {
    pub name: String,
    pub texture: asset_loading::GameTexture,
    pub toon_texture: Handle<ToonShaderMaterial>,
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub score_font: Handle<Font>,

    pub bgm_1: Handle<AudioSource>,
    pub sfx_1: Handle<AudioSource>,
    pub sfx_2: Handle<AudioSource>,
    pub laser_sfx: Handle<AudioSource>,
    pub bloop_sfx: Handle<AudioSource>,
    pub smoke_sfx: Handle<AudioSource>,
    pub eliminated_sfx: Handle<AudioSource>,
    pub candy_hit_sfx: Handle<AudioSource>,
    pub laser_hit_sfx: Handle<AudioSource>,
    pub fanfare_sfx: Handle<AudioSource>,

    pub level: Handle<Gltf>,
    pub stage: Handle<Gltf>,
    pub candy: GameMesh,
    pub laser: GameMesh,
    pub bevy_icon: asset_loading::GameTexture,
    pub level_background: asset_loading::GameTexture,
    pub title_screen_background: asset_loading::GameTexture,
    pub title_screen_logo: asset_loading::GameTexture,

    pub burro: Handle<Gltf>,
    pub burro_run: Handle<AnimationClip>,
    pub burro_assets: Vec<BurroAsset>,
    pub mechaburro_texture: BurroAsset,
    pub avatar_bottom: asset_loading::GameTexture,
    pub avatar_top: asset_loading::GameTexture,

    pub heart_texture: asset_loading::GameTexture,
    pub pinata_logo_texture: asset_loading::GameTexture,
    pub meow_logo_texture: asset_loading::GameTexture,
    pub salud_logo_texture: asset_loading::GameTexture,
    pub mexico_logo_texture: asset_loading::GameTexture,
    pub medianoche_logo_texture: asset_loading::GameTexture,
    pub morir_logo_texture: asset_loading::GameTexture,
    pub gators_logo_texture: asset_loading::GameTexture,
    pub aguas_logo_texture: asset_loading::GameTexture,
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: asset_loading::GameTexture,
}
