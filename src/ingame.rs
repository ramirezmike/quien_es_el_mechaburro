use bevy::prelude::*;
use bevy::gltf::Gltf;
use crate::{
    asset_loading,
    assets,
    AppState,
    game_camera,
};
use bevy_toon_shader::{ ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
//          .add_systems((
//              ).chain()
//              .in_set(OnUpdate(AppState::InGame))
//          );
            ;
    }
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<assets::GameAssets>,
) {
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
    assets_handler.add_mesh(
        &mut game_assets.burro.mesh,
        "models/burro.gltf#Mesh0/Primitive0",
    );
    assets_handler.add_material(
        &mut game_assets.pinata_texture,
        "textures/pinata.png",
        false,
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear_color: ResMut<ClearColor>,
    asset_server: Res<AssetServer>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
//    clear_color.0 = Color::hex(BACKGROUND_COLOR).unwrap();

//  if let Some(gltf) = assets_gltf.get(&game_assets.TJ) {
//      commands
//          .spawn((
//              RigidBody::KinematicPositionBased,
//              Collider::cuboid(0.25, 0.25, 0.25),
//              CleanupMarker,
//              ColliderMassProperties::Density(2.0),
//              KinematicCharacterController {
//                  translation: Some(Vec3::new(0.0, 0.5, 0.0)),
//                  offset: CharacterLength::Absolute(0.01),
//                  autostep: Some(CharacterAutostep {
//                      max_height: CharacterLength::Absolute(1.0),
//                      min_width: CharacterLength::Absolute(0.05),
//                      include_dynamic_bodies: true,
//                  }),
//                  ..default()
//              },
//             Velocity::default(),
//  //        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED_Y,
//          ComputedVisibility::default(),
//          Visibility::Visible,
//          TransformBundle {
//              local: Transform::from_xyz(0.0, 0.5, 0.0),
//              ..default()
//          },
//          player::PlayerBundle::new(),
//      )).with_children(|parent| {
//          parent.spawn((SceneBundle { scene: gltf.scenes[0].clone(), ..default() }, player::InnerMesh));
//      });
//  }
        
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });


    game_camera::spawn_camera(&mut commands, CleanupMarker);

    commands.spawn((DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::new(-0.8263363, -0.53950554, -0.16156079), 2.465743)),
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
//            illuminance: 10000.0,
            illuminance: 100000.0,
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    }, ToonShaderSun));

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let toon_material_textured = toon_materials.add(ToonShaderMaterial {
        base_color_texture: Some(game_assets.pinata_texture.image.clone()),
        color: Color::default(),
        sun_dir: Vec3::new(10.0, 0.5, 23.0),
        sun_color: Color::default(),
        camera_pos: Vec3::default(),
        ambient_color: Color::default(),
    });

    commands.spawn(MaterialMeshBundle {
        mesh: game_assets.burro.mesh.clone(),
        material: toon_material_textured.clone(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

#[derive(Component)]
struct CleanupMarker;
