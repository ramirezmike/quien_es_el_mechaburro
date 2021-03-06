use crate::game_camera::PanOrbitCamera;
use bevy::prelude::*;

pub struct FollowTextPlugin;
impl Plugin for FollowTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_text_position);
    }
}

#[derive(Component)]
pub struct FollowText {
    pub following: Entity,
}

fn update_text_position(
    windows: Res<Windows>,
    mut text_query: Query<(&mut Style, &CalculatedSize, &FollowText)>,
    mesh_query: Query<&Transform>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    images: Res<Assets<Image>>,
) {
    for (mut style, calculated, follow) in text_query.iter_mut() {
        if let Ok(mesh_position) = mesh_query.get(follow.following) {
            for (camera, camera_transform) in camera_query.iter() {
                let translation = Vec3::new(
                    mesh_position.translation.x,
                    mesh_position.translation.y + 1.0,
                    mesh_position.translation.z,
                );
                match camera.world_to_screen(&windows, &images, camera_transform, translation) {
                    Some(coords) => {
                        style.position.left = Val::Px(coords.x - calculated.size.width / 2.0);
                        style.position.bottom = Val::Px((coords.y) - calculated.size.height / 2.0);
                    }
                    None => {
                        // A hack to hide the text when the it's behind the camera
                        style.position.bottom = Val::Px(-1000.0);
                    }
                }
            }
        }
    }
}
