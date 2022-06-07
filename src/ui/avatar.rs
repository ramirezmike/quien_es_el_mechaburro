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
    player_map: &HashMap<BurroSkin, usize>,
    indicator_font_size: f32,
) {
    let number_of_burros = burros.len() as f32;
    let width = (1.0 / number_of_burros) * 100.0;
    let scale = number_of_burros / 8.0;
    burros.iter().for_each(|burro| {
        child_builder
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(width), Val::Percent(100.0)),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0 * scale), Val::Auto),
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

                if !burro.is_bot {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(100.0 * scale),
                                    Val::Percent(100.0 * scale),
                                ),
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            let (text, color) =
                                player::get_player_indicator(player_map[&burro.skin]);
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect {
                                        top: Val::Percent(30.0),
                                        left: Val::Percent(25.0),
                                        right: Val::Auto,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    text,
                                    TextStyle {
                                        font: game_assets.font.clone(),
                                        font_size: indicator_font_size,
                                        color,
                                    },
                                    TextAlignment::default(),
                                ),
                                ..Default::default()
                            });
                        });
                }
            });
    });
}

pub fn insert_health_indicators<'w, 's, 'a>(
    child_builder: &mut ChildBuilder<'w, 's, 'a>,
    burros: &mut Vec<BurroState>,
    game_assets: &Res<GameAssets>,
) {
    let number_of_burros = burros.len() as f32;
    let width = (1.0 / number_of_burros) * 100.0;
    let scale = 20.0 * (number_of_burros / 8.0);

    burros.iter_mut().for_each(|burro| {
        child_builder
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(width), Val::Percent(100.0)),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                burro.hearts = vec![];
                burro.hearts.push(
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Percent(scale), Val::Auto),
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
                                size: Size::new(Val::Percent(scale), Val::Auto),
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
                                size: Size::new(Val::Percent(scale), Val::Auto),
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
