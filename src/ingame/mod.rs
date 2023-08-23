use crate::{
    asset_loading, assets, bot, burro, direction, floor, game_camera, game_state, player,
    scene_hook, AppState,
};
use bevy::ecs::system::{Command, SystemState};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_mod_outline::{ OutlineBundle, OutlineVolume };
use bevy_rapier3d::prelude::*;
use bevy_toon_shader::{ToonShaderMaterial, ToonShaderSun};
use std::f32::consts::TAU;
use std::sync::{Arc, Mutex};

mod ui;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadInGame), setup)
            .add_plugins(ui::InGameUIPlugin)
            .add_systems(
                Update,
                (
                    (bot::update_bot_ai, bot::update_virtual_controllers).chain(),
                    player::handle_input,
                    player::move_player,
                    apply_deferred,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

pub struct IngameLoader;
impl Command for IngameLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            asset_loading::AssetsHandler,
            ResMut<assets::GameAssets>,
            ResMut<game_state::GameState>,
            ResMut<Assets<ToonShaderMaterial>>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets, game_state, mut toon_materials) =
            system_state.get_mut(world);

        assets_handler.add_font(&mut game_assets.font, "fonts/MexicanTequila.ttf");
        assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
        assets_handler.add_glb(&mut game_assets.burro, "models/burro_new.glb");
        assets_handler.add_material(&mut game_assets.avatar_bottom, "textures/bottom_avatar.png", true);
        assets_handler.add_material(&mut game_assets.avatar_top, "textures/top_avatar.png", true);
        assets_handler.add_animation(
            &mut game_assets.burro_run,
            "models/burro_new.glb#Animation0",
        );

        assets_handler.add_material(&mut game_assets.heart_texture, "textures/heart.png", true);

        let mut mechaburro_texture = asset_loading::GameTexture::default();
        assets_handler.add_material(&mut mechaburro_texture, &"textures/mechaburro.png", false);
        let toon_material_textured = toon_materials.add(ToonShaderMaterial {
            base_color_texture: Some(mechaburro_texture.image.clone()),
            color: Color::default(),
            sun_dir: Vec3::new(0.0, 0.0, 0.0),
            sun_color: Color::default(),
            camera_pos: Vec3::new(0.0, 1.0, -1.0),
            ambient_color: Color::default(),
        });
        game_assets.mechaburro_texture = assets::BurroAsset {
            name: "Mechaburro".into(),
            texture: mechaburro_texture,
            toon_texture: toon_material_textured,
        };

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
        );
    }
}

fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
    mut game_state: ResMut<game_state::GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    camera_settings.set_camera(20.0, Vec3::ZERO, 0.4, false, 0.5, 30.0);

    #[cfg(feature = "debug")]
    {
        if game_state.burros.is_empty() {
            *game_state = game_state::GameState::initialize(
                vec![
                    game_state::BurroState {
                        player: 0,
                        selected_burro: 0,
                        outline_color: Color::WHITE,
                        score: 0,
                        is_bot: false,
                        hearts: vec![],
                    },
                ],
                7,
                1.0,
                &game_assets.burro_assets,
            );
        }
    };

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
                hook: scene_hook::SceneHook::new(move |cmds, hook_data| {
                    if let Some(name) = hook_data.name {
                        let name = name.as_str();
                        println!("{}", name);
                        if name.contains("Cube") {
                            if let Some(mesh) = hook_data.mesh {
                                cmds.insert(
                                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                                        .unwrap(),
                                )
                                .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL));
                            }
                        }

                        if name.contains("floor") {
                            if let (Some(global_transform), Some(aabb)) =
                                (hook_data.global_transform, hook_data.aabb)
                            {
                                hook_data
                                    .floor_manager
                                    .store_floor(&global_transform, &aabb);
                            }
                        }

                        if name.contains("spawn_point") {
                            println!("LOADING SPAWN POINTS");
                            if let (Some(global_transform), Some(aabb)) =
                                (hook_data.global_transform, hook_data.aabb)
                            {
                                let matrix = global_transform.compute_matrix();
                                let translation = matrix.transform_point3(aabb.center.into());
                                if let Ok(mut spawn_points) = hook_spawn_points.lock() {
                                    spawn_points.push(translation);
                                }
                                cmds.insert(Visibility::Hidden);
                            }
                        }

                        if name.contains("Invisible") {
                            cmds.insert(Visibility::Hidden);
                        }
                    }
                }),
            },
            scene_hook::SceneOnComplete::new(move |cmds, assets_gltf, game_assets, game_state| {
                if let Ok(spawn_points) = on_complete_spawn_points.lock() {
                    for (i, burro_state) in game_state.burros.iter().enumerate() {
                        let point = spawn_points[i];

                        let toon_material_textured = game_assets.burro_assets
                            [burro_state.selected_burro]
                            .toon_texture
                            .clone();

                        if let Some(gltf) = assets_gltf.get(&burro_mesh_handle) {
                            let mut entity_commands = cmds.spawn((
                                RigidBody::KinematicPositionBased,
                                Collider::ball(1.0),
                                ColliderMassProperties::Density(2.0),
                                KinematicCharacterController {
                                    offset: CharacterLength::Relative(0.1),
                                    max_slope_climb_angle: std::f32::consts::PI / 2.0,
                                    min_slope_slide_angle: 0.0,
                                    slide: true,
                                    translation: Some(Vec3::new(0.0, 0.5, 0.0)),
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
                                burro::Burro::new(burro_state.selected_burro),
                                game_state::PlayerMarker(burro_state.player),
                                player::BurroMovement::default(),
                                TransformBundle {
                                    local: {
                                        let mut t = Transform::from_xyz(point.x, 0.5, point.z);
                                        t.rotation = Quat::from_axis_angle(Vec3::Y, TAU * 0.5);
                                        t
                                    },
                                    ..default()
                                },
                            ));

                            if burro_state.is_bot {
                                entity_commands.insert(bot::BotBundle::new());
                            } else {
                                entity_commands.insert(player::PlayerBundle::new());
                            }

                            let outline_color = burro_state.outline_color.clone();
                            entity_commands.with_children(|parent| {
                                let parent_entity = parent.parent_entity();
                                parent.spawn(scene_hook::HookedSceneBundle {
                                    scene: SceneBundle {
                                        scene: gltf.scenes[0].clone(),
                                        ..default()
                                    },
                                    hook: scene_hook::SceneHook::new(move |cmds, hook_data| {
                                        if let Some(name) = hook_data.name {
                                            let name = name.as_str();
                                            if name.contains("Armature") {
                                                cmds.insert((
                                                    assets::AnimationLink {
                                                        entity: parent_entity,
                                                    },
                                                    Transform::from_xyz(0.0, -1.0, 0.0),
                                                ));
                                            }
                                            if name.contains("Cube") {
                                                cmds.insert((
                                                    OutlineBundle {
                                                        outline: OutlineVolume {
                                                            visible: true,
                                                            width: 5.0,
                                                            colour: outline_color,
                                                        },
                                                        ..default()
                                                    },
                                                    burro::BurroMeshMarker {
                                                        parent: Some(parent_entity),
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
                illuminance: 100000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        ToonShaderSun,
    ));

    next_state.set(AppState::MechaPicker);
}

#[derive(Component)]
struct CleanupMarker;
