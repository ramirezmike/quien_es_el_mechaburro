use crate::{assets::GameAssets, burro, cleanup, game_state, hit, ui, ui::text_size, IngameState};
use bevy::prelude::*;
use std::collections::HashMap;

mod score_display;
mod winner_circle;

const UI_UPDATE: f32 = 0.5;
pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((score_display::ScoreDisplayPlugin, winner_circle::WinnerCirclePlugin)) 
            .add_systems(OnEnter(IngameState::InGame), setup)
            .insert_resource(FixedTime::new_from_secs(UI_UPDATE))
            .add_systems(
                FixedUpdate,
                (update_hearts,).run_if(in_state(IngameState::InGame)),
            )
            .add_systems(OnExit(IngameState::InGame), cleanup::<CleanupMarker>);
    }
}

#[derive(Component, Clone)]
struct CleanupMarker;
#[derive(Component, Clone)]
struct HeartImageMarker(usize);

fn update_hearts(
    burros: Query<(&burro::Burro, &game_state::PlayerMarker)>,
    mut hearts: Query<(
        &mut Visibility,
        &HeartImageMarker,
        &game_state::PlayerMarker,
    )>,
) {
    for (mut visibility, heart, heart_player) in &mut hearts {
        let mut has_burro = false;
        for (burro, burro_player) in &burros {
            if burro_player == heart_player {
                has_burro = true;
                *visibility = if heart.0 < burro.health {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }

        if !has_burro {
            *visibility = Visibility::Hidden;
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_state: Res<game_state::GameState>,
    mut images: ResMut<Assets<Image>>,
    window_size: Res<ui::text_size::WindowSize>,
) {
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
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .insert(CleanupMarker)
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(10.0),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for burro in game_state.burros.iter() {
                        builder
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(10.0),
                                    height: Val::Percent(100.0),
                                    //  position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    //  align_items: AlignItems::FlexEnd,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|builder| {
                                builder.spawn(ImageBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        width: Val::Auto,
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    image: game_assets.avatar_top.image.clone().into(),
                                    z_index: ZIndex::Global(-10),
                                    ..default()
                                });
                                builder.spawn(ImageBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        width: Val::Auto,
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    image: game_assets.avatar_bottom.image.clone().into(),
                                    z_index: ZIndex::Global(5),
                                    ..default()
                                });
                                builder
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            position_type: PositionType::Relative,
                                            justify_content: JustifyContent::SpaceAround,
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|builder| {
                                        builder.spawn((
                                            ImageBundle {
                                                style: Style {
                                                    position_type: PositionType::Relative,
                                                    width: Val::Auto,
                                                    height: Val::Percent(75.0),
                                                    ..default()
                                                },
                                                image: burro_image_handles[&burro.selected_burro]
                                                    .clone()
                                                    .into(),
                                                z_index: ZIndex::Global(4),
                                                ..default()
                                            },
                                            game_state::PlayerMarker(burro.player),
                                        ));
                                        builder
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(100.0),
                                                    height: Val::Auto,
                                                    display: Display::Flex,
                                                    position_type: PositionType::Relative,
                                                    align_items: AlignItems::Start,
                                                    margin: UiRect {
                                                        bottom: Val::Percent(3.5),
                                                        ..default()
                                                    },
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|builder| {
                                                for i in 0..3 {
                                                    builder.spawn((
                                                        ImageBundle {
                                                            style: Style {
                                                                width: Val::Auto,
                                                                height: Val::Percent(100.0),
                                                                ..default()
                                                            },
                                                            image: game_assets
                                                                .heart_texture
                                                                .image
                                                                .clone()
                                                                .into(),
                                                            z_index: ZIndex::Global(10),
                                                            ..default()
                                                        },
                                                        HeartImageMarker(i),
                                                        game_state::PlayerMarker(burro.player),
                                                    ));
                                                }
                                            });
                                    });
                            });
                    }
                });
        });
}
