use crate::{
    asset_loading, assets::GameAssets, bot, cleanup, collision, game_camera, mesh, player, AppState,
};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

pub struct DebugRoomPlugin;
impl Plugin for DebugRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Debug)
                .with_system(game_camera::spawn_camera)
                .with_system(setup),
        )
        .add_system_set(SystemSet::on_exit(AppState::Debug).with_system(cleanup::<CleanupMarker>))
        .add_system_set(
            SystemSet::on_update(AppState::Debug)
                .with_system(game_camera::pan_orbit_camera)
                .with_system(store_current_window_size)
                .with_system(paint_ui),
        )
        .insert_resource(WindowSize {
            width: 0.0,
            height: 0.0,
        });
    }
}

#[derive(Component)]
struct CleanupMarker;

struct WindowSize {
    width: f32,
    height: f32,
}
fn store_current_window_size(
    windows: Res<Windows>,
    mut win_size: ResMut<WindowSize>,
    mut resize_event: EventReader<WindowResized>,
) {
    if win_size.width == 0.0 && win_size.height == 0.0 {
        if let Some(window) = windows.get_primary() {
            win_size.width = window.width();
            win_size.height = window.height();
        }
    }

    for e in resize_event.iter() {
        win_size.width = e.width;
        win_size.height = e.height;
    }
}

fn paint_ui(
    mut ctx: ResMut<EguiContext>,
    win_size: Res<WindowSize>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
) {
    let ctx = ctx.ctx_mut();

    egui::Window::new("")
        .resizable(true)
        .title_bar(false)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.collapsing("Levels", |ui| {
                if ui.button("Level 1").clicked() {
                    //assets_handler.load(AppState::LevelOne, &mut game_assets);
                }
            });
            ui.end_row();
        });
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_glb(&mut game_assets.level, "models/level_01.glb");
    assets_handler.add_mesh(&mut game_assets.burro.mesh, "models/burro.gltf#Mesh0/Primitive0");
    assets_handler.add_material(&mut game_assets.burro.texture, "textures/burro_01.png", false);
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collidables: ResMut<collision::Collidables>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    if let Some(gltf) = assets_gltf.get(&game_assets.level) {
        commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
    }

    commands
        .spawn_bundle(PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: game_assets.burro.texture.material.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert_bundle(player::PlayerBundle::default())
        .insert(CleanupMarker);

//      for i in 0..0 {
//          commands
//              .spawn_bundle((
//                  Transform::from_xyz(2.0, 0.0, 0.0),
//                  GlobalTransform::identity(),
//              ))
//              .with_children(|parent| {
//                  parent
//                      .spawn_bundle((
//                          Transform::from_rotation(Quat::from_rotation_y(
//                              std::f32::consts::FRAC_PI_2,
//                          )),
//                          GlobalTransform::identity(),
//                      ))
//                      .with_children(|parent| {
//                          parent.spawn_scene(gltf.scenes[0].clone());
//                      });
//              })
//              .insert_bundle(bot::BotBundle::default())
//              .insert(CleanupMarker);
//      }

    collidables.reset();
}

fn update() {}
