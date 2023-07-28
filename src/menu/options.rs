use crate::{AppState, assets, ui, game_camera, game_state, asset_loading, input, input::InputCommandsExt, audio, cleanup, };
use crate::loading::command_ext::*;
use crate::util::num_ext::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Options), setup)
           .init_resource::<OptionState>()
           .add_systems(Update, (highlight_selection, handle_input, update_values).run_if(in_state(AppState::Options)))
           .add_systems(OnExit(AppState::Options), cleanup::<CleanupMarker>);
    }
}

#[derive(Default, Resource)]
pub struct OptionState {
    selected_option: Options,
    number_of_players: isize,
    number_of_bots: isize,
    unfair_advantage: isize,
}

impl OptionState {
    fn display(&self, option: &Options) -> String {
        match option {
            Options::NumberOfPlayers => format!("{}", self.number_of_players + 1),
            Options::NumberOfBots => format!("{}", self.number_of_bots + 1),
            Options::UnfairAdvantage => {
                match self.unfair_advantage {
                    0 => "Mechaburrito".to_string(),
                    1 => " Mechaburro ".to_string(),
                    _ => "Mechagigante".to_string(),
                }
            },
            option => option.get_label().to_string()
        }
    }

    fn increment(&mut self) {
        match self.selected_option {
            Options::NumberOfPlayers => {
                self.number_of_players = self.number_of_players.add_with_wrap(1, 8);
            },
            Options::NumberOfBots => {
                self.number_of_bots = self.number_of_bots.add_with_wrap(1, 8);
            },
            Options::UnfairAdvantage => {
                self.unfair_advantage = self.unfair_advantage.add_with_wrap(1, 3);
            },
            _ => ()
        }
    }

    fn decrement(&mut self) {
        match self.selected_option {
            Options::NumberOfPlayers => {
                self.number_of_players = self.number_of_players.sub_with_wrap(1, 8);
            },
            Options::NumberOfBots => {
                self.number_of_bots = self.number_of_bots.sub_with_wrap(1, 8);
            },
            Options::UnfairAdvantage => {
                self.unfair_advantage = self.unfair_advantage.sub_with_wrap(1, 3);
            },
            _ => ()
        }
    }
}

#[derive(Component)]
struct OptionDisplayMarker;

#[derive(Component, Copy, Clone, PartialEq, Default)]
enum Options {
    NumberOfPlayers,
    #[default]
    NumberOfBots,
    UnfairAdvantage,
    Vamos,
}

const _:() = {
    const OPTIONS: [Options; 4] = 
            [Options::NumberOfPlayers,
            Options::NumberOfBots,
            Options::UnfairAdvantage,
            Options::Vamos,];

    impl Options {
        fn get() -> impl IntoIterator::<Item=Options> + Clone {
            OPTIONS
        }

        fn next(&self) -> Self {
            let position = OPTIONS.iter().position(|x| x == self).unwrap();
            *OPTIONS.iter().cycle().nth(position + 1).unwrap()
        }

        fn previous(&self) -> Self {
            let position = OPTIONS.iter().rev().position(|x| x == self).unwrap();
            *OPTIONS.iter().rev().cycle().nth(position + 1).unwrap()
        }

        fn get_label(&self) -> &str {
            match self {
                Options::NumberOfPlayers => "Number of Players",
                Options::NumberOfBots => "Number of Bots",
                Options::UnfairAdvantage => "Unfair Advantage",
                Options::Vamos => "¡Vamos!",
            }
        }
    }
};

#[derive(Component)]
struct CleanupMarker;

use bevy::ecs::system::{Command, SystemState};
pub struct OptionsMenutLoader;
impl Command for OptionsMenutLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
             asset_loading::AssetsHandler,
             ResMut<assets::GameAssets>)> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_font(&mut game_assets.font, "fonts/MexicanTequila.ttf");
        assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
    }
}

fn highlight_selection(
    options_state: Res<OptionState>,
    mut options: Query<(&Options, Option<&mut BackgroundColor>, Option<&mut Text>)>,
) {
    for (&option, maybe_background_color, maybe_text) in &mut options {
        if option == options_state.selected_option {
            if let Some(mut background_color) = maybe_background_color {
                *background_color = BackgroundColor(ui::HOVERED_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::BLACK;
                }
            }
        } else {
            if let Some(mut background_color) = maybe_background_color {
                *background_color = BackgroundColor(ui::NORMAL_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::WHITE;
                }
            }
        }
    }
}

fn update_values(
    option_state: ResMut<OptionState>,
    mut options: Query<(&mut Text, &Options), With<OptionDisplayMarker>>,
) {
    for (mut text, option) in &mut options {
        text.sections[0].value = option_state.display(&option).to_string();
    }
}

fn handle_input(
    mut commands: Commands,
    mut option_state: ResMut<OptionState>,
    action_state: Query<&ActionState<input::MenuAction>>,
    game_assets: Res<assets::GameAssets>,
    mut audio: audio::GameAudio,
) {
    let action_state = action_state.single();

    if action_state.just_pressed(input::MenuAction::Up) {
        audio.play_sfx(&game_assets.sfx_1);
        option_state.selected_option = option_state.selected_option.previous();
    }

    if action_state.just_pressed(input::MenuAction::Down) {
        audio.play_sfx(&game_assets.sfx_1);
        option_state.selected_option = option_state.selected_option.next();
    }

    if action_state.just_pressed(input::MenuAction::Left) {
        audio.play_sfx(&game_assets.sfx_1);
        option_state.decrement();
    }

    if action_state.just_pressed(input::MenuAction::Right) {
        audio.play_sfx(&game_assets.sfx_1);
        option_state.increment();
    }

    if action_state.just_pressed(input::MenuAction::Select) {
        if option_state.selected_option == Options::Vamos {
            audio.play_sfx(&game_assets.sfx_1);
            commands.load_state(AppState::LoadInGame);
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    text_scaler: ui::text_size::TextScaler,
) {
    game_camera::spawn_camera(&mut commands, CleanupMarker);
    commands.spawn_menu_input(CleanupMarker);

    let root_node = commands.spawn((NodeBundle {
        z_index: ZIndex::Global(-100),
        style: Style { 
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    }, CleanupMarker)).id();

    let title_text = commands.spawn(NodeBundle {
        style: Style { 
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        builder.spawn(TextBundle {
            text: Text::from_section(
                "Game Settings",
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

    let options =
    Options::get()
    .into_iter().map(|option| {
        match option {
            Options::Vamos => {
                commands.spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(20.),
                        height: Val::Percent(15.),
                        display: Display::Flex,
                        padding: UiRect::all(Val::Percent(2.)),
                        margin: UiRect {
                            top: Val::Percent(15.),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Percent(1.0)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                }, option.clone()))
                .with_children(|builder| {
                    builder.spawn((TextBundle {
                        text: Text::from_section(
                            format!("{}", option.get_label()),
                            TextStyle {
                                font: game_assets.score_font.clone(),
                                font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    }, option.clone()));
                })
                .id()
            },
            _ => {
                commands.spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(15.),
                        display: Display::Flex,
                        padding: UiRect::all(Val::Percent(2.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                }, option.clone()))
                .with_children(|builder| {
                    builder.spawn((TextBundle {
                        text: Text::from_section(
                            format!("{}:", option.get_label()),
                            TextStyle {
                                font: game_assets.font.clone(),
                                font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    }, option.clone()));

                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(40.),
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    })
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
                        }, option.clone()));
                        builder.spawn((TextBundle {
                            text: Text::from_section(
                                "5",
                                TextStyle {
                                    font: game_assets.score_font.clone(),
                                    font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        }, option.clone(), OptionDisplayMarker));
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
                        }, option.clone()));
                    });
                }) 
                .id()
            }
        }
    }).collect::<Vec::<_>>();


    commands.entity(root_node).add_child(title_text);

    for entity in options {
        commands.entity(root_node).add_child(entity);
    }
}
