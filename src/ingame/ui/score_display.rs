use crate::input::InputCommandsExt;
use crate::loading::command_ext::*;
use crate::util::num_ext::*;
use crate::{assets, cleanup, config, game_state, input, ui, AppState, IngameState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::{collections::HashMap, fmt, time::Duration};

pub struct ScoreDisplayPlugin;
impl Plugin for ScoreDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(IngameState::ScoreDisplay), setup)
            .add_systems(OnExit(IngameState::ScoreDisplay), cleanup::<CleanupMarker>)
            .add_systems(
                Update,
                (
                    handle_score_add_event,
                    move_items_to_slots,
                    update_script,
                    update_score_displays,
                )
                    .run_if(in_state(IngameState::ScoreDisplay)),
            )
            .init_resource::<Positioner>()
            .add_event::<ScoreAddEvent>()
            .init_resource::<ScoreDisplayState>();
    }
}

const BORDER_COLOR: Color = Color::WHITE;
const SCORE_BACKGROUND_COLOR: Color = Color::rgba(0., 0., 0., 0.5);
const DEBUG_COLOR: Color = Color::rgba(0., 1., 0., 0.0);
const BACKGROUND_COLOR: Color = Color::NONE;
const OFFSCREEN_LEFT_PERCENTAGE: f32 = 120.;

#[derive(Component, Clone)]
struct CleanupMarker;

#[derive(Component)]
struct PointsDisplayMarker;
#[derive(Component)]
struct Ranking(usize);

#[derive(Event)]
pub struct ScoreAddEvent;

impl fmt::Display for Ranking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self.0 {
            0 => "1st",
            1 => "2nd",
            2 => "3rd",
            3 => "4th",
            4 => "5th",
            5 => "6th",
            6 => "7th",
            _ => "8th",
        };
        write!(f, "{}", text)
    }
}

#[derive(Resource, Default)]
struct ScoreDisplayState {
    script_step: Script,
    current_timer: Timer,
}

#[derive(Default, Debug)]
enum Script {
    #[default]
    Initial,
    MoveToLevelResults,
    AddScore,
    MoveOffScreen,
    MoveToCurrentRanking,
    Wait,
}

#[derive(Resource, Default)]
struct Positioner {
    number_of_elements: usize,
    percent_width_of_element: f32,
}

impl Positioner {
    fn get_left_for_position(&self, position: isize) -> f32 {
        let position = position as f32;
        let number_of_elements = self.number_of_elements as f32;
        let buffer = (100. - (number_of_elements * self.percent_width_of_element))
            / (number_of_elements + 1.);

        (buffer * (position + 1.)) + (self.percent_width_of_element * position)
    }
}

#[derive(Component, Default)]
struct BurroImage {
    // TODO: Rename this
    current_left: f32,
    movement_time: f32,
    target_slot: isize,
}

fn move_items_to_slots(
    mut images: Query<(&mut Style, &mut BurroImage)>,
    positioner: Res<Positioner>,
    time: Res<Time>,
) {
    let travel_time = 1.0;
    for (mut style, mut image) in &mut images {
        let target = positioner.get_left_for_position(image.target_slot);
        if image.movement_time < travel_time {
            image.movement_time += time.delta_seconds();
            let interpolation_time = image.movement_time / travel_time;
            style.left = Val::Percent(image.current_left.lerp(target, interpolation_time.min(1.)));
        } else {
            style.left = Val::Percent(target);
            image.current_left = target;
        }
    }
}

fn update_script(
    mut commands: Commands,
    mut score_display_state: ResMut<ScoreDisplayState>,
    mut score_add_event_writer: EventWriter<ScoreAddEvent>,
    mut images: Query<(&mut Style, &mut BurroImage, &game_state::PlayerMarker)>,
    mut game_state: ResMut<game_state::GameState>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    positioner: Res<Positioner>,
    action_state: Query<&ActionState<input::MenuAction>>,
    time: Res<Time>,
) {
    let action_state = action_state.single();
    let continue_pressed = action_state.just_pressed(input::MenuAction::Start);

    // TODO: set something up here so that player 1 has to press start instead?
    if continue_pressed {
        score_display_state
            .current_timer
            .set_elapsed(Duration::from_secs(999));
        for (mut style, mut image, _) in &mut images {
            let target = positioner.get_left_for_position(image.target_slot);
            style.left = Val::Percent(target);
            image.current_left = target;
            image.movement_time = 999.0;
        }
    }

    if !score_display_state
        .current_timer
        .tick(time.delta())
        .finished()
    {
        return;
    }

    match score_display_state.script_step {
        Script::Initial => {
            score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);
            score_display_state.script_step = Script::MoveToLevelResults;
        }
        Script::MoveToLevelResults => {
            score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);
            score_display_state.script_step = Script::AddScore;
        }
        Script::AddScore => {
            score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);
            score_display_state.script_step = Script::MoveOffScreen;
            score_add_event_writer.send(ScoreAddEvent);
        }
        Script::MoveOffScreen => {
            for (_, mut image, _) in &mut images {
                image.movement_time = 0.0;
                image.target_slot = -8;
            }
            score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);
            score_display_state.script_step = Script::MoveToCurrentRanking;
        }
        Script::MoveToCurrentRanking => {
            let mut burros = game_state.burros.clone();
            burros.sort_by_key(|x| x.score);
            burros.reverse();
            let ranking: HashMap<usize, usize> = burros
                .iter()
                .enumerate()
                .map(|(i, x)| (x.selected_burro, i))
                .collect();

            for (mut style, mut image, player) in &mut images {
                style.left = Val::Percent(OFFSCREEN_LEFT_PERCENTAGE);
                image.current_left = OFFSCREEN_LEFT_PERCENTAGE;
                image.movement_time = 0.0 - (0.1 * ranking[&player.0] as f32);
                image.target_slot = ranking[&player.0] as isize;
            }
            score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);
            score_display_state.script_step = Script::Wait;
        }
        Script::Wait => {
            if continue_pressed {
                game_state.current_level += 1;

                next_ingame_state.set(IngameState::Disabled);
                if game_state.current_level >= config::NUMBER_OF_LEVELS {
                    #[cfg(feature = "debug")]
                    {
                        game_state.current_level = 0;
                    }
                    commands.load_state(AppState::Splash);
                } else {
                    commands.load_state(AppState::LoadInGame);
                }
            }
        }
    }
}

fn handle_score_add_event(
    mut score_add_event_reader: EventReader<ScoreAddEvent>,
    mut game_state: ResMut<game_state::GameState>,
) {
    if score_add_event_reader.iter().count() > 0 {
        // adds a point based on the order the burro died
        // first burro to die gets 1 point, last burro to die gets the most points
        let burro_points: HashMap<usize, usize> = game_state
            .dead_burros
            .iter()
            .rev()
            .enumerate()
            .map(|(i, b)| (*b, i + 1))
            .collect();
        let max_score = game_state.dead_burros.len() + 1;

        for burro in game_state.burros.iter_mut() {
            let new_score = burro_points
                .get(&burro.selected_burro)
                .unwrap_or(&max_score);
            burro.score += new_score;
        }
    }
}

fn update_score_displays(
    game_state: Res<game_state::GameState>,
    mut score_displays: Query<(&mut Text, &game_state::PlayerMarker), With<PointsDisplayMarker>>,
) {
    for (mut text, player_marker) in &mut score_displays {
        for burro in game_state.burros.iter() {
            if burro.selected_burro == player_marker.0 {
                text.sections[0].value = format!("{}", burro.score);
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    game_state: Res<game_state::GameState>,
    mut positioner: ResMut<Positioner>,
    mut images: ResMut<Assets<Image>>,
    mut score_display_state: ResMut<ScoreDisplayState>,
    window_size: Res<ui::text_size::WindowSize>,
    text_scaler: ui::text_size::TextScaler,
) {
    *score_display_state = ScoreDisplayState::default();
    commands.spawn_menu_input(CleanupMarker);
    //    score_display_state.current_timer = Timer::from_seconds(1.0, TimerMode::Once);

    let mut burro_image_handles: HashMap<usize, Handle<Image>> =
        HashMap::<usize, Handle<Image>>::default();
    for (i, burro) in game_state.burros.iter().enumerate() {
        let image = ui::render_to_texture::create_render_image(&window_size);
        let image_handle = images.add(image);
        burro_image_handles.insert(burro.selected_burro, image_handle.clone());
        let y_base = -100.0;
        let y_offset = 10.0;

        commands.add(ui::render_to_texture::BurroImage {
            player: burro.player,
            selected_burro: burro.selected_burro,
            burro_transform: Transform::from_xyz(0.0, y_base + i as f32 * y_offset, 0.0),
            camera_transform: Transform::from_xyz(1.7, y_base + 0.9 + i as f32 * y_offset, 1.9)
                .with_rotation(Quat::from_rotation_y(0.6)),
            outline_color: burro.outline_color,
            outline_size: 30.0,
            render_layer_id: 1,
            cleanup_marker: CleanupMarker,
            clear_color: Color::NONE,
            image_handle: image_handle.clone(),
        });
    }

    let element_width = 10.;
    *positioner = Positioner {
        number_of_elements: game_state.burros.iter().len(),
        percent_width_of_element: element_width,
    };

    let root_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            },
            CleanupMarker,
        ))
        .id();

    let score_container = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(40.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: SCORE_BACKGROUND_COLOR.into(),
            ..default()
        },))
        .id();

    for (i, burro) in game_state.burros.iter().enumerate() {
        let position = game_state
            .dead_burros
            .iter()
            .position(|x| *x == burro.selected_burro)
            .map(|x| x as isize + 1)
            .unwrap_or(0);

        let image = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(element_width),
                        height: Val::Percent(70.0),
                        border: UiRect::all(Val::Percent(0.5)),
                        left: Val::Percent(OFFSCREEN_LEFT_PERCENTAGE),
                        ..default()
                    },
                    border_color: BORDER_COLOR.into(),
                    background_color: DEBUG_COLOR.into(),
                    ..default()
                },
                game_state::PlayerMarker(burro.selected_burro),
                BurroImage {
                    target_slot: position,
                    current_left: OFFSCREEN_LEFT_PERCENTAGE,
                    movement_time: 0.0 - (0.1 * i as f32),
                },
            ))
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder
                            .spawn(NodeBundle {
                                style: Style {
                                    position_type: PositionType::Relative,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(60.0),
                                    border: UiRect {
                                        bottom: Val::Percent(2.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                border_color: BORDER_COLOR.into(),
                                background_color: Color::rgba(0.2, 0.2, 0.2, 0.5).into(),
                                ..default()
                            })
                            .with_children(|builder| {
                                builder.spawn((
                                    ImageBundle {
                                        style: Style {
                                            //                                      position_type: PositionType::Relative,
                                            width: Val::Percent(100.0),
                                            //                                      height: Val::Percent(60.0),
                                            ..default()
                                        },
                                        image: burro_image_handles[&burro.selected_burro]
                                            .clone()
                                            .into(),
                                        z_index: ZIndex::Global(4),
                                        ..default()
                                    },
                                    game_state::PlayerMarker(burro.selected_burro),
                                ));
                            });

                        builder
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(10.),
                                    padding: UiRect::all(Val::Percent(5.)),
                                    position_type: PositionType::Relative,
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::FlexStart,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|builder| {
                                let text = if burro.is_bot {
                                    "BOT".to_string()
                                } else {
                                    format!("P{}", burro.player + 1)
                                };
                                builder.spawn(TextBundle {
                                    text: Text::from_section(
                                        text,
                                        TextStyle {
                                            font: game_assets.score_font.clone(),
                                            font_size: text_scaler
                                                .scale(ui::DEFAULT_FONT_SIZE * 0.5),
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });

                                let text_style = TextStyle {
                                    font: game_assets.score_font.clone(),
                                    font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 0.5),
                                    color: Color::WHITE,
                                };
                                builder.spawn((
                                    TextBundle {
                                        text: Text::from_sections([
                                            TextSection::new("0", text_style.clone()),
                                            TextSection::new(" pts", text_style.clone()),
                                        ]),
                                        ..default()
                                    },
                                    game_state::PlayerMarker(burro.selected_burro),
                                    PointsDisplayMarker,
                                ));

                                let ranking = Ranking(position as usize);
                                builder.spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            format!("{}", ranking),
                                            text_style.clone(),
                                        ),
                                        ..default()
                                    },
                                    game_state::PlayerMarker(burro.selected_burro),
                                    ranking,
                                ));
                            });
                    });
            })
            .id();

        commands.entity(score_container).add_child(image);
    }

    commands.entity(root_node).add_child(score_container);
}
