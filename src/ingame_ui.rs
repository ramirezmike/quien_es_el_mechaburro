use crate::{asset_loading, assets::GameAssets, burro, cleanup, game_state, AppState};
use bevy::prelude::*;

pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Debug).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_hearts));
    }
}

#[derive(Component)]
struct CleanupMarker;

fn update_hearts(
    game_state: Res<game_state::GameState>,
    burros: Query<&burro::Burro>,
    mut hearts: Query<&mut Visibility, With<UiImage>>,
) {
    game_state.burros.iter().for_each(|burro_state| {
        let burro = burros
            .iter()
            .filter(|b| b.burro_skin == burro_state.skin)
            .last();

        if let Some(burro) = burro {
            burro_state
                .hearts
                .iter()
                .enumerate()
                .for_each(|(i, entity)| {
                    if let Ok(mut heart_visibility) = hearts.get_mut(*entity) {
                        heart_visibility.is_visible = i < burro.health;
                    }
                });
        } else {
            // burro must be dead already
            burro_state
                .hearts
                .iter()
                .enumerate()
                .for_each(|(i, entity)| {
                    if let Ok(mut heart_visibility) = hearts.get_mut(*entity) {
                        heart_visibility.is_visible = false;
                    }
                });
        }
    });
}

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
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
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
                                burro.hearts = vec![];
                                burro.hearts.push(
                                    parent
                                        .spawn_bundle(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(20.0), Val::Auto),
                                                margin: Rect {
                                                    top: Val::Px(100.0),
                                                    left: Val::Px(padding),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            image: game_assets.heart_texture.image.clone().into(),
                                            ..Default::default()
                                        })
                                        .id(),
                                );

                                burro.hearts.push(
                                    parent
                                        .spawn_bundle(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(20.0), Val::Auto),
                                                margin: Rect {
                                                    top: Val::Px(100.0),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            image: game_assets.heart_texture.image.clone().into(),
                                            ..Default::default()
                                        })
                                        .id(),
                                );

                                burro.hearts.push(
                                    parent
                                        .spawn_bundle(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(20.0), Val::Auto),
                                                margin: Rect {
                                                    top: Val::Px(100.0),
                                                    right: Val::Px(padding),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            image: game_assets.heart_texture.image.clone().into(),
                                            ..Default::default()
                                        })
                                        .id(),
                                );
                            });
                        });
                });
        });
}
