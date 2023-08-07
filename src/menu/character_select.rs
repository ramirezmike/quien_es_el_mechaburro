use crate::{
    assets, audio, input, game_camera, asset_loading, assets::GameAssets, audio::GameAudio, cleanup, game_state, ui, AppState, scene_hook, menu,
};
use crate::input::InputCommandsExt;
use crate::util::num_ext::*;
use crate::loading::command_ext::*;
use bevy_mod_outline::{
    AutoGenerateOutlineNormalsPlugin, OutlineBundle, OutlinePlugin, OutlineVolume,
};
use bevy_toon_shader::ToonShaderMaterial;
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
                 (rotate_burros, 
                  (handle_input, 
                   force_burro_uniqueness,
                   ( 
                       update_selections, 
                       update_burro_visibility,
                       update_burro_material,
                       update_burro_outline,
                       update_center_texts,
                       update_selection_containers,
                    )
                   ).chain() ).run_if(in_state(AppState::CharacterSelect))
             )
            .add_systems(OnExit(AppState::CharacterSelect), cleanup::<CleanupMarker>);
    }
}

const BORDER_COLOR: Color = Color::NONE;
const SELECTION_BACKGROUND_COLOR: Color = Color::rgba(0., 0., 0., 0.5);
const BACKGROUND_COLOR: Color = Color::NONE;

#[derive(Component)]
pub struct CleanupMarker;

#[derive(Component)]
pub struct BurroMeshMarker;

#[derive(Component, Default)]
struct PlayerSelectionState {
    burro: usize,
    outline_color: usize,
    state: SelectionState,
}

#[derive(Default, PartialEq)]
enum SelectionState {
    #[default]
    NotPlaying,
    Burro,
    OutlineColor,
    Ready,
}

impl SelectionState {
    fn has_selected_burro(&self) -> bool {
        match self {
            SelectionState::NotPlaying | SelectionState::Burro => false,
            _ => true
        }
    }
}

const COLOR: f32 = 255.;
const COLOR_COUNT: usize = 16;
const OUTLINE_COLORS: [Color; COLOR_COUNT] = [
   Color::rgb(38. / COLOR, 70. / COLOR, 83. / COLOR),
   Color::rgb(42. / COLOR, 157. / COLOR, 143. / COLOR),
   Color::rgb(233. / COLOR, 196. / COLOR, 106. / COLOR),
   Color::rgb(244. / COLOR, 162. / COLOR, 97. / COLOR),
   Color::rgb(231. / COLOR, 111. / COLOR, 81. / COLOR),
   Color::rgb(112. / COLOR, 214. / COLOR, 255. / COLOR),
   Color::rgb(255. / COLOR, 112. / COLOR, 166. / COLOR),
   Color::rgb(233. / COLOR, 255. / COLOR, 112. / COLOR),
   Color::rgb(59. / COLOR, 53. / COLOR, 97. / COLOR),
   Color::rgb(145. / COLOR, 145. / COLOR, 233. / COLOR),
   Color::rgb(194. / COLOR, 175. / COLOR, 240. / COLOR),
   Color::rgb(89. / COLOR, 248. / COLOR, 232. / COLOR),
   Color::rgb(219. / COLOR, 84. / COLOR, 97. / COLOR),
   Color::rgb(240. / COLOR, 239. / COLOR, 235. / COLOR),
   Color::rgb(98. / COLOR, 98. / COLOR, 95. / COLOR),
   Color::rgb(62. / COLOR, 165. / COLOR, 106. / COLOR),
];


#[derive(Component)]
pub struct CenterTextMarker;

#[derive(Component)]
pub struct SelectionMarker;

#[derive(Component)]
pub struct SelectionContainerMarker;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum PlayerMarker {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

const _: () = {
    const PLAYER_MARKERS: [PlayerMarker;8] = [
        PlayerMarker::One,
        PlayerMarker::Two,
        PlayerMarker::Three,
        PlayerMarker::Four,
        PlayerMarker::Five,
        PlayerMarker::Six,
        PlayerMarker::Seven,
        PlayerMarker::Eight,
    ];

    impl PlayerMarker {
        fn get(index: usize) -> Self {
            PLAYER_MARKERS[index]
        }
    }
};

fn update_selection_containers(
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut selection_containers: Query<(&mut Visibility, &PlayerMarker), With<SelectionContainerMarker>>,
) {
    for (player_state, player) in &players {
        let (mut visibility, _) = selection_containers.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .last()
                                         .unwrap();
        *visibility = 
        match player_state.state {
            SelectionState::Burro | SelectionState::OutlineColor => Visibility::Visible, 
            _ => Visibility::Hidden,
        };
    }
}

fn update_burro_material(
    game_assets: Res<GameAssets>,
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut materials: Query<(&mut Handle<ToonShaderMaterial>, &PlayerMarker)>,
) {
    for (player_state, player) in &players {
        if let Some((mut material, _)) = materials.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .last() {
            *material = game_assets.burro_assets[player_state.burro]
                .toon_texture
                .clone();
        }
    }
}

fn update_burro_outline(
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut outlines: Query<(&mut OutlineVolume, &PlayerMarker)>,
) {
    for (player_state, player) in &players {
        if let Some((mut outline, _)) = outlines.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .last() {
            outline.colour = OUTLINE_COLORS[player_state.outline_color];
        }
    }
}

fn update_burro_visibility(
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut images: Query<(&mut Visibility, &PlayerMarker), With<UiImage>>,
) {
    for (player_state, player) in &players {
        let (mut image_visibility, _) = images.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .last()
                                         .unwrap();
        match player_state.state {
            SelectionState::NotPlaying => {
                *image_visibility = Visibility::Hidden;
            },
            _ => {
                *image_visibility = Visibility::Visible;
            }
        }
    }
}

fn update_center_texts(
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut center_texts: Query<(&mut Text, &PlayerMarker), Without<SelectionMarker>>,
) {
    for (player_state, player) in &players {
        let mut center_text = center_texts.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .map(|(t, _)| t)
                                         .last()
                                         .unwrap();
        match player_state.state {
            SelectionState::NotPlaying => {
                center_text.sections[0].value = "Press Start!".to_string();
            },
            SelectionState::Ready => {
                center_text.sections[0].value = "Ready!".to_string();
            },
            _ => {
                center_text.sections[0].value = "".to_string();
            }
        }
    }
}

fn update_selections(
    game_assets: Res<GameAssets>,
    players: Query<(&PlayerSelectionState, &PlayerMarker)>,
    mut selection_texts: Query<(&mut Text, &PlayerMarker), With<SelectionMarker>>,
) {
    for (player_state, player) in &players {
        let mut selection_text = selection_texts.iter_mut()
                                         .filter(|(_, p)| *p == player)
                                         .map(|(t, _)| t)
                                         .last()
                                         .unwrap();
        match player_state.state {
            SelectionState::Burro => {
                selection_text.sections[0].value = game_assets.burro_assets[player_state.burro].name.clone();
                selection_text.sections[0].style.color = Color::WHITE;
            },
            SelectionState::OutlineColor => {
                selection_text.sections[0].value = "COLOR".to_string();
                selection_text.sections[0].style.color = OUTLINE_COLORS[player_state.outline_color];
            },
            _ => ()
        }
    }
}

fn force_burro_uniqueness(
    mut players: Query<&mut PlayerSelectionState>,
    game_assets: Res<assets::GameAssets>,
) {
    let selected_burros = players.iter()
                                .filter(|p| p.state.has_selected_burro())
                                .map(|p| p.burro)
                                .collect::<Vec::<_>>();
    let number_of_burros = game_assets.burro_assets.len();

    for mut player in &mut players {
        if !player.state.has_selected_burro() {
            while selected_burros.contains(&player.burro) {
                player.burro = player.burro.add_with_wrap(1, number_of_burros);
            }
        }
    }
}

fn handle_input(
    mut commands: Commands,
    mut players: Query<(&mut PlayerSelectionState, &PlayerMarker, &ActionState<input::MenuAction>)>,
    game_assets: Res<assets::GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut audio: audio::GameAudio,
    #[cfg(feature = "debug")]
    mut selected_player: Local<usize>,
    #[cfg(feature = "debug")]
    keys: Res<Input<KeyCode>>,
) {
    #[cfg(feature = "debug")]
    {
        if keys.just_pressed(KeyCode::Key1) {
            *selected_player = 0;
        }
        if keys.just_pressed(KeyCode::Key2) {
            *selected_player = 1;
        }
        if keys.just_pressed(KeyCode::Key3) {
            *selected_player = 2;
        }
        if keys.just_pressed(KeyCode::Key4) {
            *selected_player = 3;
        }
        if keys.just_pressed(KeyCode::Key5) {
            *selected_player = 4;
        }
        if keys.just_pressed(KeyCode::Key6) {
            *selected_player = 5;
        }
        if keys.just_pressed(KeyCode::Key7) {
            *selected_player = 6;
        }
        if keys.just_pressed(KeyCode::Key8) {
            *selected_player = 7;
        }
    }

    let mut play_audio = false;
    let number_of_burros = game_assets.burro_assets.len();
    let selected_burros = players.iter()
                                .filter(|(p, _, _)| p.state.has_selected_burro())
                                .map(|(p, _, _)| p.burro)
                                .collect::<Vec::<_>>();
    let (playing_count, ready_count) = 
    players.iter()
           .fold((0, 0), |mut acc, (p, _, _)| {
               if p.state != SelectionState::NotPlaying {
                   acc.0 += 1;
                   if p.state == SelectionState::Ready {
                       acc.1 += 1;
                   }
               }

               acc
           });

    for (mut player_selection, player, action_state) in &mut players {
        #[cfg(feature = "debug")]
        {
            if PlayerMarker::get(*selected_player) != *player {
                continue;
            }
        }

        match player_selection.state {
            SelectionState::NotPlaying => {
                if action_state.just_pressed(input::MenuAction::Select) {
                    play_audio = true; 
                    player_selection.state = SelectionState::Burro;
                }

                if action_state.just_pressed(input::MenuAction::Back) && playing_count == 0 {
                    commands.load_state(AppState::TitleScreen);
                    play_audio = true; 
                }
            },
            SelectionState::Burro => {
                if action_state.just_pressed(input::MenuAction::Select) {
                    play_audio = true; 
                    player_selection.state = SelectionState::OutlineColor;
                }
                if action_state.just_pressed(input::MenuAction::Back) {
                    play_audio = true; 
                    player_selection.state = SelectionState::NotPlaying;
                }
                if action_state.just_pressed(input::MenuAction::Right) {
                    play_audio = true; 

                    player_selection.burro = player_selection.burro.add_with_wrap(1, number_of_burros);
                }
                if action_state.just_pressed(input::MenuAction::Left) {
                    play_audio = true; 

                    // this is only needed when moving left because the force
                    // unique system will move selections to the right
                    loop {
                        player_selection.burro = player_selection.burro.sub_with_wrap(1, number_of_burros);
                        if !selected_burros.contains(&player_selection.burro) {
                            break;
                        }
                    }
                }
            },
            SelectionState::OutlineColor => {
                if action_state.just_pressed(input::MenuAction::Select) {
                    play_audio = true; 
                    player_selection.state = SelectionState::Ready;
                }
                if action_state.just_pressed(input::MenuAction::Back) {
                    play_audio = true; 
                    player_selection.state = SelectionState::Burro;
                }
                if action_state.just_pressed(input::MenuAction::Right) {
                    play_audio = true; 
                    player_selection.outline_color = player_selection.outline_color.add_with_wrap(1, COLOR_COUNT);
                }
                if action_state.just_pressed(input::MenuAction::Left) {
                    play_audio = true; 
                    player_selection.outline_color = player_selection.outline_color.sub_with_wrap(1, COLOR_COUNT);
                }
            },
            SelectionState::Ready => {
                if action_state.just_pressed(input::MenuAction::Back) {
                    play_audio = true; 
                    player_selection.state = SelectionState::OutlineColor;
                }
                if action_state.just_pressed(input::MenuAction::Select) && ready_count == playing_count {
                    play_audio = true; 
                    commands.load_state(AppState::Settings);
                }
            },
        }
    }

    if play_audio {
        audio.play_sfx(&game_assets.sfx_1);
    }
}

fn create_render_image(window_size: &Res<ui::text_size::WindowSize>) -> Image {
    let size = Extent3d {
        width: (window_size.width / 4.0) as u32,
        height: (window_size.width / 4.0) as u32,
        ..default()
    };

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut scrolling_image_materials: ResMut<Assets<menu::title_screen::ScrollingImageMaterial>>,
) {
    let burro_mesh_handle = game_assets.burro.clone();
    let first_pass_layer = RenderLayers::layer(1);

    let mut burro_image_handles: Vec::<Handle<Image>> = vec!();
    for i in 0..8 {
        let image = create_render_image(&window_size); 
        let image_handle = images.add(image);
        burro_image_handles.push(image_handle.clone());
        let toon_material_textured = game_assets.burro_assets[i]
            .toon_texture
            .clone();

        let y_offset = 10.0;

        if let Some(gltf) = assets_gltf.get(&burro_mesh_handle) {
            commands.spawn((
                BurroMeshMarker, 
                PlayerMarker::get(i),
                CleanupMarker,
                scene_hook::HookedSceneBundle {
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
                                        colour: OUTLINE_COLORS[0],
                                    },
                                    ..default()
                                },
                                first_pass_layer,
                                PlayerMarker::get(i),
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
            UiCameraConfig {
                show_ui: false,
            },
            CleanupMarker,
            first_pass_layer,
        ));
    }

    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_xyz(0.0, -50.0, 0.0),
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: scrolling_image_materials.add(menu::title_screen::ScrollingImageMaterial {
                texture: game_assets.title_screen_background.image.clone(),
            }),
            ..default()
        },
        CleanupMarker,
    ));

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, -46.0, 0.0).looking_at(Vec3::new(0.0, -50.0, 0.0), -Vec3::Z),
        ..default()
    }, CleanupMarker));

    for i in 0..8 {
        commands.spawn((
            PlayerSelectionState::default(),
            PlayerMarker::get(i),
            CleanupMarker,
            input::create_menu_input_for_player(i),
        ));
    }

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
            background_color: BACKGROUND_COLOR.into(),
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
        }, ))
    .with_children(|builder| {
        for row in 0..2 {
            builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        top: Val::Percent(2.5),
                        bottom: Val::Percent(2.5),
                        ..default()
                    },
                    ..default()
                },
                background_color: SELECTION_BACKGROUND_COLOR.into(),
                ..default()
            })
            .with_children(|builder| {
                for column in 0..4 {
                    let player_index = column + (row * 4); 

                    builder.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(25.),
                            height: Val::Percent(100.),
                            margin: UiRect {
                                left: Val::Percent(2.5),
                                right: Val::Percent(2.5),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: SELECTION_BACKGROUND_COLOR.into(),
                        ..default()
                    }).with_children(|builder| {
                        builder.spawn((ImageBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Auto,
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            image: burro_image_handles[player_index].clone().into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        }, 
                        PlayerMarker::get(player_index),));

                        builder.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                position_type: PositionType::Absolute,
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        }).with_children(|builder| {
                            builder.spawn((TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: game_assets.score_font.clone(),
                                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            }, CenterTextMarker, PlayerMarker::get(player_index)));
                        });

                        builder.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(10.),
                                margin: UiRect {
                                    ..default()
                                },
                                position_type: PositionType::Absolute,
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

                        builder.spawn((NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                margin: UiRect {
                                    bottom: Val::Percent(10.),
                                    ..default()
                                },
                                display: Display::Flex,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        }, SelectionContainerMarker, PlayerMarker::get(player_index),))
                        .with_children(|builder| {
                            builder.spawn((TextBundle {
                                text: Text::from_section(
                                    "<",
                                    TextStyle {
                                        font: game_assets.score_font.clone(),
                                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            },));

                            builder.spawn((TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: game_assets.score_font.clone(),
                                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            }, SelectionMarker, PlayerMarker::get(player_index),));

                            builder.spawn((TextBundle {
                                text: Text::from_section(
                                    ">",
                                    TextStyle {
                                        font: game_assets.score_font.clone(),
                                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                        color: Color::WHITE,
                                    },
                                ),
                                ..default()
                            },));
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

fn rotate_burros(
    mut burros: Query<&mut Transform, With<BurroMeshMarker>>,
    time: Res<Time>
) {
    for mut transform in &mut burros {
        transform.rotate_y(time.delta_seconds());
    }
}

