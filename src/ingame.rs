use crate::{asset_loading, assets::GameAssets, cleanup, collision, game_camera, player, AppState};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup)
                .with_system(game_camera::spawn_camera)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(cleanup::<CleanupMarker>),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update)
                    .with_system(game_camera::pan_orbit_camera),
            );
    }
}

#[derive(Component)]
struct CleanupMarker;

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_glb(&mut game_assets.level, "models/level_01.glb");
    assets_handler.add_glb(&mut game_assets.burro, "models/burro.glb");
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collidables: ResMut<collision::Collidables>,
) {
    if let Some(gltf) = assets_gltf.get(&game_assets.level) {
        println!("Adding level");
        commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
    }

    if let Some(gltf) = assets_gltf.get(&game_assets.burro) {
        println!("Adding burro");
        commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent
                    .spawn_bundle((
                        Transform::from_rotation(Quat::from_rotation_y(
                            std::f32::consts::FRAC_PI_2,
                        )),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(gltf.scenes[0].clone());
                    });
            })
            .insert_bundle(player::PlayerBundle::default())
            .insert(CleanupMarker);
    }

    collidables.reset();
}

fn update() {
}
