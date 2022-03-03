use crate::{asset_loading::GameTexture, assets::GameAssets};
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::AddressMode;

pub struct MeshPlugin;
impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scroll_meshes);
    }
}

pub struct MeshBuilder {}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ScrollingPane {
    pub offset: f32,
    pub speed: f32,
    pub axis: Vec3,
    pub scroll_to: f32,
}

#[derive(Bundle, Default)]
pub struct ScrollingPaneBundle {
    pub scrolling: ScrollingPane,
}

impl MeshBuilder {
    pub fn add_scrolling_bundle(axis: Vec3) -> ScrollingPaneBundle {
        ScrollingPaneBundle {
            scrolling: ScrollingPane {
                offset: 0.0,
                speed: 0.5,
                axis,
                scroll_to: 4.0,
            },
        }
    }

    pub fn plane(
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        game_texture: &GameTexture,
        size: f32,
        z_index: f32,
    ) -> PbrBundle {
        let image = images.get_mut(game_texture.image.clone());

        let mut mesh = Mesh::from(shape::Plane::default());
        //      if let Some(VertexAttributeValues::Float32x2(uvs)) =
        //          mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
        //      {
        //          for uv in uvs {
        //              uv[0] *= size / 4.0;
        //              uv[1] *= size / 4.0;
        //          }
        //      }

        PbrBundle {
            transform: {
                let mut transform = Transform::from_scale(Vec3::splat(size));
                transform.translation.y = z_index;
                //transform.rotate(Quat::from_rotation_y(2.0 * std::f32::consts::PI));

                transform
            },
            material: game_texture.material.clone(),
            mesh: meshes.add(mesh),
            ..Default::default()
        }
    }

    pub fn plane_repeating(
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        game_texture: &GameTexture,
        size: f32,
        position: Option<Vec3>,
        rotation: Option<Quat>,
    ) -> PbrBundle {
        let image = images.get_mut(game_texture.image.clone());
        if let Some(image) = image {
            image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
            image.sampler_descriptor.address_mode_v = AddressMode::Repeat;
        }

        let mut mesh = Mesh::from(shape::Plane::default());
        if let Some(VertexAttributeValues::Float32x2(uvs)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
        {
            for uv in uvs {
                uv[0] *= size / 4.0;
                uv[1] *= size / 4.0;
            }
        }

        PbrBundle {
            transform: {
                let mut transform = Transform::from_scale(Vec3::splat(size));
                transform.translation = position.unwrap_or(transform.translation);
                transform.rotation = rotation.unwrap_or(transform.rotation);
                //transform.rotate(Quat::from_rotation_y(2.0 * std::f32::consts::PI));

                transform
            },
            material: game_texture.material.clone(),
            mesh: meshes.add(mesh),
            ..Default::default()
        }
    }
}

fn scroll_meshes(
    time: Res<Time>,
    mut scroll_meshes: Query<(&mut ScrollingPane, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    game_assets: Res<GameAssets>,
) {
    // this might be bad to do every frame
    let image = images.get_mut(&game_assets.title_screen_background.image.clone());
    if let Some(image) = image {
        image.sampler_descriptor.address_mode_u = AddressMode::Repeat;
        image.sampler_descriptor.address_mode_v = AddressMode::Repeat;
    }

    for (mut pane, mut transform) in scroll_meshes.iter_mut() {
        /*
            pub offset: f32,
            pub speed: f32,
            pub axis: Vec3,
            pub scroll_to: f32,
        */
        pane.offset += pane.speed * time.delta_seconds();
        if pane.offset > pane.scroll_to {
            transform.translation += -pane.offset * pane.axis;
            pane.offset = 0.0;
        } else {
            let movement = pane.speed * time.delta_seconds() * pane.axis;
            transform.translation += movement;
        }
    }
}
