use crate::{assets, burro, game_state, scene_hook};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::component::Component,
    ecs::system::{Command, SystemState},
    gltf::Gltf,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_mod_outline::{OutlineBundle, OutlineVolume};

pub struct BurroImage<T: Component + Clone> {
    pub player: usize,
    pub selected_burro: usize,
    pub burro_transform: Transform,
    pub camera_transform: Transform,
    pub outline_color: Color,
    pub outline_size: f32,
    pub render_layer_id: u8,
    pub cleanup_marker: T,
    pub image_handle: Handle<Image>,
    pub clear_color: Color,
}

impl<T: Component + Clone> Command for BurroImage<T> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            Res<super::text_size::WindowSize>,
            ResMut<Assets<Image>>,
            Res<assets::GameAssets>,
            Res<Assets<Gltf>>,
        )> = SystemState::new(world);
        let (_, _, game_assets, assets_gltf) = system_state.get_mut(world);

        let toon_material_textured = game_assets.burro_assets[self.selected_burro]
            .toon_texture
            .clone();
        let burro_mesh_handle = game_assets.burro.clone();
        let render_layer = RenderLayers::layer(self.render_layer_id);

        if let Some(gltf) = assets_gltf.get(&burro_mesh_handle) {
            let scene = gltf.scenes[0].clone();
            world.spawn((
                burro::BurroMeshMarker { parent: None },
                game_state::PlayerMarker(self.player),
                self.cleanup_marker.clone(),
                scene_hook::HookedSceneBundle {
                    scene: SceneBundle {
                        scene,
                        transform: self.burro_transform,
                        ..default()
                    },
                    hook: scene_hook::SceneHook::new(move |cmds, hook_data| {
                        if let Some(name) = hook_data.name {
                            let name = name.as_str();
                            if name.contains("Cube") {
                                cmds.insert((
                                    OutlineBundle {
                                        outline: OutlineVolume {
                                            visible: true,
                                            width: self.outline_size,
                                            colour: self.outline_color,
                                        },
                                        ..default()
                                    },
                                    render_layer,
                                    game_state::PlayerMarker(self.player),
                                    toon_material_textured.clone(),
                                ));
                            }
                        }
                    }),
                },
            ));
        }

        world.spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(self.clear_color),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(self.image_handle.clone()),
                    ..default()
                },
                transform: self.camera_transform,
                ..default()
            },
            UiCameraConfig { show_ui: false },
            self.cleanup_marker,
            render_layer,
        ));
    }
}

pub fn create_render_image(window_size: &Res<super::text_size::WindowSize>) -> Image {
    let size = Extent3d {
        width: (window_size.width / 4.0) as u32,
        height: (window_size.width / 4.0) as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    image
}
