use crate::AppState;
use bevy::{asset::Asset, ecs::system::SystemParam, gltf::Gltf, prelude::*};
use bevy_kira_audio::AudioSource;
use std::marker::PhantomData;

pub struct AssetLoadingPlugin;
impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueueState>()
            .init_resource::<AssetsLoading>()
            .add_systems(
                Update,
                check_assets_ready.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Default)]
pub struct GameTexture {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
}

#[derive(Resource)]
pub struct QueueState {
    pub state: AppState,
}
impl Default for QueueState {
    fn default() -> Self {
        QueueState {
            state: AppState::TitleScreen,
        }
    }
}

#[derive(Default, Resource)]
pub struct AssetsLoading {
    pub asset_handles: Vec<(HandleUntyped, String)>,
}

#[derive(SystemParam)]
pub struct AssetsHandler<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    assets_loading: ResMut<'w, AssetsLoading>,
    meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub images: ResMut<'w, Assets<Image>>,
    state: Res<'w, State<AppState>>,
    next_state: ResMut<'w, NextState<AppState>>,
    queued_state: ResMut<'w, QueueState>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> AssetsHandler<'w, 's> {
    fn add_asset<T: Asset>(&mut self, asset: &mut Handle<T>, path: &str) {
        *asset = self.asset_server.load(path);
        self.assets_loading
            .asset_handles
            .push((asset.clone_untyped(), path.to_string()));
    }

    //  pub fn load_next_level(
    //      &mut self,
    //      game_state: &ResMut<game_state::GameState>,
    //      game_assets: &mut ResMut<GameAssets>,
    //  ) {
    //      let next_state = self.get_next_state(game_state);
    //      self.queue_assets_for_state(&next_state, game_assets, game_state);
    //      self.next_state.state = next_state;
    //      self.asset_server.watch_for_changes().unwrap();
    //      self.state.set(AppState::Loading).unwrap();
    //  }

    //  fn get_next_state(&self, game_state: &game_state::GameState) -> AppState {
    //      println!("Loading level {}", game_state.current_level);

    //      match game_state.current_level {
    //          0 => AppState::Debug,
    //          1 => AppState::Debug,
    //          2 => AppState::Debug,
    //          3 => AppState::Debug,
    //          4 => AppState::Debug,
    //          5 => AppState::Debug,
    //          6 => AppState::Debug,
    //          _ => AppState::WinnerDisplay,
    //      }
    //  }

    pub fn add_mesh(&mut self, mesh: &mut Handle<Mesh>, path: &str) {
        self.add_asset(mesh, path);
    }

    pub fn add_font(&mut self, font: &mut Handle<Font>, path: &str) {
        self.add_asset(font, path);
    }

    pub fn add_audio(&mut self, audio: &mut Handle<AudioSource>, path: &str) {
        self.add_asset(audio, path);
    }

    pub fn add_glb(&mut self, glb: &mut Handle<Gltf>, path: &str) {
        self.add_asset(glb, path);
    }

    pub fn add_animation(&mut self, animation: &mut Handle<AnimationClip>, path: &str) {
        self.add_asset(animation, path);
    }

    pub fn add_standard_mesh(&mut self, handle: &mut Handle<Mesh>, mesh: Mesh) {
        *handle = self.meshes.add(mesh);
    }

    pub fn add_standard_material(
        &mut self,
        handle: &mut Handle<StandardMaterial>,
        material: StandardMaterial,
    ) {
        *handle = self.materials.add(material);
    }

    pub fn add_material(&mut self, game_texture: &mut GameTexture, path: &str, transparent: bool) {
        self.add_asset(&mut game_texture.image, path);
        game_texture.material = self.materials.add(StandardMaterial {
            base_color_texture: Some(game_texture.image.clone()),
            alpha_mode: if transparent {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        });
    }
}

fn check_assets_ready(mut assets_handler: AssetsHandler) {
    use bevy::asset::LoadState;

    let mut ready = true;
    for (handle, path) in assets_handler.assets_loading.asset_handles.iter() {
        match assets_handler.asset_server.get_load_state(handle) {
            LoadState::Failed => {
                panic!("An asset had an error: {:?}", handle);
            }
            LoadState::Loaded => {}
            LoadState::Unloaded => {
                println!("unloaded!!");
                assets_handler.asset_server.reload_asset(path);
                ready = false;
            }
            _ => {
                ready = false;
            }
        }
    }

    if ready {
        println!("ready! {:?}", assets_handler.queued_state.state);
        assets_handler.assets_loading.asset_handles = vec![]; // clear list since we've loaded everything
        assets_handler
            .next_state
            .set(assets_handler.queued_state.state);
    } else {
        println!("not ready");
    }
}
