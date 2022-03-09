use crate::{assets::GameAssets, cleanup, game_camera, game_state, player, AppState};
use bevy::prelude::*;

pub struct ScoreDisplayPlugin;
impl Plugin for ScoreDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::ScoreDisplay).with_system(setup))
            .insert_resource(ScoreState::default())
            .add_system_set(
                SystemSet::on_update(AppState::ScoreDisplay)
                    .with_system(display_scores)
                    .with_system(follow_winner),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::ScoreDisplay).with_system(cleanup::<CleanupMarker>),
            );
    }
}

#[derive(PartialEq)]
enum ScoreStates {
    Initial,
    Adding,
    Added,
    Displaying,
}
impl Default for ScoreStates {
    fn default() -> Self {
        ScoreStates::Initial
    }
}

struct ScoreState {
    state: ScoreStates,
    cooldown: f32,
    first_render: bool,
}

impl Default for ScoreState {
    fn default() -> Self {
        ScoreState {
            state: ScoreStates::default(),
            cooldown: 0.0,
            first_render: true,
        }
    }
}

#[derive(Component)]
struct CleanupMarker;

fn setup(mut score_state: ResMut<ScoreState>, mut game_state: ResMut<game_state::GameState>) {
    *score_state = ScoreState::default();

    game_state.current_level_over = true;
}

fn follow_winner(
    player: Query<&Transform, With<player::Player>>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    cameras: Query<&Transform, With<game_camera::PanOrbitCamera>>,
) {
    for p in player.iter() {
        for camera in cameras.iter() {
            camera_settings.set_camera(
                2.0,
                p.translation,
                0.4,
                true,
                p.translation.distance(camera.translation),
                5.0,
            );
        }
    }
}

fn display_scores(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_state: Res<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
    mut score_state: ResMut<ScoreState>,
    cleanups: Query<Entity, With<CleanupMarker>>,
    mut score_add_event_writer: EventWriter<game_state::ScoreAddEvent>,
    time: Res<Time>,
) {
    // this whole thing is a mess, please don't look

    score_state.cooldown -= time.delta_seconds();
    score_state.cooldown = score_state.cooldown.clamp(-10.0, 3.0);

    if !score_state.first_render {
        if score_state.cooldown <= 0.0 {
            if score_state.state == ScoreStates::Initial {
                // throw event to actually add score
                score_add_event_writer.send(game_state::ScoreAddEvent);
            }

            if score_state.state == ScoreStates::Displaying {
                app_state.pop().unwrap();
                return;
            }

            score_state.state = match score_state.state {
                ScoreStates::Initial => ScoreStates::Adding,
                ScoreStates::Adding => ScoreStates::Added,
                ScoreStates::Added => ScoreStates::Displaying,
                _ => ScoreStates::Displaying,
            };
            score_state.cooldown = 3.0;
            for entity in cleanups.iter() {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            return;
        }
    }
    score_state.first_render = false;

    let (show_score, order_by_rank) = match score_state.state {
        ScoreStates::Displaying => (false, true),
        _ => (true, false),
    };

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);
    let padding = Val::Px(20.0);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                        size: Size::new(Val::Percent(100.0), Val::Px(140.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::GRAY.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(90.0)),
                                position_type: PositionType::Absolute,
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
                                    margin: Rect {
                                        top: Val::Px(-60.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    if show_score {
                                        "Current Scores"
                                    } else {
                                        "Ranking"
                                    },
                                    TextStyle {
                                        font: game_assets.font.clone(),
                                        font_size: 80.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });

                    let mut burros = game_state.burros.iter().collect::<Vec<_>>();
                    if order_by_rank {
                        burros.sort_by_key(|b| b.score);
                        burros = burros.into_iter().rev().collect::<Vec<_>>();
                    }

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Px(90.0)),
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            use game_state::BurroSkin;
                            burros.iter().for_each(|burro| {
                                parent.spawn_bundle(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(100.0), Val::Auto),
                                        margin: Rect {
                                            left: padding,
                                            right: padding,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    image: match burro.skin {
                                        BurroSkin::Pinata => {
                                            game_assets.pinata_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Meow => {
                                            game_assets.meow_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Salud => {
                                            game_assets.salud_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Mexico => {
                                            game_assets.mexico_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Medianoche => {
                                            game_assets.medianoche_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Morir => {
                                            game_assets.morir_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Gators => {
                                            game_assets.gators_logo_texture.image.clone().into()
                                        }
                                        BurroSkin::Aguas => {
                                            game_assets.aguas_logo_texture.image.clone().into()
                                        }
                                    },
                                    ..Default::default()
                                });
                            });
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            let padding = 120.0;

                            burros.iter().enumerate().for_each(|(i, burro)| {
                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(20.0), Val::Auto),
                                        margin: Rect {
                                            top: Val::Px(100.0),
                                            left: if i != 0 {
                                                Val::Px(padding)
                                            } else {
                                                Val::Px(0.0)
                                            },
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        if show_score {
                                            format!("{}", burro.score)
                                        } else {
                                            match i {
                                                0 => "1st".to_string(),
                                                1 => "2nd".to_string(),
                                                2 => "3rd".to_string(),
                                                3 => "4th".to_string(),
                                                4 => "5th".to_string(),
                                                5 => "6th".to_string(),
                                                6 => "7th".to_string(),
                                                _ => "8th".to_string(),
                                            }
                                        },
                                        TextStyle {
                                            font: game_assets.font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment::default(),
                                    ),
                                    ..Default::default()
                                });
                            });
                        });
                });
        });
}
