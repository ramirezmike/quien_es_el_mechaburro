use crate::{
    assets::GameAssets,
    game_state::{BurroSkin, BurroState},
    player,
};
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn insert_avatars<'w, 's, 'a>(
    child_builder: &mut ChildBuilder<'w, 's, 'a>,
    burros: &Vec<&BurroState>,
    game_assets: &Res<GameAssets>,
) {
    let padding = Val::Px(20.0);
    child_builder
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
                        BurroSkin::Pinata => game_assets.pinata_logo_texture.image.clone().into(),
                        BurroSkin::Meow => game_assets.meow_logo_texture.image.clone().into(),
                        BurroSkin::Salud => game_assets.salud_logo_texture.image.clone().into(),
                        BurroSkin::Mexico => game_assets.mexico_logo_texture.image.clone().into(),
                        BurroSkin::Medianoche => {
                            game_assets.medianoche_logo_texture.image.clone().into()
                        }
                        BurroSkin::Morir => game_assets.morir_logo_texture.image.clone().into(),
                        BurroSkin::Gators => game_assets.gators_logo_texture.image.clone().into(),
                        BurroSkin::Aguas => game_assets.aguas_logo_texture.image.clone().into(),
                    },
                    ..Default::default()
                });
            });
        });
}

pub fn insert_player_indicators<'w, 's, 'a>(
    child_builder: &mut ChildBuilder<'w, 's, 'a>,
    burros: &Vec<&BurroState>,
    player_map: &HashMap<BurroSkin, usize>,
    game_assets: &Res<GameAssets>,
) {
    let padding = Val::Px(20.0);
    child_builder
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
            burros.iter().for_each(|burro| {
                let (text, color) = if burro.is_bot {
                    ("", Color::rgba(0.0, 0.0, 0.0, 0.0))
                } else {
                    player::get_player_indicator(player_map[&burro.skin])
                };

                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        margin: Rect {
                            left: padding,
                            right: padding,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        text,
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 30.0,
                            color,
                        },
                        TextAlignment {
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                });
            });
        });
}

pub fn insert_health_indicators<'w, 's, 'a>(
    child_builder: &mut ChildBuilder<'w, 's, 'a>,
    burros: &mut Vec<BurroState>,
    game_assets: &Res<GameAssets>,
) {
    let padding = 40.0;
    child_builder
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
            burros.iter_mut().for_each(|mut burro| {
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
}
