use crate::{assets, game_camera::PanOrbitCamera, ui::text_size};
use bevy::ecs::system::{Command, SystemState};
use bevy::prelude::*;

pub struct FollowTextPlugin;
impl Plugin for FollowTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text_position);
    }
}

#[derive(Component)]
pub struct FollowText {
    pub following: Entity,
    pub offset: f32,
}

pub trait FollowTextCommandsExt {
    fn spawn_follow_text<T: Component>(
        &mut self,
        follow_entity: Entity,
        text: String,
        color: Color,
        cleanup_marker: T,
    );
}

impl<'w, 's> FollowTextCommandsExt for Commands<'w, 's> {
    fn spawn_follow_text<T: Component>(
        &mut self,
        follow_entity: Entity,
        text: String,
        color: Color,
        cleanup_marker: T,
    ) {
        self.add(SpawnFollowText {
            follow_entity,
            text,
            color,
            cleanup_marker,
        });
    }
}

struct SpawnFollowText<T: Component> {
    follow_entity: Entity,
    text: String,
    color: Color,
    cleanup_marker: T,
}
impl<T: Component> Command for SpawnFollowText<T> {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(Res<assets::GameAssets>, text_size::TextScaler)> =
            SystemState::new(world);
        let (game_assets, text_scaler) = system_state.get(world);

        world
            .spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                text: Text::from_section(
                    self.text.clone(),
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(super::FOLLOW_FONT_SIZE),
                        color: self.color,
                    },
                ),
                ..Default::default()
            })
            .insert(BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)))
            .insert(self.cleanup_marker)
            .insert(FollowText {
                following: self.follow_entity,
                offset: 0.0,
            });
    }
}

use bevy::text::TextLayoutInfo;
fn update_text_position(
    mut text_query: Query<(&mut Style, &TextLayoutInfo, &mut FollowText)>,
    mesh_query: Query<&Transform>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    time: Res<Time>,
) {
    for (mut style, text_layout_info, mut follow_text) in text_query.iter_mut() {
        if let Ok(mesh_position) = mesh_query.get(follow_text.following) {
            let translation = Vec3::new(
                mesh_position.translation.x,
                mesh_position.translation.y + 1.5,
                mesh_position.translation.z,
            );

            for (camera, camera_transform) in camera_query.iter() {
                follow_text.offset += time.delta_seconds() * 2.0;
                match camera.world_to_viewport(camera_transform, translation) {
                    Some(coords) => {
                        style.left = Val::Px(coords.x)
                            .try_sub(Val::Px(text_layout_info.size.x / 2.0))
                            .unwrap();
                        style.top = Val::Px(coords.y)
                            .try_sub(Val::Px(text_layout_info.size.y / 2.0))
                            .unwrap();
                    }
                    None => {
                        // A hack to hide the text when the it's behind the camera
                        style.bottom = Val::Px(-1000.0);
                    }
                }
            }
        }
    }
}
