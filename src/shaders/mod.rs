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
        app.add_plugins(MaterialPlugin::<ScrollingImageMaterial>::default())
           .add_plugins(MaterialPlugin::<BackgroundMaterial>::default());
    }
}

#[derive(SystemParam)]
pub struct ShaderMaterials<'w, 's> {
    pub scrolling_images: ResMut<'w, Assets<ScrollingImageMaterial>>,
    pub ingame_backgrounds: ResMut<'w, Assets<BackgroundMaterial>>,
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

#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "817f64fe-6844-4822-8926-e0ed374294c8"]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub color: Color,
    #[uniform(3)]
    pub x_scroll_speed: f32,
    #[uniform(4)]
    pub y_scroll_speed: f32,
    #[uniform(5)]
    pub scale: f32,
}

impl Material for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
       AlphaMode::Blend
    }
}
