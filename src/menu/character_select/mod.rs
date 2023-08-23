// TODO: Refactor this into separate files

use crate::loading::command_ext::*;
use crate::util::num_ext::*;
use crate::{
    assets, assets::GameAssets, audio, burro, cleanup,
    game_state, input, shaders, ui, AppState,
};
use bevy:: prelude::*;
use bevy_mod_outline::{
    AutoGenerateOutlineNormalsPlugin, OutlinePlugin, OutlineVolume,
};
use bevy_toon_shader::ToonShaderMaterial;
use leafwing_input_manager::prelude::*;

pub mod loader;
pub mod state;
mod update_ui;

use self::{
    state::{ PlayerSelection, PlayerSelectionState, SelectionState }
};

pub struct CharacterSelectPlugin;
impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSelection>()
            .add_plugins((OutlinePlugin, AutoGenerateOutlineNormalsPlugin))
            .add_systems(OnEnter(AppState::CharacterSelect), setup)
            .add_systems(
                Update,
                (
                    rotate_burros,
                    (
                        handle_input,
                        force_burro_uniqueness,
                        (
                            update_selections,
                            update_burro_visibility,
                            update_burro_material,
                            update_burro_outline,
                            update_selected_burro_texts,
                            update_center_texts,
                            update_selection_containers,
                        ),
                    )
                        .chain(),
                )
                    .run_if(in_state(AppState::CharacterSelect)),
            )
            .add_systems(OnExit(AppState::CharacterSelect), cleanup::<CleanupMarker>);
    }
}

const BORDER_COLOR: Color = Color::WHITE;
const SELECTION_BACKGROUND_COLOR: Color = Color::rgba(0., 0., 0., 0.5);
const BACKGROUND_COLOR: Color = Color::NONE;

#[derive(Component, Clone)]
pub struct CleanupMarker;

const COLOR: f32 = 255.;
const COLOR_COUNT: usize = 24;
const OUTLINE_COLORS: [Color; COLOR_COUNT] = [
    Color::rgb(112. / COLOR, 214. / COLOR, 255. / COLOR),
    Color::rgb(255. / COLOR, 251. / COLOR, 204. / COLOR),
    Color::rgb(90. / COLOR, 232. / COLOR, 110. / COLOR),
    Color::rgb(145. / COLOR, 90. / COLOR, 232. / COLOR),
    Color::rgb(255. / COLOR, 246. / COLOR, 87. / COLOR),
    Color::rgb(179. / COLOR, 114. / COLOR, 43. / COLOR),
    Color::rgb(255. / COLOR, 187. / COLOR, 112. / COLOR),
    Color::rgb(99. / COLOR, 255. / COLOR, 214. / COLOR),
    Color::rgb(243. / COLOR, 138. / COLOR, 255. / COLOR),
    Color::rgb(38. / COLOR, 70. / COLOR, 83. / COLOR),
    Color::rgb(42. / COLOR, 157. / COLOR, 143. / COLOR),
    Color::rgb(233. / COLOR, 196. / COLOR, 106. / COLOR),
    Color::rgb(244. / COLOR, 162. / COLOR, 97. / COLOR),
    Color::rgb(231. / COLOR, 111. / COLOR, 81. / COLOR),
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

#[derive(Component)]
pub struct SelectedBurroMarker;

fn update_selection_containers(
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut selection_containers: Query<
        (&mut Visibility, &game_state::PlayerMarker),
        With<SelectionContainerMarker>,
    >,
) {
    for (player_state, player) in &players {
        let (mut visibility, _) = selection_containers
            .iter_mut()
            .filter(|(_, p)| *p == player)
            .last()
            .unwrap();
        *visibility = match player_state.state {
            SelectionState::Burro | SelectionState::OutlineColor => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

fn update_burro_material(
    game_assets: Res<GameAssets>,
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut materials: Query<(&mut Handle<ToonShaderMaterial>, &game_state::PlayerMarker)>,
) {
    for (player_state, player) in &players {
        if let Some((mut material, _)) = materials.iter_mut().filter(|(_, p)| *p == player).last() {
            *material = game_assets.burro_assets[player_state.burro]
                .toon_texture
                .clone();
        }
    }
}

fn update_burro_outline(
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut outlines: Query<(&mut OutlineVolume, &game_state::PlayerMarker)>,
) {
    for (player_state, player) in &players {
        if let Some((mut outline, _)) = outlines.iter_mut().filter(|(_, p)| *p == player).last() {
            outline.colour = OUTLINE_COLORS[player_state.outline_color];
        }
    }
}

fn update_burro_visibility(
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut images: Query<(&mut Visibility, &game_state::PlayerMarker), With<UiImage>>,
) {
    for (player_state, player) in &players {
        let (mut image_visibility, _) = images
            .iter_mut()
            .filter(|(_, p)| *p == player)
            .last()
            .unwrap();
        match player_state.state {
            SelectionState::NotPlaying => {
                *image_visibility = Visibility::Hidden;
            }
            _ => {
                *image_visibility = Visibility::Visible;
            }
        }
    }
}

fn update_selected_burro_texts(
    game_assets: Res<GameAssets>,
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut selected_burro_texts: Query<
        (&mut Text, &game_state::PlayerMarker),
        With<SelectedBurroMarker>,
    >,
) {
    for (player_state, player) in &players {
        let mut text = selected_burro_texts
            .iter_mut()
            .filter(|(_, p)| *p == player)
            .map(|(t, _)| t)
            .last()
            .unwrap();
        match player_state.state {
            SelectionState::NotPlaying | SelectionState::Burro => {
                text.sections[0].value = "".to_string();
            }
            _ => {
                text.sections[0].value = game_assets.burro_assets[player_state.burro].name.clone();
            }
        }
    }
}

fn update_center_texts(
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut center_texts: Query<(&mut Text, &game_state::PlayerMarker), With<CenterTextMarker>>,
) {
    for (player_state, player) in &players {
        let mut center_text = center_texts
            .iter_mut()
            .filter(|(_, p)| *p == player)
            .map(|(t, _)| t)
            .last()
            .unwrap();
        match player_state.state {
            SelectionState::NotPlaying => {
                center_text.sections[0].value = "Press Start!".to_string();
            }
            SelectionState::Ready => {
                center_text.sections[0].value = "Ready!".to_string();
            }
            _ => {
                center_text.sections[0].value = "".to_string();
            }
        }
    }
}

fn update_selections(
    game_assets: Res<GameAssets>,
    players: Query<(&PlayerSelectionState, &game_state::PlayerMarker)>,
    mut selection_texts: Query<(&mut Text, &game_state::PlayerMarker), With<SelectionMarker>>,
) {
    for (player_state, player) in &players {
        let mut selection_text = selection_texts
            .iter_mut()
            .filter(|(_, p)| *p == player)
            .map(|(t, _)| t)
            .last()
            .unwrap();
        match player_state.state {
            SelectionState::Burro => {
                selection_text.sections[0].value =
                    game_assets.burro_assets[player_state.burro].name.clone();
                selection_text.sections[0].style.color = Color::WHITE;
            }
            SelectionState::OutlineColor => {
                selection_text.sections[0].value = "OUTLINE\nCOLOR".to_string();
                selection_text.sections[0].style.color = OUTLINE_COLORS[player_state.outline_color];
            }
            _ => (),
        }
    }
}

fn force_burro_uniqueness(
    mut players: Query<&mut PlayerSelectionState>,
    game_assets: Res<assets::GameAssets>,
) {
    let selected_burros = players
        .iter()
        .filter(|p| p.state.has_selected_burro())
        .map(|p| p.burro)
        .collect::<Vec<_>>();
    let number_of_burros = game_assets.burro_assets.len() - 1;

    for mut player in &mut players {
        if !player.state.has_selected_burro() {
            while selected_burros.contains(&player.burro) {
                player.burro = player.burro.circular_increment(0, number_of_burros);
            }
        }
    }
}

fn handle_input(
    mut commands: Commands,
    mut players: Query<(
        &mut PlayerSelectionState,
        &game_state::PlayerMarker,
        &ActionState<input::MenuAction>,
    )>,
    game_assets: Res<assets::GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut audio: audio::GameAudio,
    mut player_selection: ResMut<PlayerSelection>,

    #[cfg(feature = "debug")] mut selected_player: Local<usize>,
    #[cfg(feature = "debug")] keys: Res<Input<KeyCode>>,
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
    let mut selection_completed = false;
    let number_of_burros = game_assets.burro_assets.len() - 1;
    let selected_burros = players
        .iter()
        .filter(|(p, _, _)| p.state.has_selected_burro())
        .map(|(p, _, _)| p.burro)
        .collect::<Vec<_>>();
    let (playing_count, ready_count) = players.iter().fold((0, 0), |mut acc, (p, _, _)| {
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
            if game_state::PlayerMarker(*selected_player) != *player {
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
            }
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

                    player_selection.burro = player_selection
                        .burro
                        .circular_increment(0, number_of_burros);
                }
                if action_state.just_pressed(input::MenuAction::Left) {
                    play_audio = true;

                    // this is only needed when moving left because the force
                    // unique system will move selections to the right
                    loop {
                        player_selection.burro = player_selection
                            .burro
                            .circular_decrement(0, number_of_burros);
                        if !selected_burros.contains(&player_selection.burro) {
                            break;
                        }
                    }
                }
            }
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
                    player_selection.outline_color = player_selection
                        .outline_color
                        .circular_increment(0, COLOR_COUNT - 1);
                }
                if action_state.just_pressed(input::MenuAction::Left) {
                    play_audio = true;
                    player_selection.outline_color = player_selection
                        .outline_color
                        .circular_decrement(0, COLOR_COUNT - 1);
                }
            }
            SelectionState::Ready => {
                if action_state.just_pressed(input::MenuAction::Back) {
                    play_audio = true;
                    player_selection.state = SelectionState::OutlineColor;
                }
                if action_state.just_pressed(input::MenuAction::Select)
                    && ready_count == playing_count
                {
                    play_audio = true;
                    selection_completed = true;
                }
            }
        }
    }

    if play_audio {
        audio.play_sfx(&game_assets.sfx_1);
    }

    if selection_completed {
        player_selection.players = players
            .iter()
            .filter(|(p, _, _)| p.state == SelectionState::Ready)
            .map(|(p, m, _)| (*p, *m))
            .collect::<Vec<_>>();
        commands.load_state(AppState::Settings);
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    text_scaler: ui::text_size::TextScaler,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shader_materials: shaders::ShaderMaterials,
    mut player_selection: ResMut<PlayerSelection>,
) {
    *player_selection = PlayerSelection::default();

    let mut burro_image_handles: Vec<Handle<Image>> = vec![];
    for i in 0..8 {
        let image = ui::render_to_texture::create_render_image(&window_size);
        let image_handle = images.add(image);
        burro_image_handles.push(image_handle.clone());
        let y_offset = 10.0;
        let burro_transform = Transform::from_xyz(0.0, i as f32 * y_offset, 0.0);
        let camera_transform = Transform::from_xyz(0.0, i as f32 * y_offset, 8.0)
                .looking_at(Vec3::new(0.0, i as f32 * y_offset, 0.0), Vec3::Y);

        commands.add(ui::render_to_texture::BurroImage {
            player: i,
            selected_burro: i,
            burro_transform,
            camera_transform,
            outline_color: OUTLINE_COLORS[0],
            outline_size: 5.0,
            clear_color: Color::NONE,
            render_layer_id: 1,
            cleanup_marker: CleanupMarker,
            image_handle: image_handle.clone(),
        });
    }

    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_xyz(0.0, -50.0, 0.0),
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: shader_materials
                .scrolling_images
                .add(shaders::ScrollingImageMaterial {
                    texture: game_assets.title_screen_background.image.clone(),
                }),
            ..default()
        },
        CleanupMarker,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, -46.0, 0.0)
                .looking_at(Vec3::new(0.0, -50.0, 0.0), -Vec3::Z),
            ..default()
        },
        CleanupMarker,
    ));

    for i in 0..8 {
        commands.spawn((
            PlayerSelectionState::default(),
            game_state::PlayerMarker(i),
            CleanupMarker,
            input::create_menu_input_for_player(i),
        ));
    }

    let root_node = commands
        .spawn((
            NodeBundle {
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
            },
            CleanupMarker,
        ))
        .id();

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

    let selection_container = commands
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
        },))
        .with_children(|builder| {
            for row in 0..2 {
                builder
                    .spawn(NodeBundle {
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
                        ..default()
                    })
                    .with_children(|builder| {
                        for column in 0..4 {
                            let player_index = column + (row * 4);

                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(25.),
                                        height: Val::Percent(100.),
                                        margin: UiRect {
                                            left: Val::Percent(2.5),
                                            right: Val::Percent(2.5),
                                            ..default()
                                        },
                                        border: UiRect::all(Val::Percent(0.2)),
                                        ..default()
                                    },
                                    border_color: BORDER_COLOR.into(),
                                    background_color: SELECTION_BACKGROUND_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|builder| {
                                    builder.spawn((
                                        ImageBundle {
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
                                        game_state::PlayerMarker(player_index),
                                    ));

                                    builder
                                        .spawn(NodeBundle {
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
                                        })
                                        .with_children(|builder| {
                                            builder.spawn((
                                                TextBundle {
                                                    text: Text::from_section(
                                                        "",
                                                        TextStyle {
                                                            font: game_assets.score_font.clone(),
                                                            font_size: text_scaler
                                                                .scale(ui::DEFAULT_FONT_SIZE),
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    ..default()
                                                },
                                                CenterTextMarker,
                                                game_state::PlayerMarker(player_index),
                                            ));
                                        });

                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.),
                                                height: Val::Percent(10.),
                                                padding: UiRect::all(Val::Percent(5.)),
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
                                                        font_size: text_scaler
                                                            .scale(ui::DEFAULT_FONT_SIZE),
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            });
                                        });

                                    builder
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.),
                                                height: Val::Percent(10.),
                                                margin: UiRect { ..default() },
                                                padding: UiRect::all(Val::Percent(5.)),
                                                position_type: PositionType::Absolute,
                                                display: Display::Flex,
                                                align_items: AlignItems::FlexStart,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            builder.spawn((
                                                TextBundle {
                                                    text: Text::from_section(
                                                        "",
                                                        TextStyle {
                                                            font: game_assets.score_font.clone(),
                                                            font_size: text_scaler
                                                                .scale(ui::DEFAULT_FONT_SIZE),
                                                            color: Color::WHITE,
                                                        },
                                                    ),
                                                    ..default()
                                                },
                                                game_state::PlayerMarker(player_index),
                                                SelectedBurroMarker,
                                            ));
                                        });

                                    builder
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(100.),
                                                    height: Val::Percent(100.),
                                                    padding: UiRect::all(Val::Percent(5.)),
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
                                            },
                                            SelectionContainerMarker,
                                            game_state::PlayerMarker(player_index),
                                        ))
                                        .with_children(|builder| {
                                            builder.spawn((TextBundle {
                                                text: Text::from_section(
                                                    "<",
                                                    TextStyle {
                                                        font: game_assets.score_font.clone(),
                                                        font_size: text_scaler
                                                            .scale(ui::DEFAULT_FONT_SIZE),
                                                        color: Color::WHITE,
                                                    },
                                                ),
                                                ..default()
                                            },));

                                            builder.spawn((
                                                TextBundle {
                                                    text: Text::from_section(
                                                        "",
                                                        TextStyle {
                                                            font: game_assets.score_font.clone(),
                                                            font_size: text_scaler
                                                                .scale(ui::DEFAULT_FONT_SIZE),
                                                            color: Color::WHITE,
                                                        },
                                                    )
                                                    .with_alignment(TextAlignment::Center),
                                                    ..default()
                                                },
                                                SelectionMarker,
                                                game_state::PlayerMarker(player_index),
                                            ));

                                            builder.spawn((TextBundle {
                                                text: Text::from_section(
                                                    ">",
                                                    TextStyle {
                                                        font: game_assets.score_font.clone(),
                                                        font_size: text_scaler
                                                            .scale(ui::DEFAULT_FONT_SIZE),
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

fn rotate_burros(mut burros: Query<&mut Transform, With<burro::BurroMeshMarker>>, time: Res<Time>) {
    for mut transform in &mut burros {
        transform.rotate_y(time.delta_seconds());
    }
}
