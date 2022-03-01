use crate::{
    asset_loading, assets::GameAssets, bot, cleanup, collision, game_camera, game_state, mesh,
    player, AppState,
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
        .add_system_set(SystemSet::on_exit(AppState::Debug).with_system(cleanup::<CleanupMarker>))
        .add_system_set(
            SystemSet::on_update(AppState::Debug).with_system(game_camera::pan_orbit_camera),
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
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collidables: ResMut<collision::Collidables>,
    game_state: Res<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
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

    game_state.burros.iter().for_each(|b| {
        let skin = match b.skin {
            game_state::BurroSkin::Pinata => &game_assets.pinata_texture.material,
            game_state::BurroSkin::Meow => &game_assets.meow_texture.material,
            game_state::BurroSkin::Salud => &game_assets.salud_texture.material,
            game_state::BurroSkin::Mexico => &game_assets.mexico_texture.material,
            game_state::BurroSkin::Medianoche => &game_assets.medianoche_texture.material,
            game_state::BurroSkin::Morir => &game_assets.morir_texture.material,
            game_state::BurroSkin::Gators => &game_assets.gators_texture.material,
            game_state::BurroSkin::Aguas => &game_assets.aguas_texture.material,
        };

        let burro_bundle = PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: skin.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
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

fn update() {}
