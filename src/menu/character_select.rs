use crate::{
    game_camera, asset_loading, assets::GameAssets, audio::GameAudio, cleanup, game_state, ui, AppState, scene_hook,
};
use bevy_mod_outline::{
    AutoGenerateOutlineNormalsPlugin, OutlineBundle, OutlinePlugin, OutlineVolume,
};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    gltf::Gltf,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use leafwing_input_manager::prelude::*;

pub struct CharacterSelectPlugin;
impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::CharacterSelect), setup)
            .add_systems(Update, 
                 (rotate_burros,).run_if(in_state(AppState::CharacterSelect))
             )
            .add_systems(OnExit(AppState::CharacterSelect), cleanup::<CleanupMarker>);
    }
}


#[derive(Component)]
pub struct CleanupMarker;

#[derive(Component)]
pub struct BurroMeshMarker;

#[derive(Component, Clone)]
pub struct BurroName {
    player: usize,
}

#[derive(Component)]
struct CameraImage {
    id: usize,
    image: Handle<Image>,
}

#[derive(Component)]
struct RenderedImage {
    id: usize,
}


#[derive(Component)]
pub enum GamePlayer {
    One,
    Two,
    Three,
    Four,
}

fn rotate_burros(
    mut burros: Query<&mut Transform, With<BurroMeshMarker>>,
//  mut cameras: Query<(&mut Camera, &mut Transform, &CameraImage)>,
//  mut rendered_images: Query<(&mut UiImage, &RenderedImage)>,
    time: Res<Time>
) {
    for mut transform in &mut burros {
        transform.rotate_y(time.delta_seconds());
    }

//  for (mut camera, mut transform, camera_image) in &mut cameras {
//      transform.rotate_y(time.delta_seconds());
 //     camera.target = RenderTarget::Image(camera_image.image.clone());
 //     println!("Upddated target {}", camera_image.id);
 //     for (mut ui_image, rendered_image) in &mut rendered_images {
 //         if rendered_image.id == camera_image.id {
 //             println!("Upddated image {}", rendered_image.id);
 //             ui_image.texture = camera_image.image.clone();
 //         }
 //     }
//    }
}

fn create_render_image(window_size: &Res<ui::text_size::WindowSize>) -> Image {
    let size = Extent3d {
        width: (window_size.width / 4.0) as u32,
        height: (window_size.width / 4.0) as u32,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    image
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    text_scaler: ui::text_size::TextScaler,
    assets_gltf: Res<Assets<Gltf>>,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
) {
    let burro_mesh_handle = game_assets.burro.clone();
    let first_pass_layer = RenderLayers::layer(1);

    let mut image_handles: Vec::<Handle<Image>> = vec!();
    for i in 0..8 {
        let image = create_render_image(&window_size); 
        let image_handle = images.add(image);
        image_handles.push(image_handle.clone());
        let toon_material_textured = game_assets.burro_assets[i]
            .toon_texture
            .clone();

        let y_offset = 10.0;

        if let Some(gltf) = assets_gltf.get(&burro_mesh_handle) {
            commands.spawn((BurroMeshMarker, scene_hook::HookedSceneBundle {
                scene: SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_xyz(0.0, i as f32 * y_offset, 0.0),
                    ..default()
                },
                hook: scene_hook::SceneHook::new(move |cmds, hook_data| {
                    if let Some(name) = hook_data.name {
                        let name = name.as_str();
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
                                first_pass_layer,
                                toon_material_textured.clone(),
                            ));
                        }
                    }
                }),
            }));
        }

        commands.spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, i as f32 * y_offset, 8.0))
                    .looking_at(Vec3::new(0.0, i as f32 * y_offset, 0.0), Vec3::Y),
                ..default()
            },
            CameraImage {
                id: i,
                image: image_handle.clone()
            }, 
            UiCameraConfig {
                show_ui: false,
            },
            first_pass_layer,
        ));
    }

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, CleanupMarker));

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });


    let root_node = 
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, CleanupMarker)).id();

    let title_text = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(10.),
                margin: UiRect {
                    top: Val::Percent(1.),
                    ..default()
                },
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "Choose your Burro!",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 1.2),
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        })
        .id();

    let selection_container =
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(90.),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }, CleanupMarker))
    .with_children(|builder| {
        for row in 0..2 {
            builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .with_children(|builder| {
                for column in 0..4 {
                    let player_index = column + (row * 4); 

                    builder.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(25.),
                            height: Val::Percent(100.),
                            border: UiRect::all(Val::Percent(1.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    }).with_children(|builder| {
                        let image_handle = &image_handles[player_index as usize];
                        builder.spawn((ImageBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Auto,
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            image: image_handle.clone().into(),
                            ..default()
                        }, RenderedImage {
                            id: player_index as usize
                        }));

                        builder.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(10.),
                                margin: UiRect {
                                    ..default()
                                },
                                display: Display::Flex,
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::FlexStart,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("P{}", player_index + 1),
                                    TextStyle {
                                        font: game_assets.score_font.clone(),
                                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            });
                        });
                    });
                }
            });
        }
    })
    .id();


    commands.entity(root_node).add_child(title_text);
    commands.entity(root_node).add_child(selection_container);
}
