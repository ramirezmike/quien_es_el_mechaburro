use crate::{
    asset_loading, assets::GameAssets, bot, burro::Burro, cleanup, collision, game_camera,
    game_state, mesh, player, AppState,
};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct DebugRoomPlugin;
impl Plugin for DebugRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Debug)
                .with_system(game_camera::spawn_camera)
                .with_system(setup),
        )
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(check_for_next_level))
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup::<CleanupMarker>));
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

#[derive(Component)]
struct CleanupMarker;

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_glb(&mut game_assets.level, "models/level_00.glb");
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collidables: ResMut<collision::Collidables>,
    game_state: Res<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
    mut clear_color: ResMut<ClearColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    *clear_color = ClearColor(Color::rgb(1.0, 0.65, 0.62));
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

    game_state.burros.iter().for_each(|b| {
        let (skin, position) = match b.skin {
            game_state::BurroSkin::Pinata => (&game_assets.pinata_texture.material, (-14.0, -14.0)),
            game_state::BurroSkin::Meow => (&game_assets.meow_texture.material, (-14.0, 14.0)),
            game_state::BurroSkin::Salud => (&game_assets.salud_texture.material, (14.0, -14.0)),
            game_state::BurroSkin::Mexico => (&game_assets.mexico_texture.material, (14.0, 14.0)),
            game_state::BurroSkin::Medianoche => {
                (&game_assets.medianoche_texture.material, (-4.0, -4.0))
            }
            game_state::BurroSkin::Morir => (&game_assets.morir_texture.material, (-4.0, 4.0)),
            game_state::BurroSkin::Gators => (&game_assets.gators_texture.material, (4.0, -4.0)),
            game_state::BurroSkin::Aguas => (&game_assets.aguas_texture.material, (4.0, 4.0)),
        };

        let burro_bundle = PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: skin.clone(),
            transform: Transform::from_xyz(position.0, 1.0, position.1),
            ..Default::default()
        };

        if b.is_bot {
            commands
                .spawn_bundle(burro_bundle)
                .insert(CleanupMarker)
                .insert_bundle(bot::BotBundle::new(b.skin));
        } else {
            commands
                .spawn_bundle(burro_bundle)
                .insert(CleanupMarker)
                .insert_bundle(player::PlayerBundle::new(b.skin));
        }
    });

    collidables.reset();

    app_state.push(AppState::MechaPicker).unwrap();
}
