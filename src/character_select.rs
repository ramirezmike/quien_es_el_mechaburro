use crate::{
    asset_loading, assets::GameAssets, audio::GameAudio, cleanup, game_controller, game_state,
    game_state::BurroSkin, menus, menus::options, title_screen, ui::text_size, AppState,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::collections::HashMap;

pub struct CharacterSelectPlugin;
impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MenuAction>::default())
            .add_system_set(SystemSet::on_enter(AppState::CharacterSelect).with_system(setup))
            .insert_resource(LocalCooldown::default())
            .add_system_set(
                SystemSet::on_update(AppState::CharacterSelect)
                    .with_system(update_character_selection.after("handle_input"))
                    .with_system(
                        handle_controllers
                            .label("handle_input")
                            .after("store_controller_inputs"),
                    )
                    .with_system(handle_labels)
                    .with_system(update_burro_skins),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::CharacterSelect)
                    .with_system(cleanup::<title_screen::CleanupMarker>)
                    .with_system(cleanup::<CharacterSelectCleanupMarker>),
            );
    }
}

#[derive(Default)]
struct LocalCooldown {
    cooldown: f32,
}

#[derive(Component)]
pub struct CharacterSelectCleanupMarker;

#[derive(Component, Clone)]
pub struct BurroName {
    player: usize,
}

#[derive(Component, Clone, Copy)]
pub struct BurroCharacter {
    pub player: usize,
    pub is_playing: bool,
    pub has_picked: bool,
    pub selected_burro: BurroSkin,
    pub action_cooldown: f32,
}

#[derive(Component)]
pub enum GamePlayer {
    One,
    Two,
    Three,
    Four,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Left,
    Right,
    Select,
}
impl MenuAction {
    pub fn default_input_map(player: GamePlayer) -> InputMap<MenuAction> {
        let mut input_map = match player {
            GamePlayer::One => {
                let mut input_map = InputMap::default();
                input_map.insert(MenuAction::Left, KeyCode::A);
                input_map.insert(MenuAction::Right, KeyCode::D);
                input_map.insert(MenuAction::Left, KeyCode::Left);
                input_map.insert(MenuAction::Right, KeyCode::Right);
                input_map.insert(MenuAction::Select, KeyCode::Return);
                input_map
            }
            GamePlayer::Two => InputMap::default(),
            GamePlayer::Three => InputMap::default(),
            GamePlayer::Four => InputMap::default(),
        };

        input_map.build()
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut local_cooldown: ResMut<LocalCooldown>,
    mut clear_color: ResMut<ClearColor>,
    text_scaler: text_size::TextScaler,
) {
    *clear_color = ClearColor(Color::rgb(0.0, 0.0, 0.0));
    local_cooldown.cooldown = 0.2;
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CharacterSelectCleanupMarker);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.00,
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 5.0, -0.0001).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .with_children(|parent| {
            const HALF_SIZE: f32 = 100.0;
            parent.spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    // Configure the projection to better fit the scene
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..Default::default()
                    },
                    shadows_enabled: false,
                    ..Default::default()
                },
                transform: Transform {
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: game_assets.pinata_texture.material.clone(),
            visibility: Visibility { is_visible: true },
            transform: {
                let mut transform = Transform::from_xyz(5.0, -10.0, 2.5);
                //            transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
                transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
                transform
            },
            ..Default::default()
        })
        .insert(BurroCharacter {
            player: 0,
            is_playing: true,
            has_picked: false,
            action_cooldown: 0.2,
            selected_burro: BurroSkin::Pinata,
        })
        .insert_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(GamePlayer::One),
            action_state: ActionState::default(),
        })
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: game_assets.meow_texture.material.clone(),
            visibility: Visibility { is_visible: false },
            transform: {
                let mut transform = Transform::from_xyz(-5.5, -10.0, 2.5);
                transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
                transform
            },
            ..Default::default()
        })
        .insert(BurroCharacter {
            player: 1,
            is_playing: false,
            has_picked: false,
            action_cooldown: 0.2,
            selected_burro: BurroSkin::Meow,
        })
        .insert_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(GamePlayer::Two),
            action_state: ActionState::default(),
        })
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: game_assets.gators_texture.material.clone(),
            visibility: Visibility { is_visible: false },
            transform: {
                let mut transform = Transform::from_xyz(5.0, -10.0, -2.5);
                transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
                transform
            },
            ..Default::default()
        })
        .insert(BurroCharacter {
            player: 2,
            is_playing: false,
            has_picked: false,
            action_cooldown: 0.2,
            selected_burro: BurroSkin::Gators,
        })
        .insert_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(GamePlayer::Three),
            action_state: ActionState::default(),
        })
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(PbrBundle {
            mesh: game_assets.burro.mesh.clone(),
            material: game_assets.aguas_texture.material.clone(),
            visibility: Visibility { is_visible: false },
            transform: {
                let mut transform = Transform::from_xyz(-5.5, -10.0, -2.5);
                transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
                transform
            },
            ..Default::default()
        })
        .insert(BurroCharacter {
            player: 3,
            is_playing: false,
            has_picked: false,
            action_cooldown: 0.2,
            selected_burro: BurroSkin::Aguas,
        })
        .insert_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(GamePlayer::Four),
            action_state: ActionState::default(),
        })
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CharacterSelectCleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(98.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CharacterSelectCleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    options::add_title(
                        parent,
                        game_assets.font.clone(),
                        text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                        "Choose your Burro!",
                        Vec::<BurroName>::new(), // just an empty vec since can't do <impl Trait>
                    );
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(98.0), Val::Percent(28.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            options::add_title(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                "",
                                vec![BurroName { player: 0 }],
                            );
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            options::add_title(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                "",
                                vec![BurroName { player: 1 }],
                            );
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(98.0), Val::Percent(40.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            options::add_title(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                "",
                                vec![BurroName { player: 2 }],
                            );
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            options::add_title(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                "",
                                vec![BurroName { player: 3 }],
                            );
                        });
                });
        });
}

fn update_character_selection(
    mut burros: Query<(
        &mut Transform,
        &mut BurroCharacter,
        &ActionState<MenuAction>,
        &Handle<StandardMaterial>,
    )>,
    time: Res<Time>,
    mut audio: GameAudio,
    mut game_assets: ResMut<GameAssets>,
    game_state: ResMut<game_state::GameState>,
    mut option_state: ResMut<options::OptionState>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut local_cooldown: ResMut<LocalCooldown>,
) {
    let picked_skins = burros
        .iter()
        .filter(|(_, b, _, _)| b.is_playing && b.has_picked)
        .map(|(_, b, _, _)| b.selected_burro)
        .collect::<Vec<_>>();

    for (mut transform, burro, _, material) in burros.iter_mut() {
        if !burro.has_picked {
            transform.rotate(Quat::from_rotation_z(time.delta_seconds()));
        } else {
            transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0);
        }

        if let Some(material) = assets_handler.materials.get_mut(material) {
            if picked_skins.contains(&burro.selected_burro) {
                material.base_color = Color::GRAY;
            } else {
                material.base_color = Color::WHITE;
            }
        }
    }

    local_cooldown.cooldown -= time.delta_seconds();
    local_cooldown.cooldown = local_cooldown.cooldown.clamp(-10.0, 3.0);

    if local_cooldown.cooldown > 0.0 {
        return;
    }

    let left_burro = HashMap::from([
        (BurroSkin::Pinata, BurroSkin::Meow),
        (BurroSkin::Meow, BurroSkin::Salud),
        (BurroSkin::Salud, BurroSkin::Mexico),
        (BurroSkin::Mexico, BurroSkin::Medianoche),
        (BurroSkin::Medianoche, BurroSkin::Morir),
        (BurroSkin::Morir, BurroSkin::Gators),
        (BurroSkin::Gators, BurroSkin::Aguas),
        (BurroSkin::Aguas, BurroSkin::Pinata),
    ]);

    let right_burro = HashMap::from([
        (BurroSkin::Meow, BurroSkin::Pinata),
        (BurroSkin::Salud, BurroSkin::Meow),
        (BurroSkin::Mexico, BurroSkin::Salud),
        (BurroSkin::Medianoche, BurroSkin::Mexico),
        (BurroSkin::Morir, BurroSkin::Medianoche),
        (BurroSkin::Gators, BurroSkin::Morir),
        (BurroSkin::Aguas, BurroSkin::Gators),
        (BurroSkin::Pinata, BurroSkin::Aguas),
    ]);

    let mut attempt_to_start_game = false;
    let mut player_hasnt_picked = false;

    for (_, mut burro, action_state, _) in burros.iter_mut() {
        burro.action_cooldown -= time.delta_seconds();
        burro.action_cooldown = burro.action_cooldown.clamp(-10.0, 3.0);

        if burro.action_cooldown > 0.0 || burro.has_picked {
            continue;
        }

        if action_state.just_pressed(MenuAction::Left) {
            burro.selected_burro = left_burro[&burro.selected_burro];
            burro.action_cooldown = 0.2;

            audio.play_sfx(&game_assets.sfx_1);
        }
        if action_state.just_pressed(MenuAction::Right) {
            burro.selected_burro = right_burro[&burro.selected_burro];
            burro.action_cooldown = 0.2;

            audio.play_sfx(&game_assets.sfx_1);
        }
        if action_state.just_pressed(MenuAction::Select) {
            if !burro.is_playing {
                burro.is_playing = true;
                audio.play_sfx(&game_assets.sfx_2);
            } else if !picked_skins.contains(&burro.selected_burro) {
                audio.play_sfx(&game_assets.sfx_2);
                burro.action_cooldown = 0.2;
                burro.has_picked = true;

                attempt_to_start_game = true;
            }
        }

        if burro.is_playing && !burro.has_picked {
            player_hasnt_picked = true;
        }
    }

    if attempt_to_start_game && !player_hasnt_picked {
        let players = burros
            .iter()
            .filter(|(_, b, _, _)| b.is_playing)
            .map(|(_, b, _, _)| *b)
            .collect();

        audio.play_sfx(&game_assets.fanfare_sfx);
        *option_state = options::OptionState::initialize(players);
        assets_handler.load(AppState::Options, &mut game_assets, &game_state);
    }
}

fn update_burro_skins(
    game_assets: Res<GameAssets>,
    mut burros: Query<(
        &mut Handle<StandardMaterial>,
        &BurroCharacter,
        &mut Visibility,
    )>,
) {
    for (mut handle, burro, mut visibility) in burros.iter_mut() {
        let skin = match burro.selected_burro {
            BurroSkin::Pinata => &game_assets.pinata_texture.material,
            BurroSkin::Meow => &game_assets.meow_texture.material,
            BurroSkin::Salud => &game_assets.salud_texture.material,
            BurroSkin::Mexico => &game_assets.mexico_texture.material,
            BurroSkin::Medianoche => &game_assets.medianoche_texture.material,
            BurroSkin::Morir => &game_assets.morir_texture.material,
            BurroSkin::Gators => &game_assets.gators_texture.material,
            BurroSkin::Aguas => &game_assets.aguas_texture.material,
        };
        *handle = skin.clone();

        visibility.is_visible = burro.is_playing;
    }
}

fn handle_labels(
    mut burro_names: Query<(&BurroName, &mut Text)>,
    players: Query<(Entity, &BurroCharacter)>,
) {
    for (name, mut text) in burro_names.iter_mut() {
        for (_, player) in players.iter() {
            if player.player == name.player {
                if player.is_playing {
                    text.sections[0].value = match player.selected_burro {
                        game_state::BurroSkin::Pinata => "Pinata",
                        game_state::BurroSkin::Meow => "Meow",
                        game_state::BurroSkin::Salud => "Salud",
                        game_state::BurroSkin::Mexico => "Mexico",
                        game_state::BurroSkin::Medianoche => "Medianoche",
                        game_state::BurroSkin::Morir => "Morir",
                        game_state::BurroSkin::Gators => "Gators",
                        game_state::BurroSkin::Aguas => "Aguas",
                    }
                    .to_string();
                } else {
                    text.sections[0].value = format!("P{} Press Start", player.player + 1);
                }
            }
        }
    }
}

fn handle_controllers(
    controllers: Res<game_controller::GameController>,
    mut players: Query<(Entity, &BurroCharacter, &mut ActionState<MenuAction>)>,
) {
    for (_, player, mut action_state) in players.iter_mut() {
        if let Some(pressed) = controllers.pressed.get(&player.player) {
            // release all buttons
            // this probably affects durations but for
            // this game it might not be a big deal
            action_state.release(MenuAction::Left);
            action_state.release(MenuAction::Right);

            action_state.release(MenuAction::Select);

            if pressed.contains(&game_controller::GameButton::Left) {
                action_state.press(MenuAction::Left);
            }
            if pressed.contains(&game_controller::GameButton::Right) {
                action_state.press(MenuAction::Right);
            }
            if pressed.contains(&game_controller::GameButton::ActionDown)
                || pressed.contains(&game_controller::GameButton::Start)
            {
                action_state.press(MenuAction::Select);
            }
        }
    }
}
