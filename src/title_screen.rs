use crate::{
    asset_loading, assets::GameAssets, audio::GameAudio, cleanup, game_controller, menus, mesh,
    ui::text_size, AppState,
};
use bevy::app::AppExit;
use bevy::ecs::event::Events;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MenuAction>::default())
            .add_system_set(SystemSet::on_enter(AppState::TitleScreen).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::TitleScreen)
                    .with_system(update_menu_buttons.after("handle_input"))
                    .with_system(
                        handle_controllers
                            .label("handle_input")
                            .after("store_controller_inputs"),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::TitleScreen).with_system(cleanup::<CleanupMarker>),
            );
    }
}

#[derive(Component)]
pub struct CleanupMarker;

const NORMAL_BUTTON: Color = Color::rgba(1.00, 1.00, 1.00, 0.0);
const HOVERED_BUTTON: Color = Color::rgb(1.00, 1.00, 0.75);

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Select,
}
impl MenuAction {
    pub fn default_input_map() -> InputMap<MenuAction> {
        use MenuAction::*;
        let mut input_map = InputMap::default();

        input_map.insert(Up, KeyCode::Up);
        input_map.insert(Up, KeyCode::W);

        input_map.insert(Down, KeyCode::Down);
        input_map.insert(Down, KeyCode::S);

        input_map.insert(Left, KeyCode::Left);
        input_map.insert(Left, KeyCode::A);

        input_map.insert(Right, KeyCode::Right);
        input_map.insert(Right, KeyCode::D);

        input_map.insert(Select, KeyCode::Space);
        input_map.insert(Select, KeyCode::Return);

        input_map
    }
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_audio(&mut game_assets.bgm_1, "audio/baila.ogg");
    assets_handler.add_audio(&mut game_assets.sfx_1, "audio/blip.wav");
    assets_handler.add_audio(&mut game_assets.sfx_2, "audio/select.wav");
    assets_handler.add_font(&mut game_assets.font, "fonts/MexicanTequila.ttf");
    assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
    assets_handler.add_material(
        &mut game_assets.title_screen_background,
        "textures/background.png",
        false,
    );
    assets_handler.add_material(
        &mut game_assets.title_screen_logo,
        "textures/logo.png",
        true,
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
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut audio: GameAudio,
    text_scaler: text_size::TextScaler,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);
    commands
        .spawn_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(mesh::MeshBuilder::plane_repeating(
            &mut meshes,
            &mut images,
            &game_assets.title_screen_background,
            20.0,
            None,
            None,
        ))
        .insert(CleanupMarker)
        .insert_bundle(mesh::MeshBuilder::add_scrolling_bundle(-Vec3::Z));

    commands
        .spawn_bundle(mesh::MeshBuilder::plane(
            &mut meshes,
            &game_assets.title_screen_logo,
            3.0,
            1.0,
        ))
        .insert(CleanupMarker);

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
        .insert(CleanupMarker);

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
                ..Default::default()
            },
            text: Text::with_section(
                "by michael ramirez".to_string(),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: text_scaler.scale(menus::BY_LINE_FONT_SIZE),
                    color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(25.0)),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Percent(60.0),
                    ..Default::default()
                },
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: Rect::all(Val::Auto),
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(CleanupMarker);

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit",
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(CleanupMarker);
        });

    audio.play_bgm(&game_assets.bgm_1);
}

fn update_menu_buttons(
    mut selected_button: Local<usize>,
    mut exit: ResMut<Events<AppExit>>,
    buttons: Query<Entity, With<Button>>,
    mut button_colors: Query<&mut UiColor, With<Button>>,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    action_state: Query<&ActionState<MenuAction>>,
    //mut assets_handler: asset_loading::AssetsHandler,
    game_assets: Res<GameAssets>,
    mut audio: GameAudio,
    mut app_state: ResMut<State<AppState>>,
) {
    let action_state = action_state.single();
    let number_of_buttons = buttons.iter().count();
    let mut pressed_button = action_state.pressed(MenuAction::Select);

    if action_state.just_pressed(MenuAction::Up) {
        audio.play_sfx(&game_assets.sfx_1);
        *selected_button = selected_button
            .checked_sub(1)
            .unwrap_or(number_of_buttons - 1);
    }
    if action_state.just_pressed(MenuAction::Down) {
        audio.play_sfx(&game_assets.sfx_1);
        let new_selected_button = selected_button.checked_add(1).unwrap_or(0);
        *selected_button = if new_selected_button > number_of_buttons - 1 {
            0
        } else {
            new_selected_button
        };
    }

    // mouse
    for (button_entity, interaction) in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => pressed_button = true,
            Interaction::Hovered => {
                *selected_button = buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| *x == button_entity)
                    .map(|(i, _)| i)
                    .last()
                    .unwrap_or(*selected_button)
            }
            _ => (),
        }
    }

    for (i, mut color) in button_colors.iter_mut().enumerate() {
        if i == *selected_button {
            *color = HOVERED_BUTTON.into();
        } else {
            *color = NORMAL_BUTTON.into();
        }
    }

    if pressed_button {
        if *selected_button == 0 {
            audio.play_sfx(&game_assets.sfx_2);
            app_state.set(AppState::CharacterSelect).unwrap();
        }
        if *selected_button == 1 {
            exit.send(AppExit);
        }
    }
}

fn handle_controllers(
    controllers: Res<game_controller::GameController>,
    mut players: Query<(Entity, &mut ActionState<MenuAction>)>,
) {
    let just_pressed = &controllers.just_pressed;
    for (_, mut action_state) in players.iter_mut() {
        if just_pressed.contains(&game_controller::GameButton::Up) {
            action_state.release(MenuAction::Up);
            action_state.press(MenuAction::Up);
        }
        if just_pressed.contains(&game_controller::GameButton::Down) {
            action_state.release(MenuAction::Down);
            action_state.press(MenuAction::Down);
        }
        if just_pressed.contains(&game_controller::GameButton::ActionDown)
            || just_pressed.contains(&game_controller::GameButton::Start)
        {
            action_state.release(MenuAction::Select);
            action_state.press(MenuAction::Select);
        }
    }
}
