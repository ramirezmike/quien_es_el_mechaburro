use crate::{asset_loading, assets::GameAssets, burro, cleanup, game_state, AppState};
use bevy::prelude::*;

pub struct ScoreDisplayPlugin;
impl Plugin for ScoreDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::ScoreDisplay).with_system(setup),
        )

            .add_system_set(
            SystemSet::on_exit(AppState::ScoreDisplay).with_system(cleanup::<CleanupMarker>),
        );
    }
}

enum ScoreState {
    Initial,
    Adding,
    Displaying,
}

#[derive(Component)]
struct CleanupMarker;

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    let padding = Val::Px(20.0);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(90.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
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
                            use game_state::BurroSkin;
                            game_state.burros.iter().for_each(|burro| {
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
                            let padding = 40.0;

                            game_state.burros.iter_mut().for_each(|mut burro| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(20.0), Val::Auto),
                                            margin: Rect {
                                                top: Val::Px(100.0),
                                                left: Val::Px(padding),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        text: Text::with_section(
                                            format!("{}", burro.score).to_string(),
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
