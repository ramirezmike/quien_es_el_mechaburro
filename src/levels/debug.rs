use crate::{
    asset_loading, assets::GameAssets, bot, burro::Burro, burro::BurroDeathEvent, cleanup,
    collision, follow_text, game_camera, game_state, inspect, player, AppState,
};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct DebugRoomPlugin;
impl Plugin for DebugRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Debug)
                .with_system(game_camera::spawn_camera)
                .with_system(setup),
        )
        .insert_resource(RoundEndTimer {
            round_ending: false,
            cooldown: 0.0,
        })
        .add_event::<PlayersDiedEarlyEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(check_for_next_level)
                .with_system(check_for_all_players_dead)
                .with_system(handle_players_died_early),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::ScoreDisplay)
                .with_system(despawn_round_end_text)
                .with_system(stop_firing),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(cleanup::<CleanupMarker>)
                .with_system(game_camera::despawn_camera),
        );
    }
}

fn check_for_next_level(
    mut game_state: ResMut<game_state::GameState>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
) {
    if game_state.current_level_over {
        game_state.current_level += 1;
        assets_handler.load_next_level(&game_state, &mut game_assets);
    }
}

fn stop_firing(mut players: Query<&mut player::Player>) {
    for mut player in players.iter_mut() {
        player.is_firing = false;
    }
}

struct PlayersDiedEarlyEvent;
#[derive(Component)]
struct RoundEndText;
struct RoundEndTimer {
    round_ending: bool,
    cooldown: f32,
}

fn despawn_round_end_text(mut commands: Commands, texts: Query<Entity, With<RoundEndText>>) {
    for entity in texts.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_players_died_early(
    mut players_died_early_event_reader: EventReader<PlayersDiedEarlyEvent>,
    mut text: Query<&mut Visibility, With<RoundEndText>>,
    mut round_end_timer: ResMut<RoundEndTimer>,
    mut burro_death_event_writer: EventWriter<BurroDeathEvent>,
    remaining_burros: Query<(Entity, &Burro)>,
    time: Res<Time>,
) {
    if round_end_timer.round_ending {
        round_end_timer.cooldown -= time.delta_seconds();
        round_end_timer.cooldown = round_end_timer.cooldown.clamp(-10.0, 5.0);

        if round_end_timer.cooldown <= 0.0 {
            for (entity, remaining_burro) in remaining_burros.iter() {
                burro_death_event_writer.send(BurroDeathEvent {
                    entity,
                    skin: remaining_burro.burro_skin,
                });
            }
        }
    } else {
        for _ in players_died_early_event_reader.iter() {
            for mut visibility in text.iter_mut() {
                visibility.is_visible = true;
            }
            round_end_timer.cooldown = 5.0;
            round_end_timer.round_ending = true;
        }
    }
}

fn check_for_all_players_dead(
    game_state: Res<game_state::GameState>,
    mut players_died_early_event_writer: EventWriter<PlayersDiedEarlyEvent>,
) {
    let are_any_players_alive = game_state
        .burros
        .iter()
        .filter(|b| !b.is_bot && !game_state.dead_burros.contains(&b.skin))
        .count()
        > 0;

    if !are_any_players_alive {
        players_died_early_event_writer.send(PlayersDiedEarlyEvent);
    }
}

#[derive(Component)]
struct CleanupMarker;

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
    game_state: &ResMut<game_state::GameState>,
) {
    match game_state.current_level {
        1 => assets_handler.add_glb(&mut game_assets.level, "models/level_02.glb"),
        2 => assets_handler.add_glb(&mut game_assets.level, "models/level_03.glb"),
        3 => assets_handler.add_glb(&mut game_assets.level, "models/level_06.glb"),
        4 => assets_handler.add_glb(&mut game_assets.level, "models/level_05.glb"),
        5 => assets_handler.add_glb(&mut game_assets.level, "models/level_04.glb"),
        6 => assets_handler.add_glb(&mut game_assets.level, "models/level_01.glb"),
        _ => assets_handler.add_glb(&mut game_assets.level, "models/level_00.glb"),
    }

    assets_handler.add_audio(&mut game_assets.bloop_sfx, "audio/bloop.wav");
    assets_handler.add_audio(&mut game_assets.laser_sfx, "audio/laser.wav");
    assets_handler.add_mesh(
        &mut game_assets.candy.mesh,
        "models/candy.gltf#Mesh0/Primitive0",
    );
    assets_handler.add_mesh(
        &mut game_assets.laser.mesh,
        "models/laser.gltf#Mesh0/Primitive0",
    );
    assets_handler.add_mesh(
        &mut game_assets.burro.mesh,
        "models/burro.gltf#Mesh0/Primitive0",
    );
    assets_handler.add_material(
        &mut game_assets.pinata_texture,
        "textures/pinata.png",
        false,
    );
    assets_handler.add_material(&mut game_assets.meow_texture, "textures/meow.png", false);
    assets_handler.add_material(&mut game_assets.salud_texture, "textures/salud.png", false);
    assets_handler.add_material(
        &mut game_assets.mexico_texture,
        "textures/mexico.png",
        false,
    );
    assets_handler.add_material(
        &mut game_assets.medianoche_texture,
        "textures/medianoche.png",
        false,
    );
    assets_handler.add_material(&mut game_assets.morir_texture, "textures/morir.png", false);
    assets_handler.add_material(
        &mut game_assets.gators_texture,
        "textures/gators.png",
        false,
    );
    assets_handler.add_material(&mut game_assets.aguas_texture, "textures/aguas.png", false);
    assets_handler.add_material(
        &mut game_assets.mechaburro_texture,
        "textures/mechaburro.png",
        false,
    );

    assets_handler.add_material(
        &mut game_assets.pinata_logo_texture,
        "textures/pinata_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.meow_logo_texture,
        "textures/meow_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.salud_logo_texture,
        "textures/salud_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.mexico_logo_texture,
        "textures/mexico_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.medianoche_logo_texture,
        "textures/medianoche_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.morir_logo_texture,
        "textures/morir_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.gators_logo_texture,
        "textures/gators_icon.png",
        true,
    );
    assets_handler.add_material(
        &mut game_assets.aguas_logo_texture,
        "textures/aguas_icon.png",
        true,
    );

    assets_handler.add_material(
        &mut game_assets.level_background,
        "textures/no_manches.png",
        true,
    );

    assets_handler.add_material(&mut game_assets.heart_texture, "textures/heart.png", true);
}

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    game_assets: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut collidables: ResMut<collision::Collidables>,
    mut game_state: ResMut<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
    mut clear_color: ResMut<ClearColor>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    mut round_end_timer: ResMut<RoundEndTimer>,
    inspector_data: Res<inspect::InspectorData>,
) {
    camera_settings.set_camera(20.0, Vec3::ZERO, 0.4, false, 0.5, 30.0);

    game_state.current_level_over = false;
    game_state.on_new_level();

    round_end_timer.cooldown = 0.0;
    round_end_timer.round_ending = false;

    // SETTING LEVEL BACKGROUND
    *clear_color = match game_state.current_level {
        0 => ClearColor(Color::rgb(0.55, 0.92, 0.96)), //light blue
        1 => ClearColor(Color::rgb(1.0, 0.65, 0.62)),  // orange
        2 => ClearColor(Color::rgb(0.72, 0.98, 0.75)), // green
        3 => ClearColor(Color::rgb(0.81, 0.72, 0.94)), // purple
        4 => ClearColor(Color::rgb(1.0, 0.65, 0.62)),  // orange
        5 => ClearColor(Color::rgb(0.72, 0.98, 0.75)), // green
        6 => ClearColor(Color::rgb(0.81, 0.72, 0.94)), // purple
        _ => ClearColor(Color::rgb(1.0, 0.65, 0.62)),
    };
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    if let Some(gltf) = assets_gltf.get(&game_assets.level) {
        let parent = commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .insert(CleanupMarker)
            .id();
        scene_spawner.spawn_as_child(gltf.scenes[0].clone(), parent);
    }

    //  commands.spawn_bundle(mesh::MeshBuilder::plane_repeating(
    //      &mut meshes,
    //      &mut images,
    //      &game_assets.level_background,
    //      80.0,
    //      Some(Vec3::new(0.0, -3.0, 0.0)),
    //      Some(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
    //  ))
    //  .insert(CleanupMarker)
    //  .insert_bundle(mesh::MeshBuilder::add_scrolling_bundle(-Vec3::X * 3.0));

    use game_state::BurroSkin::*;
    let level_positions: HashMap<usize, HashMap<game_state::BurroSkin, (f32, f32)>> =
        HashMap::from([
            // level 1
            (
                0,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-14.0, 14.0)),
                    (Salud, (14.0, -14.0)),
                    (Mexico, (14.0, 14.0)),
                    (Medianoche, (-4.0, -4.0)),
                    (Morir, (-4.0, 4.0)),
                    (Gators, (4.0, -4.0)),
                    (Aguas, (4.0, 4.0)),
                ]),
            ),
            // level 2
            (
                1,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-14.0, 14.0)),
                    (Salud, (14.0, -14.0)),
                    (Mexico, (14.0, 14.0)),
                    (Medianoche, (-8.0, -8.0)),
                    (Morir, (-8.0, 8.0)),
                    (Gators, (8.0, -8.0)),
                    (Aguas, (8.0, 8.0)),
                ]),
            ),
            // level 3
            (
                2,
                HashMap::from([
                    (Pinata, (-14.0, -4.0)),
                    (Meow, (-12.0, 4.0)),
                    (Salud, (14.0, -4.0)),
                    (Mexico, (12.0, 4.0)),
                    (Medianoche, (-6.0, -8.0)),
                    (Morir, (-6.0, 8.0)),
                    (Gators, (6.0, -10.0)),
                    (Aguas, (6.0, 10.0)),
                ]),
            ),
            // level 4
            (
                3,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-14.0, 14.0)),
                    (Salud, (-5.0, -13.0)),
                    (Mexico, (-5.0, 13.0)),
                    (Medianoche, (-6.0, 0.0)),
                    (Morir, (8.0, 0.0)),
                    (Gators, (0.0, -8.0)),
                    (Aguas, (0.0, 8.0)),
                ]),
            ),
            // level 5
            (
                4,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-6.0, 14.0)),
                    (Salud, (6.0, -3.0)),
                    (Mexico, (6.0, 3.0)),
                    (Medianoche, (-6.0, 0.0)),
                    (Morir, (6.0, 0.0)),
                    (Gators, (0.0, -8.0)),
                    (Aguas, (14.0, 12.0)),
                ]),
            ),
            // level 6
            (
                5,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-14.0, 14.0)),
                    (Salud, (14.0, -14.0)),
                    (Mexico, (14.0, 14.0)),
                    (Medianoche, (-8.0, 0.0)),
                    (Morir, (8.0, 0.0)),
                    (Gators, (0.0, -8.0)),
                    (Aguas, (0.0, 8.0)),
                ]),
            ),
            // level 7
            (
                6,
                HashMap::from([
                    (Pinata, (-14.0, -14.0)),
                    (Meow, (-14.0, 14.0)),
                    (Salud, (14.0, -14.0)),
                    (Mexico, (14.0, 14.0)),
                    (Medianoche, (-4.0, -4.0)),
                    (Morir, (-4.0, 4.0)),
                    (Gators, (4.0, -4.0)),
                    (Aguas, (4.0, 4.0)),
                ]),
            ),
        ]);

    game_state.burros.iter().for_each(|b| {
        let (skin, position) = match b.skin {
            Pinata => (
                &game_assets.pinata_texture.material,
                level_positions[&game_state.current_level][&Pinata],
            ),
            Meow => (
                &game_assets.meow_texture.material,
                level_positions[&game_state.current_level][&Meow],
            ),
            Salud => (
                &game_assets.salud_texture.material,
                level_positions[&game_state.current_level][&Salud],
            ),
            Mexico => (
                &game_assets.mexico_texture.material,
                level_positions[&game_state.current_level][&Mexico],
            ),
            Medianoche => (
                &game_assets.medianoche_texture.material,
                level_positions[&game_state.current_level][&Medianoche],
            ),
            Morir => (
                &game_assets.morir_texture.material,
                level_positions[&game_state.current_level][&Morir],
            ),
            Gators => (
                &game_assets.gators_texture.material,
                level_positions[&game_state.current_level][&Gators],
            ),
            Aguas => (
                &game_assets.aguas_texture.material,
                level_positions[&game_state.current_level][&Aguas],
            ),
        };

        let burro_bundle = PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: skin.clone(),
            transform: {
                let mut transform = Transform::from_xyz(position.0, 1.0, position.1);
                transform.rotation = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI);
                transform
            },
            ..Default::default()
        };

        if b.is_bot {
            commands
                .spawn_bundle(burro_bundle)
                .insert(CleanupMarker)
                .insert_bundle(bot::BotBundle::new(b.skin, inspector_data.burro_speed));
        } else {
            let entity = commands
                .spawn_bundle(burro_bundle)
                .insert(CleanupMarker)
                .insert_bundle(player::PlayerBundle::new(
                    b.skin,
                    inspector_data.burro_speed,
                ))
                .id();

            let player_map = game_state.get_skin_player_map();
            let (text, color) = match player_map[&b.skin] {
                1 => ("P2", Color::GREEN),
                2 => ("P3", Color::RED),
                3 => ("P4", Color::BLUE),
                _ => ("P1", Color::YELLOW),
            };
            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(5.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },
                        size: Size {
                            width: Val::Px(200.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        text.to_string(),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 40.0,
                            color,
                        },
                        TextAlignment {
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(CleanupMarker)
                .insert(follow_text::FollowText { following: entity });
        }
    });

    collidables.reset();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position_type: PositionType::Absolute,
                margin: Rect {
                    top: Val::Px(20.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    visibility: Visibility { is_visible: false },
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            bottom: Val::Auto,
                            top: Val::Auto,
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Round ending in 5 seconds".to_string(),
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(RoundEndText)
                .insert(CleanupMarker);
        });

    app_state.push(AppState::MechaPicker).unwrap();
}
