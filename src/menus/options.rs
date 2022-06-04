use crate::{
    asset_loading, assets::GameAssets, audio::GameAudio, character_select::BurroCharacter, cleanup,
    game_controller, game_state, menus, title_screen::MenuAction, AppState,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Options)
                .with_system(setup)
                .with_system(game_controller::clear_presses),
        )
        .insert_resource(CurrentOption(0))
        .insert_resource(OptionState::default())
        .add_event::<OptionChangeEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::Options)
                .with_system(update_menu_buttons.after("handle_input"))
                .with_system(highlight_options)
                .with_system(handle_option_changes)
                .with_system(display_current_options)
                .with_system(
                    handle_controllers
                        .label("handle_input")
                        .after("store_controller_inputs"),
                ),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Options)
                .with_system(cleanup::<CleanupMarker>)
                .with_system(game_controller::clear_presses),
        );
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Component)]
struct OptionRow {
    row: usize,
}

struct CurrentOption(usize);

enum OptionChange {
    Increase,
    Decrease,
    Select,
}

#[derive(Default)]
pub struct OptionState {
    players: Vec<BurroCharacter>,
    number_of_players: usize,
    number_of_bots: usize,
}

impl OptionState {
    pub fn initialize(players: Vec<BurroCharacter>) -> Self {
        OptionState {
            number_of_players: players.len(),
            number_of_bots: 8 - players.len(),
            players,
        }
    }
}

#[derive(Component)]
struct OptionValueMarker;

struct OptionChangeEvent {
    action: OptionChange,
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut current_option: ResMut<CurrentOption>,
) {
    current_option.0 = 0;

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    commands
        .spawn_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(CleanupMarker);

    // Options Title text
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
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        margin: Rect {
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..Default::default()
                            },
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Game Settings".to_string(),
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
                    });
                });

            // # of bots option
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 0 })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: Rect {
                                    left: Val::Percent(2.0),
                                    ..Default::default()
                                },
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Number of Bots:".to_string(),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: 35.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(OptionRow { row: 0 });

                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: Rect {
                                    left: Val::Percent(2.0),
                                    ..Default::default()
                                },
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "<".to_string(),
                                TextStyle {
                                    font: game_assets.score_font.clone(),
                                    font_size: 35.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(OptionRow { row: 0 });

                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: Rect {
                                    left: Val::Percent(2.0),
                                    ..Default::default()
                                },
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "".to_string(),
                                TextStyle {
                                    font: game_assets.score_font.clone(),
                                    font_size: 35.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(OptionValueMarker)
                        .insert(OptionRow { row: 0 });

                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: Rect {
                                    left: Val::Percent(2.0),
                                    ..Default::default()
                                },
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexStart,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                ">".to_string(),
                                TextStyle {
                                    font: game_assets.score_font.clone(),
                                    font_size: 35.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(OptionRow { row: 0 });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        margin: Rect {
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 1 })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..Default::default()
                                },
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "¡Vamos!".to_string(),
                                TextStyle {
                                    font: game_assets.score_font.clone(),
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
                        .insert(OptionRow { row: 1 });
                });
        });
}

fn highlight_options(
    current_option: Res<CurrentOption>,
    mut options: Query<(&OptionRow, Option<&mut UiColor>, Option<&mut Text>)>,
) {
    for (option, maybe_ui_color, maybe_text) in options.iter_mut() {
        if option.row == current_option.0 {
            if let Some(mut ui_color) = maybe_ui_color {
                *ui_color = UiColor(menus::HOVERED_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for mut text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::BLACK;
                }
            }
        } else {
            if let Some(mut ui_color) = maybe_ui_color {
                *ui_color = UiColor(menus::NORMAL_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::WHITE;
                }
            }
        }
    }
}

fn update_menu_buttons(
    mut current_option: ResMut<CurrentOption>,
    action_state: Query<&ActionState<MenuAction>>,
    game_assets: Res<GameAssets>,
    mut audio: GameAudio,
    mut option_change_event_writer: EventWriter<OptionChangeEvent>,
) {
    let action_state = action_state.single();

    if action_state.just_pressed(MenuAction::Up) {
        audio.play_sfx(&game_assets.sfx_1);
        current_option.0 = current_option.0.saturating_sub(1);
    }
    if action_state.just_pressed(MenuAction::Down) {
        audio.play_sfx(&game_assets.sfx_1);
        current_option.0 = usize::min(current_option.0 + 1, 1);
    }
    if action_state.just_pressed(MenuAction::Right) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Increase,
        });
    }
    if action_state.just_pressed(MenuAction::Left) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Decrease,
        });
    }
    if action_state.just_pressed(MenuAction::Select) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Select,
        });
    }
}

fn handle_controllers(
    controllers: Res<game_controller::GameController>,
    mut players: Query<(Entity, &mut ActionState<MenuAction>)>,
) {
    for (_, mut action_state) in players.iter_mut() {
        for (_, just_pressed) in controllers.just_pressed.iter() {
            action_state.release(MenuAction::Up);
            action_state.release(MenuAction::Down);

            action_state.release(MenuAction::Select);

            if just_pressed.contains(&game_controller::GameButton::Up) {
                action_state.press(MenuAction::Up);
            }
            if just_pressed.contains(&game_controller::GameButton::Down) {
                action_state.press(MenuAction::Down);
            }
            if just_pressed.contains(&game_controller::GameButton::ActionDown)
                || just_pressed.contains(&game_controller::GameButton::Start)
            {
                action_state.press(MenuAction::Select);
            }
        }
    }
}

fn handle_option_changes(
    current_option: Res<CurrentOption>,
    mut option_change_event_reader: EventReader<OptionChangeEvent>,
    mut options: ResMut<OptionState>,
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut assets_handler: asset_loading::AssetsHandler,
) {
    for option_change in option_change_event_reader.iter() {
        match current_option.0 {
            0 => match option_change.action {
                OptionChange::Increase => {
                    options.number_of_bots =
                        usize::min(options.number_of_bots + 1, 8 - options.number_of_players);
                }
                OptionChange::Decrease => {
                    let minimum_number_of_bots = if options.number_of_players == 1 { 1 } else { 0 };
                    options.number_of_bots =
                        usize::max(options.number_of_bots - 1, minimum_number_of_bots);
                }
                _ => (),
            },
            1 => {
                if let OptionChange::Select = option_change.action {
                    *game_state = game_state::GameState::initialize(
                        options.players.clone(),
                        options.number_of_bots,
                    );
                    assets_handler.load(AppState::Debug, &mut game_assets, &game_state);
                }
            }
            _ => (),
        }
    }
}

fn display_current_options(
    option_state: ResMut<OptionState>,
    mut options: Query<(&mut Text, &OptionRow), With<OptionValueMarker>>,
) {
    for (mut option_text, option_row) in options.iter_mut() {
        if option_row.row == 0 {
            option_text.sections[0].value = option_state.number_of_bots.to_string();
        }
    }
}
