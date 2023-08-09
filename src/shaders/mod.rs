use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};
use std::marker::PhantomData;

pub struct ShaderPlugin;
impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ScrollingImageMaterial>::default());
    }
}

#[derive(SystemParam)]
pub struct ShaderMaterials<'w, 's> {
    pub scrolling_images: ResMut<'w, Assets<ScrollingImageMaterial>>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct ScrollingImageMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for ScrollingImageMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/scroll_texture.wgsl".into()
    }
}
