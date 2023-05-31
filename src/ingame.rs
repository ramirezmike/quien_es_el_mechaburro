use crate::{asset_loading, assets, burro, game_camera, game_state, player, scene_hook, AppState};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_mod_outline::{
    AutoGenerateOutlineNormalsPlugin, OutlineBundle, OutlinePlugin, OutlineVolume,
};
use bevy_rapier3d::prelude::*;
use bevy_toon_shader::{ToonShaderMaterial, ToonShaderSun};
use std::sync::{Arc, Mutex};

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OutlinePlugin)
            .add_plugin(AutoGenerateOutlineNormalsPlugin)
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    burro::handle_burros,
                    player::handle_input,
                    player::move_player,
                    apply_system_buffers,
                )
                    .chain()
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<assets::GameAssets>,
    game_state: &ResMut<game_state::GameState>,
) {
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
    assets_handler.add_glb(&mut game_assets.burro, "models/burro_new.glb");
    assets_handler.add_animation(
        &mut game_assets.burro_run,
        "models/burro_new.glb#Animation0",
    );
    assets_handler.add_material(
        &mut game_assets.pinata_texture,
        "textures/pinata.png",
        false,
    );
    assets_handler.add_mesh(
        &mut game_assets.candy.mesh,
        "models/candy.gltf#Mesh0/Primitive0",
    );
    assets_handler.add_mesh(
        &mut game_assets.laser.mesh,
        "models/laser.gltf#Mesh0/Primitive0",
    );

    assets_handler.add_glb(
        &mut game_assets.level,
        &format!("models/level_{:02}.glb", game_state.current_level),
    )
}

fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
    mut game_state: ResMut<game_state::GameState>,
) {
    camera_settings.set_camera(20.0, Vec3::ZERO, 0.4, false, 0.5, 30.0);

    game_state.current_level_over = false;
    game_state.on_new_level();

    // SETTING LEVEL BACKGROUND
    *clear_color = match game_state.current_level {
        0 => ClearColor(Color::rgb(0.55, 0.92, 0.96)), // light blue
        1 => ClearColor(Color::rgb(1.0, 0.65, 0.62)),  // orange
        2 => ClearColor(Color::rgb(0.72, 0.98, 0.75)), // green
        3 => ClearColor(Color::rgb(0.81, 0.72, 0.94)), // purple
        4 => ClearColor(Color::rgb(1.0, 0.65, 0.62)),  // orange
        5 => ClearColor(Color::rgb(0.72, 0.98, 0.75)), // green
        6 => ClearColor(Color::rgb(0.81, 0.72, 0.94)), // purple
        _ => ClearColor(Color::rgb(1.0, 0.65, 0.62)),
    };

    let toon_material_textured = toon_materials.add(ToonShaderMaterial {
        base_color_texture: Some(game_assets.pinata_texture.image.clone()),
        color: Color::default(),
        sun_dir: Vec3::new(0.0, 0.0, 0.0),
        sun_color: Color::default(),
        camera_pos: Vec3::new(0.0, 1.0, -1.0),
        ambient_color: Color::default(),
    });

    let hook_spawn_points = Arc::new(Mutex::new(vec![]));
    let on_complete_spawn_points = Arc::clone(&hook_spawn_points);
    let burro_mesh_handle = game_assets.burro.clone();

    if let Some(gltf) = assets_gltf.get(&game_assets.level) {
        commands.spawn((
            scene_hook::HookedSceneBundle {
                scene: SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                },
                hook: scene_hook::SceneHook::new(move |entity, cmds, hook_data| {
                    if let Some(name) = entity.get::<Name>().map(|t| t.as_str()) {
                        if name.contains("Cube") {
                            if let Some(mesh) = hook_data.mesh {
                                cmds.insert(
                                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                                        .unwrap(),
                                )
                                .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL));
                            }
                        }

                        if name.contains("spawn_point") {
                            let matrix = hook_data.global_transform.unwrap().compute_matrix();
                            let translation =
                                matrix.transform_point3(hook_data.aabb.unwrap().center.into());
                            if let Ok(mut spawn_points) = hook_spawn_points.lock() {
                                spawn_points.push(translation);
                            }
                            cmds.insert(Visibility::Hidden);
                        }

                        if name.contains("Invisible") {
                            cmds.insert(Visibility::Hidden);
                        }
                    }
                }),
            },
            scene_hook::SceneOnComplete::new(move |cmds, assets_gltf| {
                if let Ok(spawn_points) = on_complete_spawn_points.lock() {
                    for point in spawn_points.iter() {
                        let toon_material_textured = toon_material_textured.clone();
                        if let Some(gltf) = assets_gltf.get(&burro_mesh_handle) {
                            cmds.spawn((
                                RigidBody::KinematicPositionBased,
                                Collider::ball(1.0),
                                ColliderMassProperties::Density(2.0),
                                KinematicCharacterController {
                                    offset: CharacterLength::Relative(0.1),
                                    max_slope_climb_angle: std::f32::consts::PI / 2.0,
                                    min_slope_slide_angle: 0.0,
                                    slide: true,
                                    translation: Some(Vec3::new(0.0, 1.0, 0.0)),
                                    filter_groups: Some(CollisionGroups::new(
                                        Group::GROUP_2,
                                        Group::GROUP_1,
                                    )),
                                    ..default()
                                },
                                Velocity::default(),
                                ComputedVisibility::default(),
                                Visibility::Visible,
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
                                player::PlayerBundle::new(),
                                burro::Burro::new(game_state::BurroSkin::Pinata),
                                TransformBundle {
                                    local: Transform::from_xyz(point.x, 0.5, point.z),
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                let parent_entity = parent.parent_entity();
                                parent.spawn(scene_hook::HookedSceneBundle {
                                    scene: SceneBundle {
                                        scene: gltf.scenes[0].clone(),
                                        ..default()
                                    },
                                    hook: scene_hook::SceneHook::new(move |entity, cmds, _| {
                                        if let Some(name) = entity.get::<Name>().map(|t| t.as_str())
                                        {
                                            if name.contains("Armature") {
                                                cmds.insert(assets::AnimationLink {
                                                    entity: parent_entity,
                                                });
                                            }
                                            if name.contains("Cube") {
                                                cmds.insert((
                                                    OutlineBundle {
                                                        outline: OutlineVolume {
                                                            visible: true,
                                                            width: 5.0,
                                                            colour: Color::WHITE,
                                                        },
                                                        ..default()
                                                    },
                                                    toon_material_textured.clone(),
                                                ));
                                            }
                                        }
                                    }),
                                });
                            });
                        }
                    }
                }
            }),
        ));
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    game_camera::spawn_camera(&mut commands, CleanupMarker);

    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::new(-0.8263363, -0.53950554, -0.16156079),
                2.465743,
            )),
            directional_light: DirectionalLight {
                // Configure the projection to better fit the scene
                //            illuminance: 10000.0,
                illuminance: 100000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        ToonShaderSun,
    ));
}

#[derive(Component)]
struct CleanupMarker;
